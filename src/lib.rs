//! Main library for the `weavetui` application.
//! 
//! This crate re-exports key components from `weavetui_core` and `weavetui_derive`
//! to provide a convenient facade for building Text User Interface (TUI) applications.

pub use weavetui_core::{
    Component,
    ComponentAccessor,
    event::{Action, Event},
    tui::Tui,
    ComponentHandler,
};

pub use weavetui_derive::component;

// You might also want to re-export other modules or types as needed
// For example, if there are common utility functions within the weavetui application itself.
