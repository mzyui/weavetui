# weavetui_core

![Crates.io](https://img.shields.io/crates/v/weavetui_core) ![Docs.rs](https://docs.rs/weavetui_core/badge.svg)

`weavetui_core` is the foundational library for the `weavetui` Text User Interface (TUI) framework. It provides the essential building blocks, traits, and utilities that enable the creation of robust and interactive terminal applications.

## ✨ Features

*   **Component-Oriented Design:** Defines the `Component` and `ComponentAccessor` traits, promoting a modular and reusable approach to TUI development.
*   **Comprehensive Event Handling:** Offers a standardized mechanism for processing keyboard, mouse, tick, and paste events, ensuring smooth user interactions.
*   **Flexible TUI Primitives:** Includes core types and functions for managing terminal state, drawing elements, and handling input/output.
*   **Action Dispatch System:** Facilitates inter-component communication through a clear and efficient action dispatching system.

## 🚀 Getting Started

This crate is primarily designed to be a dependency for `weavetui` applications and other `weavetui`-related crates. To use it in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
weavetui_core = "0.1.0" # Replace with the latest version or a path/git dependency for development
```

## 📚 Examples

While `weavetui_core` provides the underlying traits, its usage is best demonstrated within a `weavetui` application. Here's a glimpse of how components interact:

```rust
use weavetui_core::{
    Component,
    ComponentAccessor,
    event::Action,
    tui::Tui,
};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Paragraph, Block, Borders},
};

// A simple component implementing the core traits
#[derive(Debug)]
pub struct MyCustomComponent {
    // ... fields
}

impl ComponentAccessor for MyCustomComponent {
    fn name(&self) -> String { "MyCustomComponent".to_string() }
    fn is_active(&self) -> bool { true }
    fn set_active(&mut self, _active: bool) { /* ... */ }
    fn register_action_handler(&mut self, _tx: tokio::sync::mpsc::UnboundedSender<String>) { /* ... */ }
    fn send(&self, _action: &str) { /* ... */ }
    fn send_action(&self, _action: Action) { /* ... */ }
    fn as_active(self) -> Self { self }
    fn get_children(&mut self) -> &mut std::collections::BTreeMap<String, Box<dyn Component>> { 
        // Return a mutable reference to an empty BTreeMap if no children
        static mut CHILDREN: Option<std::collections::BTreeMap<String, Box<dyn Component>>> = None;
        unsafe {
            CHILDREN.get_or_insert_with(std::collections::BTreeMap::new)
        }
    }
}

impl Component for MyCustomComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("My Component");
        f.render_widget(Paragraph::new("Hello from Core!").block(block), area);
    }

    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) -> Option<Action> {
        // Handle key events
        None
    }
}

// In your main application:
// let mut tui = Tui::new().unwrap();
// tui.enter().unwrap();
// let mut component = MyCustomComponent { /* ... */ };
// tui.draw(|f| component.draw(f, f.size())).unwrap();
// tui.exit().unwrap();
```

## 🤝 Contributing

We welcome contributions to `weavetui_core`! Please refer to the main `weavetui` project's [CONTRIBUTING.md](link-to-contributing-md) for detailed guidelines on how to get involved, report issues, and submit pull requests.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](link-to-license-file) file for details.