<div align="center">
  <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&weight=700&size=26&duration=2500&pause=800&color=36BCF7&center=true&vCenter=true&width=800&lines=weavetui_derive;Procedural+Macros+for+weavetui" alt="weavetui_derive banner" />
  <h1>weavetui_derive</h1>
  <p>Procedural macro crate • `#[component]` to eliminate boilerplate in Rust TUI components</p>
  <!-- Package Badges -->
  <a href="https://crates.io/crates/weavetui_derive"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/weavetui_derive?style=flat-square&logo=rust&color=orange"></a>
  <a href="https://docs.rs/weavetui_derive"><img alt="docs.rs" src="https://img.shields.io/docsrs/weavetui_derive?style=flat-square&logo=docs.rs&color=blue"></a>
  <a href="https://crates.io/crates/weavetui_derive"><img alt="Crates.io Downloads" src="https://img.shields.io/crates/d/weavetui_derive?style=flat-square&logo=rust&color=orange"></a>

  <!-- Framework Badges -->
  <img alt="Framework Component" src="https://img.shields.io/badge/weavetui-derive-brightgreen?style=flat-square">
  <img alt="Procedural Macro" src="https://img.shields.io/badge/type-proc--macro-purple?style=flat-square&logo=rust">

  <!-- Quality and Language -->
  <a href="https://github.com/mzyui/weavetui/actions/workflows/rust.yml"><img alt="CI Status" src="https://img.shields.io/github/actions/workflow/status/mzyui/weavetui/rust.yml?style=flat-square&logo=github&label=CI"></a>
  <img alt="Rust Edition" src="https://img.shields.io/badge/rust%20edition-2024-red?style=flat-square&logo=rust">

  <!-- Legal -->
  <a href="../LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square"></a>
</div>

---

**weavetui_derive** provides the `#[component]` procedural macro that dramatically reduces boilerplate when creating weavetui components. This macro automatically implements the required traits, injects context fields, and optionally generates default implementations - making component development fast and ergonomic.

## 🚀 Features

### Automatic Trait Implementation
- **Auto-implements** `weavetui_core::Component` and `ComponentAccessor` traits
- **Context injection**: Automatically injects `pub _ctx: weavetui_core::ComponentContext`
- **Default generation**: Creates `Default` impl that properly initializes context and children

### Declarative Component Definition
- **`#[component(default)]`**: Generates default `draw()` method for rapid prototyping
- **`#[component(children(...))]`**: Declarative child component management
- **Zero boilerplate**: Focus on your component logic, not trait implementations

### Smart Field Management
- **Automatic field injection**: Adds required context fields if missing
- **Child initialization**: Automatically initializes declared children in `Default` impl
- **Debug support**: Maintains or adds `Debug` trait implementation

## 📦 Installation

```toml
[dependencies]
weavetui_derive = "0.1.2"
weavetui_core = "0.1.2"  # Required for traits and types
```

Or use the main crate which re-exports everything:

```toml
[dependencies]
weavetui = "0.1.2"
```

## 🔧 Usage

### Basic Component

The simplest component with default draw implementation:

```rust
use weavetui_derive::component;

#[component(default)]
pub struct SimpleComponent {
    pub message: String,
}

// That's it! The macro generates:
// - ComponentAccessor implementation
// - Component implementation with default draw()
// - Default implementation
// - Required _ctx field injection
```

### Custom Component with Draw

Override the `draw` method for custom rendering:

```rust
use weavetui_derive::component;
use weavetui_core::Component;
use ratatui::{Frame, layout::Rect, widgets::{Block, Paragraph}};

#[component]
pub struct CustomComponent {
    pub counter: i32,
    pub title: String,
}

impl Component for CustomComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let block = Block::bordered().title(&self.title);
        let text = format!("Counter: {}", self.counter);
        f.render_widget(Paragraph::new(text).block(block), area);
    }

    fn on_event(&mut self, message: &str) {
        match message {
            "increment" => self.counter += 1,
            "decrement" => self.counter -= 1,
            "reset" => self.counter = 0,
            _ => {}
        }
    }
}
```

### Component with Children

Define child components declaratively:

```rust
use weavetui_derive::component;

#[component(default)]
pub struct Header {
    pub title: String,
}

#[component(default)]
pub struct Footer {
    pub status: String,
}

#[component(children("header" => Header, "footer" => Footer))]
pub struct MainLayout {
    pub content: String,
}

impl Component for MainLayout {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let chunks = Layout::vertical([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Content
            Constraint::Length(3),  // Footer
        ]).split(area);

        // Access children easily
        if let Some(header) = self.child_mut("header") {
            if let Some(header_comp) = header.downcast_mut::<Header>() {
                header_comp.title = "Dynamic Title".to_string();
            }
            header.draw(f, chunks[0]);
        }

        // Render main content
        let content_block = Block::bordered().title("Content");
        f.render_widget(Paragraph::new(&self.content).block(content_block), chunks[1]);

        // Draw footer
        if let Some(footer) = self.child_mut("footer") {
            footer.draw(f, chunks[2]);
        }
    }
}
```

