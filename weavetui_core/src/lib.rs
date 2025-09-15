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
pub mod internal;
pub mod keyboard;
pub mod macros;
pub mod theme;
pub mod tui;

pub use internal::ComponentContext;

use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{layout::Rect, Frame};
use tokio::sync::mpsc::UnboundedSender;

use event::Action;

use crate::{event::Event, keyboard::KeyBindings, theme::ThemeManager};

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

    /// Collects and registers custom keybindings from the component and its children.
    /// This function is called by the application to allow components to register their own keybindings.
    pub(crate) fn handle_custom_keybindings(&mut self, kb: &mut KeyBindings) {
        component_manager::custom_keybindings(self.c.as_mut(), kb);
    }

    /// Handles the theme for the component and its children.
    pub(crate) fn handle_theme(&mut self, th: ThemeManager) {
        component_manager::handle_theme(self.c.as_mut(), &th);
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

    /// Sets the component as active.
    fn active(&mut self);

    /// Sets the component as inactive.
    fn deactive(&mut self);

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

    /// Gets the theme manager for the component.
    fn get_theme_manager(&self) -> &ThemeManager;

    /// Sets the theme manager for the component.
    fn set_theme_manager(&mut self, theme_manager: ThemeManager);
}

impl_downcast!(Component);

/// The main trait for all UI components in the `weavetui` framework.
///
/// This trait defines the core functionality of a component, including event handling
/// (`handle_key_events`, `handle_mouse_events`, etc.) and rendering (`draw`).
///
/// Implementors must also implement `ComponentAccessor` and `Downcast`.
pub trait Component: ComponentAccessor + Downcast {
    /// Initializes the component, optionally using the provided area.
    ///
    /// This method is called once before the first render, allowing the component to perform
    /// any necessary setup, such as initializing state or creating resources.
    /// The default implementation does nothing.
    ///
    /// # Arguments
    ///
    /// * `area` - The initial rectangular area assigned to the component.
    #[allow(unused)]
    fn init(&mut self, area: Rect) {}

    /// Renders the component within the given area of the frame.
    ///
    /// This method is called on each render cycle and is responsible for drawing the component's UI.
    ///
    /// # Arguments
    ///
    /// * `f` - The frame to render on.
    /// * `area` - The area in which the component should be drawn.
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect);

    /// Returns the keybindings for this component.
    ///
    /// These keybindings can be used to display help to the user or for other introspective purposes.
    /// The default implementation returns an empty set of keybindings.
    fn keybindings(&self) -> KeyBindings {
        KeyBindings::default()
    }

    /// Handles key press events.
    ///
    /// This method is called when a key event is received and the component is active.
    /// It can be used to trigger actions based on user input.
    /// The default implementation does nothing.
    ///
    /// # Arguments
    ///
    /// * `key` - The `KeyEvent` to be processed.
    ///
    /// # Returns
    ///
    /// An `Option<Action>` which is `Some` if the event triggered an action, and `None` otherwise.
    #[allow(unused_variables)]
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        None
    }

    /// Handles mouse events.
    ///
    /// This method is called when a mouse event is received and the component is active.
    /// It can be used to handle clicks, scrolls, and other mouse interactions.
    /// The default implementation does nothing.
    ///
    /// # Arguments
    ///
    /// * `mouse` - The `MouseEvent` to be processed.
    ///
    /// # Returns
    ///
    /// An `Option<Action>` which is `Some` if the event triggered an action, and `None` otherwise.
    #[allow(unused_variables)]
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Option<Action> {
        None
    }

    /// Handles tick events.
    ///
    /// This method is called on each application tick, allowing for periodic updates or animations.
    /// The default implementation does nothing.
    ///
    /// # Returns
    ///
    /// An `Option<Action>` which is `Some` if the tick triggered an action, and `None` otherwise.
    #[allow(unused_variables)]
    fn handle_tick_event(&mut self) -> Option<Action> {
        None
    }

    /// Handles frame events.
    ///
    /// This method is called on each render frame, allowing for frame-based animations or updates.
    /// The default implementation does nothing.
    ///
    /// # Returns
    ///
    /// An `Option<Action>` which is `Some` if the frame event triggered an action, and `None` otherwise.
    #[allow(unused_variables)]
    fn handle_frame_event(&mut self) -> Option<Action> {
        None
    }

    /// Handles paste events.
    ///
    /// This method is called when text is pasted into the terminal.
    /// The default implementation does nothing.
    ///
    /// # Arguments
    ///
    /// * `message` - The pasted string.
    ///
    /// # Returns
    ///
    /// An `Option<Action>` which is `Some` if the event triggered an action, and `None` otherwise.
    #[allow(unused_variables)]
    fn handle_paste_event(&mut self, message: &str) -> Option<Action> {
        None
    }

    /// Updates the component's state based on a received action.
    ///
    /// This method is called for every action that is dispatched in the application,
    /// allowing the component to react to changes in the application state.
    /// The default implementation does nothing.
    ///
    /// # Arguments
    ///
    /// * `action` - The `Action` to be processed.
    #[allow(unused_variables)]
    fn update(&mut self, action: &Action) {}

    /// Handles custom string-based events.
    ///
    /// This method allows components to communicate with each other using simple string messages.
    /// The default implementation does nothing.
    ///
    /// # Arguments
    ///
    /// * `message` - The string message to be processed.
    #[allow(unused_variables)]
    fn on_event(&mut self, message: &str) {}

    /// Gets a mutable reference to a child component by name.
    ///
    /// This allows for modifying the state of a child component.
    /// The returned value is a `Box<dyn Component>`, which may need to be downcasted
    /// to the specific child component type.
    ///
    /// # Example
    ///
    /// ```ignore
    /// if let Some(child) = self.child_mut("child_name") {
    ///     if let Some(downcasted_child) = child.downcast_mut::<MyComponent>() {
    ///         // do something with the downcasted child
    ///     }
    /// }
    /// ```
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the child component.
    ///
    /// # Returns
    ///
    /// An `Option` containing a mutable reference to the child component, or `None` if not found.
    fn child_mut(&mut self, name: &str) -> Option<&mut Box<dyn Component>> {
        self.get_children().get_mut(name)
    }

    /// Gets an immutable reference to a child component by name.
    ///
    /// This allows for accessing the state of a child component.
    /// The returned value is a `Box<dyn Component>`, which may need to be downcasted
    /// to the specific child component type.
    ///
    /// # Example
    ///
    /// ```ignore
    /// if let Some(child) = self.child("child_name") {
    ///     if let Some(downcasted_child) = child.downcast_ref::<MyComponent>() {
    ///         // do something with the downcasted child
    ///     }
    /// }
    /// ```
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the child component.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the child component, or `None` if not found.
    #[allow(clippy::borrowed_box)] // Intentional to allow direct downcasting of the Box
    fn child(&mut self, name: &str) -> Option<&Box<dyn Component>> {
        self.get_children().get(name)
    }

    /// Called when the component's active state changes.
    ///
    /// This method is a hook that allows the component to react to being activated or deactivated.
    /// The default implementation does nothing.
    ///
    /// # Arguments
    ///
    /// * `active` - The new active state.
    #[allow(unused_variables)]
    fn on_active_changed(&mut self, active: bool) {}
}
