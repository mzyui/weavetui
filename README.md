<div align="center">
  <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&weight=700&size=28&duration=2500&pause=800&color=36BCF7&center=true&vCenter=true&width=800&lines=Modern%2C+Robust+%26+Modular+Rust+TUI+Framework" alt="weavetui banner" />
  <h1>weavetui</h1>
  <p>Modern, Robust & Modular Rust TUI Framework • Built on ratatui + tokio</p>
  <!-- Core Package Badges -->
  <a href="https://crates.io/crates/weavetui"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/weavetui?style=flat-square&logo=rust&color=orange"></a>
  <a href="https://docs.rs/weavetui"><img alt="docs.rs" src="https://img.shields.io/docsrs/weavetui?style=flat-square&logo=docs.rs&color=blue"></a>
  <a href="https://crates.io/crates/weavetui"><img alt="Crates.io Downloads" src="https://img.shields.io/crates/d/weavetui?style=flat-square&logo=rust&color=orange"></a>

  <!-- Workspace Package Badges -->
  <a href="https://crates.io/crates/weavetui_core"><img alt="weavetui_core version" src="https://img.shields.io/crates/v/weavetui_core?style=flat-square&label=core&color=brightgreen"></a>
  <a href="https://crates.io/crates/weavetui_derive"><img alt="weavetui_derive version" src="https://img.shields.io/crates/v/weavetui_derive?style=flat-square&label=derive&color=brightgreen"></a>

  <!-- CI/CD and Quality Badges -->
  <a href="https://github.com/mzyui/weavetui/actions/workflows/rust.yml"><img alt="Build Status" src="https://img.shields.io/github/actions/workflow/status/mzyui/weavetui/rust.yml?style=flat-square&logo=github&label=CI"></a>
  <a href="https://github.com/mzyui/weavetui"><img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/mzyui/weavetui?style=flat-square&logo=github&color=lightgrey"></a>

  <!-- Language and Platform Badges -->
  <img alt="Rust Edition" src="https://img.shields.io/badge/rust%20edition-2024-red?style=flat-square&logo=rust">
  <img alt="Platform Support" src="https://img.shields.io/badge/platform-windows%20%7C%20macos%20%7C%20linux-lightgrey?style=flat-square">

  <!-- Community and Legal Badges -->
  <a href="LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square"></a>
  <a href="https://github.com/mzyui/weavetui/issues"><img alt="GitHub issues" src="https://img.shields.io/github/issues/mzyui/weavetui?style=flat-square&logo=github&color=red"></a>
  <a href="https://github.com/mzyui/weavetui/stargazers"><img alt="GitHub stars" src="https://img.shields.io/github/stars/mzyui/weavetui?style=flat-square&logo=github&color=yellow"></a>
</div>

---

A modern, robust, and modular Text User Interface (TUI) framework for Rust, built on top of `ratatui` and `tokio`. This workspace provides a component-based architecture with declarative macros, async event handling, and comprehensive theming support - making it easy to build sophisticated terminal applications with minimal boilerplate.

## Features

### 🧩 Component-Based Architecture
- **Modular Design**: Build UIs from reusable components using `Component` and `ComponentAccessor` traits
- **Child Management**: Hierarchical component structure with automatic child component handling
- **Component Lifecycle**: Full lifecycle management with init, draw, update, and event handling methods

### 🔧 Declarative Development
- **`#[component]` Macro**: Automatically inject context and implement required traits with minimal boilerplate
- **Automatic Field Injection**: Auto-generated `_ctx: ComponentContext` with children, area, active state, and action handlers
- **Default Implementations**: Optional default `draw()` method generation for rapid prototyping

### ⚡ Async Event System
- **Tokio-Powered**: Non-blocking event processing with full async/await support
- **Action-Based Communication**: Type-safe action dispatching between components and application
- **Multi-Event Support**: Handle keyboard, mouse, tick, frame, paste, and custom events

