# weavetui_core

![Crates.io](https://img.shields.io/crates/v/weavetui_core) ![Docs.rs](https://docs.rs/weavetui_core/badge.svg)

`weavetui_core` is the foundational library for the `weavetui` Text User Interface (TUI) framework. It provides the essential building blocks, traits, and utilities that enable the creation of robust and interactive terminal applications.

## ✨ Features

*   **Component Traits:** Defines the `Component` and `ComponentAccessor` traits, which are the contracts for all UI components in `weavetui`. These traits promote a modular and reusable approach to TUI development.
*   **Event Handling:** Offers a standardized mechanism for processing keyboard, mouse, tick, and paste events, ensuring smooth user interactions.
*   **TUI Primitives:** Provides core types and functions for low-level terminal interaction, including managing terminal state, drawing elements, and handling raw input/output.
*   **Action Dispatch System:** Facilitates inter-component communication through a clear and efficient action dispatching system.

## 🚀 Getting Started

This crate is primarily designed to be a dependency for `weavetui` applications and other `weavetui`-related crates. To use it in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
weavetui_core = "0.1.1" # Replace with the latest version or a path/git dependency for development
```

## 📚 Usage and Examples

`weavetui_core` provides the fundamental traits that define how components behave and interact. While you typically implement these traits using the `weavetui_derive` macro, understanding them is crucial for advanced customization.

Here's a simplified example demonstrating the core `Component` and `ComponentAccessor` traits:

```rust
use weavetui_core::{
    Component,
    ComponentAccessor,
    event::Action,
};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Paragraph, Block, Borders},
};
use std::collections::BTreeMap;
use tokio::sync::mpsc::UnboundedSender;

// A simple component implementing the core traits
#[derive(Debug, Default)]
pub struct MyCustomComponent {
    // Components often hold their own state
    message: String,
    // And sometimes a sender for dispatching actions
    action_tx: Option<UnboundedSender<Action>>,
}

impl ComponentAccessor for MyCustomComponent {
    fn name(&self) -> String { "MyCustomComponent".to_string() }
    fn is_active(&self) -> bool { true }
    fn set_active(&mut self, _active: bool) { /* Logic to activate/deactivate component */ }
    fn area(&self) -> Option<Rect> { None }
    fn set_area(&mut self, _area: Rect) { /* Logic to set component's drawing area */ }
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) {
        self.action_tx = Some(tx);
    }
    fn send(&self, action_str: &str) {
        if let Some(tx) = &self.action_tx {
            let _ = tx.send(Action::from(action_str.to_string()));
        }
    }
    fn send_action(&self, action: Action) {
        if let Some(tx) = &self.action_tx {
            let _ = tx.send(action);
        }
    }
    fn as_active(self) -> Self { self } // Placeholder, actual logic might involve state changes
    fn get_children(&mut self) -> &mut BTreeMap<String, Box<dyn Component>> {
        // For a component without explicit children, return a mutable reference to a static empty map.
        // In real applications, this would return a map of actual child components.
        static mut EMPTY_CHILDREN: BTreeMap<String, Box<dyn Component>> = BTreeMap::new();
        unsafe { &mut EMPTY_CHILDREN }
    }
}

impl Component for MyCustomComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("My Core Component");
        f.render_widget(Paragraph::new(self.message.clone()).block(block), area);
    }

    // Other Component trait methods like handle_key_events, handle_mouse_events would go here.
    // For brevity, they are omitted in this basic example.
}

// In a real application, you would instantiate and manage this component
// within a `weavetui` application's `ComponentManager`.
```

For more comprehensive and runnable examples, please refer to the `examples` directory in the main `weavetui` repository, which showcases how `weavetui_core` is utilized in a full application context.
```

## 🤝 Contributing

We welcome contributions to `weavetui_core`! Please refer to the main `weavetui` project's [CONTRIBUTING.md](../../CONTRIBUTING.md) for detailed guidelines on how to get involved, report issues, and submit pull requests.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](../../LICENSE) file for details.