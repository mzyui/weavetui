//! Main library for the `weavetui` application.
//!
//! This crate re-exports key components from `weavetui_core` and `weavetui_derive`
//! to provide a convenient facade for building Text User Interface (TUI) applications.

pub use weavetui_core::{Component, ComponentAccessor, app, components, event, kb, keyboard, tui};
pub use weavetui_derive::*;