### ⌨️ Advanced Input Handling
- **Flexible Keybindings**: `KeyBindings` struct with intuitive `kb!` macro for single and multi-key combinations
- **Key Sequence Support**: Multi-key combinations like `<ctrl-x><ctrl-s>` with automatic parsing
- **Modifier Support**: Full support for `ctrl`, `alt`, `shift` modifiers and special keys

### 🎨 Comprehensive Theming
- **Theme Management**: `ThemeManager` for multiple themes with runtime switching
- **Component-Level Styling**: Easy access to colors and styles via `get_color()` and `get_style()` methods
- **Consistent Design**: Theme inheritance across component hierarchies

### 🔄 Robust Runtime
- **App Orchestration**: Central `App` struct manages component lifecycle and event distribution
- **Configurable Rates**: Customizable tick rates and frame rates for optimal performance
- **Error Handling**: Built-in error handling with `anyhow` integration
- **Terminal Management**: Abstracted terminal I/O with automatic setup and cleanup
- **Performance Metrics**: Optional monitoring (events/actions processed, FPS) via `App::with_performance_monitoring(true)`
- **Event/Action Batching**: Efficient processing with configurable batch sizes

## Architecture Overview

### Workspace Structure
```
weavetui/
├── weavetui/           # Main library crate
│   └── src/lib.rs      # Re-exports and prelude
├── weavetui_core/      # Core traits and runtime
│   ├── app.rs          # App struct and main event loop
│   ├── component_manager.rs # Component lifecycle management
│   ├── event.rs        # Event and Action definitions
│   ├── keyboard.rs     # Keybinding system and parsing
│   ├── theme.rs        # Theme and ThemeManager
│   ├── tui.rs          # Terminal I/O abstraction
│   └── lib.rs          # Core traits and types
├── weavetui_derive/    # Procedural macros
│   └── src/lib.rs      # #[component] macro implementation
└── examples/           # Example applications
    ├── counter_app.rs
    ├── on_off_redux.rs
    ├── on_off_redux_children.rs
    ├── on_off_children.rs
    └── store_in_children.rs
```

### Core Components

#### 🎯 **Component System**
- **`Component` trait**: Main interface for UI components with rendering and event handling
- **`ComponentAccessor` trait**: Manages component properties (name, area, active state, children)
- **`ComponentHandler`**: Wraps components for lifecycle management
- **`ComponentContext`**: Injected context containing children, area, theme, and action handlers

#### 🚀 **Runtime System**
- **`App`**: Main application orchestrator managing event loop and component coordination with batched events/actions
- **`Event` enum**: Input events (Key, Mouse, Tick, Frame, Paste, Resize, etc.)
- **`Action` enum**: Application commands (Quit, Render, AppAction, Key, etc.)
- **`Tui`**: Terminal abstraction layer handling crossterm and ratatui integration

#### ⌨️ **Input System**
- **`KeyBindings`**: HashMap-based keybinding management with multi-key support
- **Key parsing**: String-to-KeyEvent conversion (`"<ctrl-c>"`, `"<alt-x><alt-y>"`)
- **Modifier support**: Full ctrl/alt/shift combinations and special keys
- **`kb!` macro**: Declarative keybinding definition with flexible syntax

#### 🎨 **Theming System**
- **`Theme`**: Color and style definitions with named lookup
- **`ThemeManager`**: Multi-theme management with active theme switching
- **Component integration**: Direct theme access via `get_color()` and `get_style()`

## Installation & Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
weavetui = "0.1.2"
tokio = { version = "1.47.1", features = ["macros", "rt-multi-thread"] }
anyhow = "1.0"
ratatui = "0.29.0"
```

### Simple Application

```rust
use weavetui::prelude::*;

#[component(default)]
struct MyComponent {
    counter: i32,
}

