//! # weavetui
//!
//! A simple TUI framework for building elegant and responsive terminal applications.
//!
//! `weavetui` is built on top of `ratatui` and `tokio`, providing a component-based
//! architecture that simplifies the development of complex TUIs.
//!
//! ## Features
//!
//! *   **Component-Based:** Build your UI from reusable components.
//! *   **Declarative Macros:** Use the `#[component]` macro to reduce boilerplate.
//! *   **Async Event Handling:** Powered by `tokio` for non-blocking event processing.
//! *   **Keybinding System:** A simple and flexible keybinding system.
//!
//! ## Getting Started
//!
//! To get started, add `weavetui` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! weavetui = "0.1.0"
//! ```
//!
//! Then, create a simple application:
//!
//! ```rust,no_run
//! use weavetui::prelude::*;
//!
//! #[component(default)]
//! struct MyComponent;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let mut app = App::new([("<q>", "app:quit")], vec![Box::new(MyComponent::default())]);
//!     app.run().await?;
//!     Ok(())
//! }
//! ```

/// A prelude for `weavetui` applications.
///
/// This prelude re-exports the most commonly used traits and types from the `weavetui` ecosystem.
pub mod prelude {
    pub use weavetui_core::{
        app::App,
        event::{Action, Event},
        macros::*,
        keyboard::{key_event_to_string, KeyBindings},
        tui::Tui,
        Component,
        ComponentAccessor,
    };
    pub use weavetui_derive::component;
}

pub use weavetui_core::{app, event, kb, components, keyboard, tui, Component, ComponentAccessor};
pub use weavetui_derive::component;
