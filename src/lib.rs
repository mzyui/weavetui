//! Main library for the `weavetui` application.
//!
//! This crate re-exports key components from `weavetui_core` and `weavetui_derive`
//! to provide a convenient facade for building Text User Interface (TUI) applications.

/// Re-exports core traits and modules from the `weavetui_core` crate.
///
/// This includes:
/// *   `Component`: The core trait for UI components.
/// *   `ComponentAccessor`: A trait for accessing component properties.
/// *   `app`: The main application module.
/// *   `components`: A collection of pre-built UI components.
/// *   `event`: Modules for handling events and actions.
/// *   `kb`: Utilities for keybindings.
/// *   `keyboard`: Keyboard event handling utilities.
/// *   `tui`: Terminal UI management.
pub use weavetui_core::{app, event, kb, keyboard, tui, Component, ComponentAccessor};
pub use weavetui_derive::*;
