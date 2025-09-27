<div align="center">
  <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&weight=700&size=26&duration=2500&pause=800&color=36BCF7&center=true&vCenter=true&width=800&lines=Core+Library+for+Modern+Rust+TUI" alt="weavetui_core banner" />
  <h1>weavetui_core</h1>
  <p>Core library powering the weavetui framework • Traits, events, theming, and terminal I/O</p>
  <!-- Package Badges -->
  <a href="https://crates.io/crates/weavetui_core"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/weavetui_core?style=flat-square&logo=rust&color=orange"></a>
  <a href="https://docs.rs/weavetui_core"><img alt="docs.rs" src="https://img.shields.io/docsrs/weavetui_core?style=flat-square&logo=docs.rs&color=blue"></a>
  <a href="https://crates.io/crates/weavetui_core"><img alt="Crates.io Downloads" src="https://img.shields.io/crates/d/weavetui_core?style=flat-square&logo=rust&color=orange"></a>

  <!-- Framework Badges -->
  <img alt="Framework Component" src="https://img.shields.io/badge/weavetui-core-brightgreen?style=flat-square">
  <img alt="Rust Edition" src="https://img.shields.io/badge/rust%20edition-2024-red?style=flat-square&logo=rust">

  <!-- Quality and Dependencies -->
  <a href="https://github.com/mzyui/weavetui/actions/workflows/rust.yml"><img alt="CI Status" src="https://img.shields.io/github/actions/workflow/status/mzyui/weavetui/rust.yml?style=flat-square&logo=github&label=CI"></a>
  <img alt="Dependencies" src="https://img.shields.io/badge/deps-tokio%20%7C%20ratatui%20%7C%20crossterm-blue?style=flat-square">

  <!-- Legal -->
  <a href="../LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square"></a>
</div>

---

**weavetui_core** provides the essential traits, types, and utilities for building interactive terminal applications with a component-driven architecture. This crate forms the foundation of the weavetui framework, offering async event handling, comprehensive theming, and robust terminal I/O management.

## 🚀 Features

### Core Traits & Architecture
- **`Component`**: Main interface for UI components with lifecycle methods (`draw`, `init`, event handlers)
- **`ComponentAccessor`**: Component property management (name, area, active state, children)
- **`ComponentHandler`**: Lifecycle wrapper for component management
- **`ComponentContext`**: Injected context containing children, theme, and action handlers

### Event System
- **`Event` enum**: Terminal events (Key, Mouse, Tick, Frame, Paste, Resize, Focus)
- **`Action` enum**: Application commands (Quit, Render, AppAction, Key, custom actions)
- **Async processing**: Built on tokio for non-blocking event handling

### Input Management
- **`KeyBindings`**: HashMap-based keybinding system with multi-key support
- **`kb!` macro**: Declarative keybinding creation with flexible syntax
- **Key parsing**: String-to-KeyEvent conversion (`"<ctrl-c>"`, `"<alt-x><alt-y>"`)
- **Full modifier support**: ctrl, alt, shift combinations and special keys

### Theming System
- **`Theme`**: Color and style definitions with named lookup
- **`ThemeManager`**: Multi-theme management with runtime switching
- **Component integration**: Easy theme access via `get_color()` and `get_style()`

### Runtime & Terminal I/O
- **`App`**: Main application orchestrator managing event loop and components
- **`Tui`**: Terminal abstraction over `crossterm` and `ratatui`
- **Configurable rates**: Customizable tick rates and frame rates

## 📦 Installation

```toml
[dependencies]
weavetui_core = "0.1.2"
ratatui = "0.29.0"
tokio = { version = "1.47.1", features = ["sync", "rt-multi-thread"] }
```

## 🔧 Usage

### Basic Component Implementation