impl Component for MyComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let block = Block::bordered().title("My App");
        let text = format!("Counter: {}", self.counter);
        f.render_widget(Paragraph::new(text).block(block), area);
    }

    fn on_event(&mut self, message: &str) {
        match message {
            "increment" => self.counter += 1,
            "decrement" => self.counter -= 1,
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = App::default()
        .with_components(components![MyComponent::default()])
        .with_keybindings(kb![
            "<ctrl-c>" => Action::Quit,
            "<right>" => "increment",
            "<left>" => "decrement"
        ]);

    app.run().await
}
```

### Advanced Features

#### Component with Children
```rust
#[component(default)]
struct Header;

#[component(default)]
struct Footer;

#[component(children("header" => Header, "footer" => Footer))]
struct MainApp {
    title: String,
}

impl Component for MainApp {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let chunks = Layout::vertical([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Main content
            Constraint::Length(3),  // Footer
        ]).split(area);

        if let Some(header) = self.child_mut("header") {
            header.set_area(chunks[0]);
            header.draw(f, chunks[0]);
        }
        if let Some(footer) = self.child_mut("footer") {
            footer.set_area(chunks[2]);
            footer.draw(f, chunks[2]);
        }
    }
}
```

#### Redux Store Injection into Children
Two common patterns to give children access to `dispatch()` and `store()`:

- Child as Redux component with its own store:
```rust
#[derive(Clone, Debug, Default, PartialEq)]
struct ToggleState { on: bool }
impl AppState for ToggleState {}
#[derive(Clone, Debug)] enum ToggleAction { Toggle }
fn reducer(s:&ToggleState,a:&ToggleAction)->ToggleState{ match a { ToggleAction::Toggle=>ToggleState{on:!s.on} } }

#[component(state=ToggleState, action=ToggleAction, reducer=reducer)]
struct ChildToggle;

impl Component for ChildToggle {
    fn on_event(&mut self, msg:&str) { if msg=="toggle" { self.dispatch(ToggleAction::Toggle); } }
}
```

- Parent injects a store into child declared via `children(...)`:
```rust
#[component(children("child" => ChildToggle))]
struct Parent;

impl Component for Parent {
    fn init(&mut self, _area: Rect) {
        let store = weavetui_core::redux::Store::new(ToggleState::default(), reducer);
        if let Some(child) = self.get_children().get_mut("child") {
            *child = Box::new(ChildToggle::new(store));
        }
    }
}
```

#### Advanced Keybindings
```rust
let keybindings = kb![
    "<ctrl-c>" => Action::Quit,
    "<ctrl-x><ctrl-s>" => "save",
    "<alt-enter>" => "fullscreen",
    "<f1>" => "help",
    "q" => Action::Quit,
];
```

## Development

### Building and Testing

```bash
# Build the entire workspace
cargo build --release

# Run tests across all crates
cargo test --verbose

# Run examples
cargo run --example counter_app
cargo run --example on_off_redux
cargo run --example on_off_redux_children
cargo run --example on_off_children
cargo run --example store_in_children

# Build documentation
cargo doc --open
```

### Termux (Android) Notes
- Ensure Rust toolchain is installed via `rustup` in Termux.
- Terminal should support ANSI escape codes; crossterm works in Termux.
- If `cargo doc --open` fails, run `cargo doc` and open docs from `target/doc` manually.

## Component Development Guide

- Prefer `weavetui::prelude::*` for imports.
- Always bind `Action::Quit` (e.g., `<ctrl-c>`) to ensure graceful exit.
- Use theming via `Theme`/`ThemeManager`; access with `get_color()` and `get_style()` from components.
- For child component access, downcast if you need concrete types.

## API Reference

- `Component`, `ComponentAccessor`, `App`, `Action`, `Event`, `KeyBindings`, `Theme`, `ThemeManager`
- Macros: `#[component]`, `kb!`, `components!`

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Ensure CI passes (`cargo build --verbose`, `cargo test --verbose`).

## License

MIT — see [LICENSE](LICENSE).

---

<div align="center">
  <p><strong>Built with ❤️ by <a href="https://github.com/mzyui">Val</a></strong></p>
  <p>
    <a href="https://github.com/mzyui/weavetui">🌟 Star on GitHub</a> •
    <a href="https://docs.rs/weavetui">📚 Documentation</a> •
    <a href="https://crates.io/crates/weavetui">📦 Crates.io</a>
  </p>
</div>