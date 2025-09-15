# weavetui_core

![Crates.io](https://img.shields.io/crates/v/weavetui_core) ![Docs.rs](https://docs.rs/weavetui_core/badge.svg)

`weavetui_core` is the foundational library for the `weavetui` Text User Interface (TUI) framework. It provides the essential building blocks, traits, and utilities that enable the creation of robust and interactive terminal applications.

## ✨ Features

*   **Component Traits (`Component`, `ComponentAccessor`):** Defines the fundamental interfaces for all UI components, enabling modular and reusable TUI development. `Component` handles rendering and event processing, while `ComponentAccessor` manages component properties like name, area, active state, and children.
*   **Event Handling System:** Provides a robust and standardized mechanism for processing various input events (keyboard, mouse, paste) and internal events (tick, render), ensuring smooth and responsive user interactions. It leverages `Event` and `Action` enums for clear communication.
*   **TUI Primitives & Utilities:** Offers core types and functions for low-level terminal interaction, including managing terminal state, drawing elements with `ratatui`, and handling raw input/output through the `Tui` utility.
*   **Action Dispatch & Management:** Facilitates inter-component communication and application-wide state changes through a clear `Action` dispatching system, managed by the `App` and `ComponentManager`.
*   **Keybinding System:** Integrates a flexible keybinding system (`KeyBindings` and `kb!` macro) to easily map complex key sequences to specific actions.
*   **Theming Support:** Includes `Theme` and `ThemeManager` for defining and applying consistent visual styles (colors, styles) across components.

## 🚀 Getting Started

This crate is primarily designed to be a dependency for `weavetui` applications and other `weavetui`-related crates, especially when defining custom components or extending core functionalities. To use it in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
weavetui_core = "0.1.1" # Replace with the latest version or a path/git dependency for development
```

## 📚 Usage and Examples

`weavetui_core` provides the fundamental traits and utilities that define how components behave and interact within the `weavetui` framework. While you typically implement these traits using the `weavetui_derive` macro, understanding them is crucial for advanced customization and direct interaction with the core functionalities.

Key components and concepts include:
*   **`Component` and `ComponentAccessor` traits:** The contracts for all UI elements.
*   **`Event` and `Action` enums:** Define the types of input and commands processed by the application.
*   **`KeyBindings` and `kb!` macro:** For declarative and flexible key event mapping.
*   **`Theme` and `ThemeManager`:** For managing application-wide styling.
*   **`Tui`:** The low-level terminal interface manager.

Here's a simplified example demonstrating the core `Component` and `ComponentAccessor` traits, along with basic event handling and drawing:

```rust
use weavetui_core::{
    Component,
    ComponentAccessor,
    event::{Action, Event},
    keyboard::KeyEvent,
};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Paragraph, Block, Borders},
    style::{Color, Style},
};
use std::collections::BTreeMap;
use tokio::sync::mpsc::UnboundedSender;

// A simple component implementing the core traits
#[derive(Debug, Default)]
pub struct MyCustomComponent {
    message: String,
    action_tx: Option<UnboundedSender<Action>>,
    // In a real component, you'd have a ComponentContext here
    // _ctx: weavetui_core::ComponentContext,
}

impl ComponentAccessor for MyCustomComponent {
    fn name(&self) -> String { "MyCustomComponent".to_string() }
    fn is_active(&self) -> bool { true }
    fn set_active(&mut self, _active: bool) { /* ... */ }
    fn area(&self) -> Option<Rect> { None }
    fn set_area(&mut self, _area: Rect) { /* ... */ }
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) {
        self.action_tx = Some(tx);
    }
    fn send(&self, action_str: &str) {
        if let Some(tx) = &self.action_tx {
            let _ = tx.send(Action::AppAction(action_str.to_string()));
        }
    }
    fn send_action(&self, action: Action) {
        if let Some(tx) = &self.action_tx {
            let _ = tx.send(action);
        }
    }
    fn get_children(&mut self) -> &mut BTreeMap<String, Box<dyn Component>> {
        static mut EMPTY_CHILDREN: BTreeMap<String, Box<dyn Component>> = BTreeMap::new();
        unsafe { &mut EMPTY_CHILDREN }
    }
    fn get_theme_manager(&self) -> &weavetui_core::theme::ThemeManager {
        // Placeholder for example, in real component this would come from _ctx
        static DEFAULT_THEME_MANAGER: weavetui_core::theme::ThemeManager = weavetui_core::theme::ThemeManager::new();
        &DEFAULT_THEME_MANAGER
    }
    fn set_theme_manager(&mut self, _theme_manager: weavetui_core::theme::ThemeManager) { /* ... */ }
}

impl Component for MyCustomComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("My Core Component")
            .border_style(Style::default().fg(Color::Blue));
        f.render_widget(Paragraph::new(self.message.clone()).block(block), area);
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        match key.code {
            ratatui::prelude::KeyCode::Char('h') => {
                self.message = "Hello from key 'h'!".to_string();
                Some(Action::Render) // Request a re-render
            },
            ratatui::prelude::KeyCode::Char('q') => Some(Action::Quit),
            _ => None,
        }
    }

    fn on_event(&mut self, message: &str) {
        if message == "update_message" {
            self.message = "Message updated by external event!".to_string();
        }
    }
}

// In a real application, you would instantiate and manage this component
// within a `weavetui` application's `App` and `ComponentManager`.
```

For more comprehensive and runnable examples, please refer to the `examples` directory in the main `weavetui` repository, which showcases how `weavetui_core` is utilized in a full application context.

## 🤝 Contributing

We welcome contributions to `weavetui_core`! Please refer to the main `weavetui` project's [CONTRIBUTING.md](../../CONTRIBUTING.md) for detailed guidelines on how to get involved, report issues, and submit pull requests.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](../../LICENSE) file for details.
