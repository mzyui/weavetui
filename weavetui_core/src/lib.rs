use downcast_rs::{impl_downcast, Downcast};
use std::collections::BTreeMap;
use std::fmt::Debug;

pub mod app;
pub mod event;
pub mod keyboard;
pub mod macros;
pub mod tui;

use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{
    layout::{Rect, Size},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use event::Action;

use crate::event::Event;

pub type Children = BTreeMap<String, Box<dyn Component>>;

pub(crate) struct ComponentHandler {
    c: Box<dyn Component>,
}

impl ComponentHandler {
    pub fn for_(component: Box<dyn Component>) -> Self {
        Self { c: component }
    }

    pub(crate) fn handle_init(&mut self, area: Size) {
        init(self.c.as_mut(), area);
    }

    pub(crate) fn receive_action_handler(&mut self, tx: UnboundedSender<String>) {
        receive_action_handler(self.c.as_mut(), tx);
    }

    pub(crate) fn handle_events(&mut self, event: Option<Event>) -> Vec<Action> {
        handle_event_for(event, self.c.as_mut())
    }

    pub(crate) fn handle_update(&mut self, action: Action) {
        update(self.c.as_mut(), &action);
    }

    pub(crate) fn handle_message(&mut self, message: String) {
        handle_message(self.c.as_mut(), message);
    }

    pub(crate) fn handle_draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        if self.c.is_active() {
            self.c.draw(f, area);
        }
    }
}

/// Update the component and its childrend recursively, based on a received action.
fn update<T: Component + ?Sized>(c: &mut T, action: &Action) {
    if c.is_active() {
        c.update(action);

        for child in c.get_children().values_mut() {
            update(child.as_mut(), action);
        }
    }
}

/// Handle a message for a specific component and its children, recursively.
fn handle_message<T: Component + ?Sized>(c: &mut T, message: String) {
    if c.is_active() {
        c.receive_message(message.clone());

        for child in c.get_children().values_mut() {
            handle_message(child.as_mut(), message.clone());
        }
    }
}

/// Initialize a component and its children recursively.
fn init<T: Component + ?Sized>(c: &mut T, area: Size) {
    c.init(area);

    for child in c.get_children().values_mut() {
        init(child.as_mut(), area);
    }
}

/// Set the action handler for a component and its children recursively.
fn receive_action_handler<T: Component + ?Sized>(c: &mut T, tx: UnboundedSender<String>) {
    c.register_action_handler(tx.clone());

    for child in c.get_children().values_mut() {
        receive_action_handler(child.as_mut(), tx.clone());
    }
}

/// handle event for a specific component and its children, recursively.
fn handle_event_for<T: Component + ?Sized>(event: Option<Event>, c: &mut T) -> Vec<Action> {
    if c.is_active() {
        let mut actions = vec![];

        let action = match event {
            Some(Event::Key(key_event)) => c.handle_key_events(key_event),
            Some(Event::Mouse(mouse_event)) => c.handle_mouse_events(mouse_event),
            Some(Event::Tick) => c.handle_tick_event(),
            Some(Event::Render) => c.handle_frame_event(),
            Some(Event::Paste(ref event)) => c.handle_paste_event(event.clone()),
            _ => None,
        };

        if let Some(action) = action {
            actions.push(action);
        }

        for child in c.get_children().values_mut() {
            let child_actions = handle_event_for(event.clone(), child.as_mut());
            actions.extend(child_actions);
        }

        actions
    } else {
        vec![]
    }
}

pub trait ComponentAccessor: Debug {
    /// returns the name of the component
    fn name(&self) -> String;

    /// returns the active state of the component
    fn is_active(&self) -> bool;

    /// sets the active state of the component
    fn set_active(&mut self, active: bool);

    /// registers an action handler that can send actions for processing if necessary
    fn register_action_handler(&mut self, tx: UnboundedSender<String>);

    /// send a message to through the action handler bus
    fn send(&self, action: &str);

    /// send a message to through the action handler bus
    fn send_action(&self, action: Action);

    #[allow(clippy::wrong_self_convention)]
    fn as_active(self) -> Self
    where
        Self: Sized;

    /// Get all child components. This is necessary if the component has children, as will be
    /// used by other functions to have knowledge of the children.
    fn get_children(&mut self) -> &mut Children;
}

impl_downcast!(Component);

