//! Core library for the `weavetui` TUI framework.
//!
//! This crate defines the fundamental traits and types for building interactive
//! Text User Interface (TUI) components, including `Component` for rendering
//! and event handling, and `ComponentAccessor` for managing component properties
//! and children.
//!
//! It provides the building blocks for the `weavetui` ecosystem, designed to be
//! used in conjunction with the `weavetui_derive` crate for declarative component
//! creation.

use downcast_rs::{impl_downcast, Downcast};
use std::collections::BTreeMap;
use std::fmt::Debug;

pub mod app;
pub mod component_manager;
pub mod event;
pub mod keyboard;
pub mod macros;
pub mod theme;
pub mod tui;

use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{layout::Rect, Frame};
use tokio::sync::mpsc::UnboundedSender;

use event::Action;

use crate::{event::Event, keyboard::KeyBindings};

/// A type alias for a `BTreeMap` that stores child components.
///
/// The keys are `String` representations of the child component names,
/// and the values are `Box<dyn Component>` trait objects, allowing for
/// polymorphic storage of different component types.
pub type Children = BTreeMap<String, Box<dyn Component>>;

/// A handler for managing the lifecycle of a component.
///
/// `ComponentHandler` wraps a `Box<dyn Component>` and provides methods
/// to initialize, handle events, update, and draw the component.
#[derive(Debug)]
pub struct ComponentHandler {
    c: Box<dyn Component>,
}

impl ComponentHandler {
    /// Creates a new `ComponentHandler` for the given component.
    pub fn for_(component: Box<dyn Component>) -> Self {
        Self { c: component }
    }

    /// Handles the initialization of the component.
    pub(crate) fn handle_init(&mut self, area: Rect) {
        component_manager::init(self.c.as_mut(), area);
    }

    /// Receives and sets up the action handler for the component.
    pub(crate) fn receive_action_handler(&mut self, tx: UnboundedSender<Action>) {
        component_manager::receive_action_handler(self.c.as_mut(), tx);
    }

    /// Handles input events and returns a list of resulting actions.
    /// Handles input events and returns a list of resulting actions.
    pub(crate) fn handle_events(&mut self, event: &Option<Event>) -> Vec<Action> {
        component_manager::handle_event_for(self.c.as_mut(), event)
    }

    /// Updates the component's state based on a received action.
    pub(crate) fn handle_update(&mut self, action: &Action) {
        component_manager::update(self.c.as_mut(), action);
    }

    /// Handles a string message received by the component.
    pub(crate) fn handle_message(&mut self, message: &str) {
        component_manager::handle_message(self.c.as_mut(), message);
    }

    /// Handles the drawing process of the component to the frame.
    pub(crate) fn handle_draw(&mut self, f: &mut Frame<'_>) {
        component_manager::handle_draw(self.c.as_mut(), f);
    }

    /// Handles custom keybindings for the component.
    /// This function is called by the application to allow components to register their own keybindings.
    pub(crate) fn handle_custom_keybindings(&mut self, kb: &mut KeyBindings) {
        component_manager::custom_keybindings(self.c.as_mut(), kb);
    }
}

/// A trait that provides access to the basic properties of a component.
///
/// This trait is used to manage a component's name, area, active state, and children.
pub trait ComponentAccessor: Debug {
    /// Returns the name of the component.
    fn name(&self) -> String;

    /// Returns the area (`Rect`) of the component, if it has been set.
    fn area(&self) -> Option<Rect>;

    /// Sets the area (`Rect`) for the component.
    fn set_area(&mut self, area: Rect);

    /// Returns the active state of the component.
    fn is_active(&self) -> bool;

    /// Sets the active state of the component.
    fn set_active(&mut self, active: bool);

    /// Registers an action handler that can send `Action`s for processing.
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>);

    /// Sends a string message through the action handler bus.
    fn send(&self, action: &str);

    /// Sends an `Action` through the action handler bus.
    fn send_action(&self, action: Action);

    /// Sets the component as active on initialization (builder-pattern).
    #[allow(clippy::wrong_self_convention)] // This is a builder-pattern method
    fn as_active(self) -> Self
    where
        Self: Sized;

    /// Gets all child components. This is necessary if the component has children,
    /// as it will be used by other functions to have knowledge of the children.
    fn get_children(&mut self) -> &mut Children;
}

impl_downcast!(Component);

/// The main trait for all UI components in the `weavetui` framework.
///
/// This trait defines the core functionality of a component, including event handling
/// (`handle_key_events`, `handle_mouse_events`, etc.) and rendering (`draw`).
///
/// Implementors must also implement `ComponentAccessor` and `Downcast`.
pub trait Component: ComponentAccessor + Downcast {
    /// Initialize the component with a specified area if necessary. Usefull for components that
    /// need to performe some initialization before the first render.
    ///
    /// # Arguments
    ///
    /// * `area` - Rectangular area where the component will be rendered the first time.
    #[allow(unused)]
    fn init(&mut self, area: Rect) {}

    /// Render the component on the screen. (REQUIRED)
    ///
    /// # Arguments
    ///
    /// * `f` - A frame used for rendering.
    /// * `area` - The area in which the component should be drawn.
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect);

    /// Returns the `KeyBindings` for this component.
    ///
    /// `KeyBindings` are used to display keybinding hints to the user.
    fn keybindings(&self) -> KeyBindings {
        KeyBindings::default()
    }

    /// Handle key events and produce actions if necessary.
    ///
    /// # Arguments
    ///
    /// * `key` - A key event to be processed.
    ///
    /// # Returns
    ///
    /// * `Option<Action>` - An action to be processed or none.
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
    /// * `Option<Action>` - An action to be processed or none.
    #[allow(unused_variables)]
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Option<Action> {
        None
    }

    /// Handle Tick events and produce actions if necessary.
    ///
    /// # Returns
    ///
    /// * `Option<Action>` - An action to be processed or none.
    #[allow(unused_variables)]
    fn handle_tick_event(&mut self) -> Option<Action> {
        None
    }

    /// Handle frame events and produce actions if necessary.
    ///
    /// # Returns
    ///
    /// * `Option<Action>` - An action to be processed or none.
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
    /// * `Option<Action>` - An action to be processed or none.
    #[allow(unused_variables)]
    fn handle_paste_event(&mut self, message: &str) -> Option<Action> {
        None
    }

    /// Update the state of the component based on a received action.
    ///
    /// # Arguments
    ///
    /// * `action` - An action that may modify the state of the component.
    #[allow(unused_variables)]
    fn update(&mut self, action: &Action) {}

    /// Receive a custom event, probably from another component.
    /// # Arguments
    ///
    /// * `message` - A string message to be processed.
    #[allow(unused_variables)]
    fn on_event(&mut self, message: &str) {}

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
    #[allow(clippy::borrowed_box)] // Intentional to allow direct downcasting of the Box
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
