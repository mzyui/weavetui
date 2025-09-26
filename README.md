# weavetui

A modern, robust, and modular Text User Interface (TUI) framework for Rust, built on top of `ratatui` and `tokio`. This workspace contains the primary library crate (`weavetui`) alongside `weavetui_core` and `weavetui_derive`, enabling a clean component-based architecture with minimal boilerplate.

<p align="center">
  <a href="https://crates.io/crates/weavetui"><img alt="crates.io" src="https://img.shields.io/crates/v/weavetui.svg"></a>
  <a href="https://docs.rs/weavetui"><img alt="docs.rs" src="https://img.shields.io/docsrs/weavetui"></a>
  <a href="https://crates.io/crates/weavetui_core"><img alt="weavetui_core" src="https://img.shields.io/crates/v/weavetui_core.svg"></a>
  <a href="https://docs.rs/weavetui_core"><img alt="docs.rs core" src="https://img.shields.io/docsrs/weavetui_core"></a>
  <a href="https://crates.io/crates/weavetui_derive"><img alt="weavetui_derive" src="https://img.shields.io/crates/v/weavetui_derive.svg"></a>
  <a href="https://docs.rs/weavetui_derive"><img alt="docs.rs derive" src="https://img.shields.io/docsrs/weavetui_derive"></a>
  <a href="https://github.com/mzyui/weavetui/actions/workflows/rust.yml"><img alt="Build Status" src="https://github.com/mzyui/weavetui/actions/workflows/rust.yml/badge.svg"></a>
  <a href="LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
</p>

---

## Highlights

- Component-driven UI: implement `Component` + `ComponentAccessor` for modular design
- Declarative ergonomics: `#[component]` macro (from `weavetui_derive`) injects context and default impls
- Robust runtime: `App` + `ComponentManager` orchestrate events (`Event`) and commands (`Action`)
- First-class keybindings: `KeyBindings` and `kb!` macro for simple/compound shortcuts
- Theming support: `ThemeManager` with `get_color`/`get_style` helpers in components

## Workspace Layout

- `weavetui/` (root library)
  - Re-exports a convenient `prelude` and the `component` macro
  - Path: `src/lib.rs`
- `weavetui_core/`
  - Core traits, modules, and utilities: `app.rs`, `component_manager.rs`, `event.rs`, `internal.rs`, `keyboard.rs`, `lib.rs`, `macros.rs`, `theme.rs`, `tui.rs`
- `weavetui_derive/`
  - Proc-macro crate implementing `#[component]`
- Examples: `examples/counter_app.rs`
- CI: `.github/workflows/rust.yml`

## Quick Start

Prerequisites:
- Rust toolchain (via `rustup`), Cargo included

Build the workspace:
```bash
cargo build --release
```

Run the example application:
```bash
cargo run --example counter_app
```
Keybindings in the counter example:
- `<ctrl-c>` → quit
- `<right>` / `<left>` → increment / decrement
- `r` → reset

Run tests:
```bash
cargo test
```

## Using weavetui in Your App

Add a dependency and use the prelude:
```rust
use weavetui::prelude::*;

#[component(default)]
struct MyComponent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = App::new([("<q>", "app:quit")], vec![Box::new(MyComponent::default())]);
    app.run().await?;
    Ok(())
}
```

## Design Notes

- Components receive `_ctx: weavetui_core::ComponentContext` injected by the macro
- Children are managed via `BTreeMap<String, Box<dyn Component>>`
- Actions are dispatched through `UnboundedSender<Action>` registered in `_ctx`
- Use `ThemeManager` accessors in `ComponentAccessor` for style consistency
- The `App` runtime requires a quit binding; CI builds/tests enforce overall health

## Contributing

Contributions are welcome. Please read:
- [CONTRIBUTING.md](CONTRIBUTING.md)
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)

## License

Licensed under MIT. See [LICENSE](LICENSE).