pub trait Component: ComponentAccessor + Downcast {
    /// Initialize the component with a specified area if necessary. Usefull for components that
    /// need to performe some initialization before the first render.
    ///
    /// # Arguments
    ///
    /// * `area` - Rectangular area where the component will be rendered the first time.
    #[allow(unused)]
    fn init(&mut self, area: Size) {}

    /// Handle key events and produce actions if necessary.
    ///
    /// # Arguments
    ///
    /// * `key` - A key event to be processed.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Action>>` - An action to be processed or none.
    #[allow(unused_variables)]
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        None
    }

    /// Handle mouse events and produce actions if necessary.
    ///
    /// # Arguments
    ///
    /// * `mouse` - A mouse event to be processed.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Action>>` - An action to be processed or none.
    #[allow(unused_variables)]
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Option<Action> {
        None
    }

    /// Handle Tick events and produce actions if necessary.
    ///
    /// # Arguments
    ///
    /// * `tick` - A tick event to be processed.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Action>>` - An action to be processed or none.
    #[allow(unused_variables)]
    fn handle_tick_event(&mut self) -> Option<Action> {
        None
    }

    /// Handle frame events and produce actions if necessary.
    ///
    /// # Arguments
    ///
    /// * `tick` - A tick event to be processed.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Action>>` - An action to be processed or none.
    #[allow(unused_variables)]
    fn handle_frame_event(&mut self) -> Option<Action> {
        None
    }

    /// Handle paste events and produce actions if necessary.
    ///
    /// # Arguments
    ///
    /// * `message` - A string message to be processed.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Action>>` - An action to be processed or none.
    #[allow(unused_variables)]
    fn handle_paste_event(&mut self, message: String) -> Option<Action> {
        None
    }

    /// Update the state of the component based on a received action.
    ///
    /// # Arguments
    ///
    /// * `action` - An action that may modify the state of the component.
    #[allow(unused_variables)]
    fn update(&mut self, action: &Action) {}

    /// Receive a custom message, probably from another component.
    /// # Arguments
    ///
    /// * `message` - A string message to be processed.
    #[allow(unused_variables)]
    fn receive_message(&mut self, message: String) {}

    /// Render the component on the screen. (REQUIRED)
    ///
    /// # Arguments
    ///
    /// * `f` - A frame used for rendering.
    /// * `area` - The area in which the component should be drawn.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - An Ok result or an error.
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect);

    /// Get a child component by name as a mutable reference.
    ///
    /// The method will return the child as a mutable reference to a `Box<dyn Component>`, which
    /// means that the caller will have to downcast it to the desired type if necessary.
    ///
    /// ```ignore
    /// let child = self.child_mut("child_name").unwrap();
    ///
    /// if let Some(downcasted_child) = child.downcast_mut::<MyComponent>() {
    ///    // do something with the downcasted child    
    /// }
    /// ```
    ///
    /// ... or just use the [child_downcast_mut] utility function
    ///
    /// # Arguments
    /// * `name` - The name of the child component.
    ///
    /// # Returns
    /// * `Option<&mut Box<dyn Component>>` - A mutable reference to the child component or none.
    fn child_mut(&mut self, name: &str) -> Option<&mut Box<dyn Component>> {
        self.get_children().get_mut(name)
    }

    /// Get a child component by name as an immutable reference.
    ///
    /// The method will return the child as a reference to a `Box<dyn Component>`, which means that
    /// the caller will have to downcast it to the desired type if necessary.
    ///
    /// ```ignore
    /// let child = self.child("child_name").unwrap();
    ///
    /// if let Some(downcasted_child) = child.downcast_ref::<MyComponent>() {
    ///     // do something with the downcasted child
    /// }
    /// ```
    ///
    /// ... or just use the [child_downcast] utility functions.
    ///
    /// # Arguments
    /// * `name` - The name of the child component.
    ///
    /// # Returns
    /// * `Option<&Box<dyn Component>>` - A reference to the child component or none.
    #[allow(clippy::borrowed_box)]
    fn child(&mut self, name: &str) -> Option<&Box<dyn Component>> {
        self.get_children().get(name)
    }

    /// Notify the component that its active state has changed.
    ///
    /// Whenever the active state of a component changes, the component will be notified through
    /// this method. This is useful for components that need to perform some action when they are
    /// activated or deactivated.
    ///
    /// # Arguments
    /// * `active` - The new active state of the component.
    #[allow(unused_variables)]
    fn on_active_changed(&mut self, active: bool) {}
}