```rust
use weavetui_core::{Component, ComponentAccessor, event::Action, Children};
use ratatui::{Frame, layout::Rect, widgets::{Paragraph, Block}, style::Color};
use tokio::sync::mpsc::UnboundedSender;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct BasicComponent {
    message: String,
    children: Children,
    area: Option<Rect>,
    active: bool,
    action_tx: Option<UnboundedSender<Action>>,
    theme_manager: weavetui_core::theme::ThemeManager,
}

impl Default for BasicComponent {
    fn default() -> Self {
        Self {
            message: "Hello, weavetui_core!".to_string(),
            children: BTreeMap::new(),
            area: None,
            active: true,
            action_tx: None,
            theme_manager: weavetui_core::theme::ThemeManager::new(),
        }
    }
}

impl ComponentAccessor for BasicComponent {
    fn name(&self) -> String { "BasicComponent".to_string() }
    fn is_active(&self) -> bool { self.active }
    fn set_active(&mut self, active: bool) { self.active = active; }
    fn area(&self) -> Option<Rect> { self.area }
    fn set_area(&mut self, area: Rect) { self.area = Some(area); }

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

    fn get_children(&mut self) -> &mut Children { &mut self.children }
    fn get_theme_manager(&self) -> &weavetui_core::theme::ThemeManager { &self.theme_manager }
    fn set_theme_manager(&mut self, tm: weavetui_core::theme::ThemeManager) { self.theme_manager = tm; }
}

impl Component for BasicComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let primary_color = self.get_color("primary");
        let block = Block::bordered()
            .title("Basic Component")
            .border_style(ratatui::style::Style::default().fg(primary_color));

        f.render_widget(Paragraph::new(&self.message).block(block), area);
    }

    fn on_event(&mut self, message: &str) {
        match message {
            "update_message" => self.message = "Message updated!".to_string(),
            _ => {}
        }
    }
}
```

### Using with App Runtime

```rust
use weavetui_core::{app::App, components, kb, event::Action};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = App::default()
        .with_components(components![BasicComponent::default()])
        .with_keybindings(kb![
            "<ctrl-c>" => Action::Quit,
            "u" => "update_message"
        ]);

    app.run().await
}
```

### Advanced Features

#### Custom Theme Integration
```rust
use weavetui_core::theme::{Theme, ThemeManager};
use ratatui::style::{Color, Style};

let custom_theme = Theme::new("custom")
    .add_color("primary", Color::Cyan)
    .add_color("secondary", Color::Yellow)
    .add_style("title", Style::default().fg(Color::Cyan).bold())
    .add_style("border", Style::default().fg(Color::Gray));

let mut app = App::default()
    .add_theme(custom_theme)
    .with_components(components![BasicComponent::default()]);
```

#### Multi-Key Bindings
```rust
let advanced_bindings = kb![
    "<ctrl-c>" => Action::Quit,
    "<ctrl-x><ctrl-s>" => "save",          // Multi-key sequence
    "<alt-enter>" => "fullscreen",         // Modifier combinations
    "<f1>" => "help",                      // Function keys
    "<shift-tab>" => "previous_field"      // Shift modifiers
];
```

## 🏗️ Architecture

### Component Lifecycle
1. **Initialization**: `init()` called with initial area
2. **Event Handling**: Various event handlers process input
3. **State Updates**: `update()` processes actions
4. **Rendering**: `draw()` renders component to terminal
5. **Cleanup**: Automatic cleanup when component is dropped

### Event Flow
```
Terminal Input → Event → KeyBindings → Action → Component.update() → Component.draw() → Terminal Output
```

### Theme Inheritance
```
App Theme → Component Theme → Child Component Theme
```

## 💡 Best Practices

- **Use `weavetui_derive`** for automatic trait implementation instead of manual implementation
- **Always bind `Action::Quit`** for graceful application exit
- **Leverage theme system** for consistent styling across components
- **Handle errors gracefully** using the built-in error handling
- **Use async patterns** for non-blocking operations

## 🔗 Related Crates

- **[weavetui](https://crates.io/crates/weavetui)**: Main framework with prelude
- **[weavetui_derive](https://crates.io/crates/weavetui_derive)**: Procedural macros for components

## 🤝 Contributing

See the main project's [CONTRIBUTING.md](../CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md).

## 📄 License

Licensed under the MIT License. See [LICENSE](../LICENSE) for details.

---

<div align="center">
  <p><strong>Part of the weavetui ecosystem • Built with ❤️ by <a href="https://github.com/mzyui">Val</a></strong></p>
</div>
