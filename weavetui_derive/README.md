<div align="center">
  <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&weight=700&size=26&duration=2500&pause=800&color=36BCF7&center=true&vCenter=true&width=800&lines=Procedural+Macros+for+weavetui" alt="weavetui_derive banner" />
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

### Redux Helpers (Macro-based)
- **`state=Type`**: Attach a Redux state type implementing `AppState`
- **`action=Type`**: Attach an action enum type
- **`reducer=fn`**: Provide the reducer function signature `(state: &State, action: &Action) -> State`
- **Generated helpers**: `.new(store)`, `.with_initial_state(...)`, `.dispatch(...)`, `.store()`, `.update_from_store()`, `.state()`

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

```rust
use weavetui::prelude::*;

#[component(default)]
pub struct SimpleComponent {
    pub message: String,
}
```

### Component with Children

```rust
use weavetui::prelude::*;

#[component(children("header" => Header, "footer" => Footer))]
pub struct MainLayout {
    pub content: String,
}
```

### Redux Component

```rust
use weavetui::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
struct ToggleState { on: bool }
impl AppState for ToggleState {}
#[derive(Clone, Debug)] enum ToggleAction { Toggle }
fn reducer(s:&ToggleState,a:&ToggleAction)->ToggleState{ match a { ToggleAction::Toggle=>ToggleState{on:!s.on} } }

#[component(state=ToggleState, action=ToggleAction, reducer=reducer)]
pub struct Toggle;

impl Component for Toggle {
    fn on_event(&mut self, msg:&str) { if msg=="toggle" { self.dispatch(ToggleAction::Toggle); } }
}
```

### Inject a Shared Store into Children

```rust
use weavetui::prelude::*;

#[component(state=ToggleState, action=ToggleAction, reducer=reducer)]
pub struct ChildToggle;

#[component(children("child" => ChildToggle))]
pub struct Parent;

impl Component for Parent {
    fn init(&mut self, _area: Rect) {
        let store = Store::new(ToggleState::default(), reducer);
        if let Some(child) = self.get_children().get_mut("child") {
            *child = Box::new(ChildToggle::new(store));
        }
    }
}
```

## ⚠️ Notes & Gotchas

- Do not combine `#[component(default)]` with a manual `impl Component for ...` that defines `draw()` — this will cause conflicts.
- Always import `ComponentAccessor` when manually interacting with child components.
- Ensure `Action::Quit` is bound in your App keybindings before running (`<ctrl-c>` recommended).

## 💡 Best Practices

- Use `weavetui::prelude::*` for concise imports and macro access.
- Prefer macro-based Redux components when you need `dispatch()`/`store()` helpers.
- Forward parent actions/messages to children as needed; children can call `.dispatch(...)` if Redux-enabled.

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
