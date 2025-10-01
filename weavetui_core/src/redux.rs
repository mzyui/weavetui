//! Redux-style state management for weavetui applications.
//!
//! This module provides a Redux-like architecture with:
//! - Central store for application state
//! - Pure reducer functions for state transitions
//! - Action dispatching and state subscriptions
//! - Middleware support for cross-cutting concerns

use std::{fmt, sync::{Arc, RwLock}};
use tokio::sync::broadcast;
use crate::event::Action;

/// Trait for application state that can be managed by Redux store
pub trait AppState: Clone + Send + Sync + fmt::Debug + 'static {}

/// A pure function that takes current state and action, returns new state
pub type Reducer<S, A> = fn(&S, &A) -> S;

/// Middleware function type for intercepting actions
pub type Middleware<S, A> = Box<dyn for<'a, 'b, 'c> Fn(&'a S, &'b A, &'c Store<S, A>) -> bool + Send + Sync>;

/// Central Redux store for managing application state
#[derive(Clone)]
pub struct Store<S: AppState, A: Clone + Send + Sync + 'static> {
    inner: Arc<StoreInner<S, A>>,
}

impl<S: AppState, A: Clone + Send + Sync + 'static> fmt::Debug for Store<S, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Store")
            .field("state", &self.get_state())
            .finish()
    }
}

struct StoreInner<S: AppState, A: Clone + Send + Sync + 'static> {
    state: RwLock<S>,
    reducer: Reducer<S, A>,
    subscribers: RwLock<Vec<broadcast::Sender<S>>>,
    middleware: RwLock<Vec<Middleware<S, A>>>,
}

impl<S: AppState, A: Clone + Send + Sync + 'static> fmt::Debug for StoreInner<S, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StoreInner")
            .field("state", &"<RwLock>")
            .field("reducer", &"<fn>")
            .field("subscribers", &"<RwLock<Vec<Sender>>>")
            .field("middleware", &"<RwLock<Vec<Middleware>>>")
            .finish()
    }
}

impl<S: AppState, A: Clone + Send + Sync + 'static> Store<S, A> {
    /// Create a new Redux store with initial state and reducer
    pub fn new(initial_state: S, reducer: Reducer<S, A>) -> Self {
        Self {
            inner: Arc::new(StoreInner {
                state: RwLock::new(initial_state),
                reducer,
                subscribers: RwLock::new(Vec::new()),
                middleware: RwLock::new(Vec::new()),
            }),
        }
    }

    /// Get current state (immutable)
    pub fn get_state(&self) -> S {
        self.inner.state.read().unwrap().clone()
    }

    /// Dispatch an action to update state
    pub fn dispatch(&self, action: &A) {
        // Run middleware first
        let current_state = self.get_state();
        let middleware = self.inner.middleware.read().unwrap();

        for middleware_fn in middleware.iter() {
            if !middleware_fn(&current_state, action, self) {
                return; // Middleware blocked the action
            }
        }

        // Apply reducer to get new state
        let new_state = (self.inner.reducer)(&current_state, action);

        // Update state
        {
            let mut state = self.inner.state.write().unwrap();
            *state = new_state.clone();
        }

        // Notify subscribers
        self.notify_subscribers(new_state);
    }

    /// Subscribe to state changes
    pub fn subscribe(&self) -> broadcast::Receiver<S> {
        let (tx, rx) = broadcast::channel(100);

        {
            let mut subscribers = self.inner.subscribers.write().unwrap();
            subscribers.push(tx);
        }

        rx
    }

    /// Add middleware to the store
    pub fn add_middleware(&self, middleware: Middleware<S, A>) {
        let mut middleware_vec = self.inner.middleware.write().unwrap();
        middleware_vec.push(middleware);
    }

    fn notify_subscribers(&self, state: S) {
        let mut subscribers = self.inner.subscribers.write().unwrap();

        // Keep only active subscribers
        subscribers.retain(|tx| tx.receiver_count() > 0);

        // Send new state to all subscribers
        for tx in subscribers.iter() {
            let _ = tx.send(state.clone());
        }
    }
}

/// Trait for components that can connect to Redux store
pub trait ConnectedComponent<S: AppState, A: Clone + Send + Sync + 'static> {
    /// Called when state changes
    fn on_state_change(&mut self, state: &S);

    /// Get reference to store for dispatching actions
    fn get_store(&self) -> Option<&Store<S, A>>;
}

/// Helper struct to connect a component to Redux store
#[derive(Debug)]
pub struct StoreConnection<S: AppState, A: Clone + Send + Sync + 'static> {
    store: Store<S, A>,
    receiver: broadcast::Receiver<S>,
}

impl<S: AppState, A: Clone + Send + Sync + 'static> StoreConnection<S, A> {
    /// Create a new store connection
    pub fn new(store: Store<S, A>) -> Self {
        let receiver = store.subscribe();
        Self { store, receiver }
    }

    /// Get the store reference
    pub fn store(&self) -> &Store<S, A> {
        &self.store
    }

    /// Try to receive the latest state update (non-blocking)
    pub fn try_recv_state(&mut self) -> Option<S> {
        self.receiver.try_recv().ok()
    }

    /// Wait for next state update (blocking)
    pub async fn recv_state(&mut self) -> Option<S> {
        self.receiver.recv().await.ok()
    }
}

/// Macro to create a selector function for accessing specific parts of state
#[macro_export]
macro_rules! create_selector {
    ($state_type:ty, $field:ident, $return_type:ty) => {
        pub fn $field(state: &$state_type) -> $return_type {
            state.$field.clone()
        }
    };
}

/// Type alias for Action-based Redux store
pub type ActionStore<S> = Store<S, Action>;

/// Type alias for Action-based store connection
pub type ActionStoreConnection<S> = StoreConnection<S, Action>;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct TestState {
        counter: i32,
        message: String,
    }

    impl AppState for TestState {}

    #[derive(Clone, Debug)]
    enum TestAction {
        Increment,
        Decrement,
        SetMessage(String),
    }

    fn test_reducer(state: &TestState, action: &TestAction) -> TestState {
        match action {
            TestAction::Increment => TestState {
                counter: state.counter + 1,
                message: state.message.clone(),
            },
            TestAction::Decrement => TestState {
                counter: state.counter - 1,
                message: state.message.clone(),
            },
            TestAction::SetMessage(msg) => TestState {
                counter: state.counter,
                message: msg.clone(),
            },
        }
    }

    #[test]
    fn test_store_creation() {
        let initial_state = TestState {
            counter: 0,
            message: "Hello".to_string(),
        };

        let store = Store::new(initial_state.clone(), test_reducer);
        assert_eq!(store.get_state(), initial_state);
    }

    #[test]
    fn test_dispatch_action() {
        let initial_state = TestState {
            counter: 0,
            message: "Hello".to_string(),
        };

        let store = Store::new(initial_state, test_reducer);
        store.dispatch(&TestAction::Increment);

        assert_eq!(store.get_state().counter, 1);
    }
}