### Complex Component with Multiple Features

Combine all features for complex components:

```rust
use weavetui_derive::component;
use weavetui_core::{Component, event::Action};
use ratatui::{Frame, layout::Rect, widgets::*};

#[component(default)]
pub struct StatusBar;

#[component(default)]
pub struct Sidebar;

#[component(children("status" => StatusBar, "sidebar" => Sidebar))]
pub struct MainApp {
    pub current_view: String,
    pub user_name: String,
    pub notifications: Vec<String>,
}

impl Component for MainApp {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let main_chunks = Layout::horizontal([
            Constraint::Length(20), // Sidebar
            Constraint::Min(0),     // Main content
        ]).split(area);

        let right_chunks = Layout::vertical([
            Constraint::Min(0),     // Content
            Constraint::Length(3),  // Status bar
        ]).split(main_chunks[1]);

        // Draw sidebar
        if let Some(sidebar) = self.child_mut("sidebar") {
            sidebar.draw(f, main_chunks[0]);
        }

        // Draw main content with theme colors
        let primary_color = self.get_color("primary");
        let main_block = Block::bordered()
            .title(&self.current_view)
            .border_style(Style::default().fg(primary_color));

        f.render_widget(main_block, right_chunks[0]);

        // Draw status bar
        if let Some(status) = self.child_mut("status") {
            status.draw(f, right_chunks[1]);
        }
    }

    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) -> Option<Action> {
        use crossterm::event::{KeyCode, KeyModifiers};

        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('n')) => {
                self.notifications.push("New notification!".to_string());
                Some(Action::AppAction("refresh_ui".to_string()))
            }
            (KeyModifiers::ALT, KeyCode::Char('v')) => {
                self.current_view = "Alternative View".to_string();
                None
            }
            _ => None
        }
    }

    fn on_event(&mut self, message: &str) {
        match message {
            "switch_view" => self.current_view = "Switched View".to_string(),
            "clear_notifications" => self.notifications.clear(),
            _ => {}
        }
    }
}
```

## 🏗️ Macro Attributes

### `#[component]`
Basic component with manual `draw()` implementation required.

### `#[component(default)]`
Generates a default `draw()` method that displays component name and dimensions in a bordered box.

### `#[component(children("name" => Type, ...))]`
Declares child components that will be automatically initialized in the `Default` implementation.

```rust
#[component(children(
    "header" => HeaderComponent,
    "content" => ContentComponent,
    "footer" => FooterComponent
))]
pub struct Layout {
    pub theme: String,
}
```

### Combined Attributes
```rust
#[component(default, children("child1" => ChildType, "child2" => AnotherType))]
pub struct ComplexComponent;
```

## 🔍 Generated Code

The macro automatically generates:

### Injected Fields
If not present, these fields are automatically added:
```rust
pub _ctx: weavetui_core::ComponentContext
```

### ComponentAccessor Implementation
```rust
impl ComponentAccessor for YourComponent {
    fn name(&self) -> String { /* ... */ }
    fn area(&self) -> Option<Rect> { /* ... */ }
    fn set_area(&mut self, area: Rect) { /* ... */ }
    fn is_active(&self) -> bool { /* ... */ }
    fn set_active(&mut self, active: bool) { /* ... */ }
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) { /* ... */ }
    fn send(&self, action: &str) { /* ... */ }
    fn send_action(&self, action: Action) { /* ... */ }
    fn get_children(&mut self) -> &mut Children { /* ... */ }
    fn get_theme_manager(&self) -> &ThemeManager { /* ... */ }
    fn set_theme_manager(&mut self, tm: ThemeManager) { /* ... */ }
}
```

### Default Implementation
```rust
impl Default for YourComponent {
    fn default() -> Self {
        // Initializes _ctx and declared children
    }
}
```

## 💡 Best Practices

- **Use `weavetui::prelude::*`** to access all necessary types and re-exported macros
- **Prefer `#[component]`** over manual trait implementation for consistency
- **Use `#[component(default)]`** for rapid prototyping and placeholder components
- **Leverage children declarations** for complex nested component hierarchies
- **Access theme colors** via `self.get_color("color_name")` in your components

## 🔗 Related Crates

- **[weavetui](https://crates.io/crates/weavetui)**: Main framework with prelude (recommended)
- **[weavetui_core](https://crates.io/crates/weavetui_core)**: Core traits and runtime

## 🤝 Contributing

See the main project's [CONTRIBUTING.md](../CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md).

## 📄 License

Licensed under the MIT License. See [LICENSE](../LICENSE) for details.

---

<div align="center">
  <p><strong>Part of the weavetui ecosystem • Built with ❤️ by <a href="https://github.com/mzyui">Val</a></strong></p>
</div>