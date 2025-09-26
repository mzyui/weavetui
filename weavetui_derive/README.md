# weavetui_derive

Procedural macro crate for the `weavetui` TUI framework. Provides the `#[component]` attribute to eliminate boilerplate when building components.

<p align="center">
  <a href="https://crates.io/crates/weavetui_derive"><img alt="crates.io" src="https://img.shields.io/crates/v/weavetui_derive.svg"></a>
  <a href="https://docs.rs/weavetui_derive"><img alt="docs.rs" src="https://docs.rs/weavetui_derive/badge.svg"></a>
</p>

## Features

- Implements `weavetui_core::Component` and `ComponentAccessor` for your struct
- Injects `pub _ctx: weavetui_core::ComponentContext` if missing
- Optional default `draw` via `#[component(default)]`
- Declarative children via `#[component(children("name" => ChildType, ...))]`

## Installation

```toml
[dependencies]
weavetui_derive = "0.1.2"
```

## Usage

Basic component:
```rust
use weavetui_derive::component;

#[component(default)]
pub struct MyComponent;
```

Component with children:
```rust
use weavetui_derive::component;

#[component(default)]
pub struct Header;
#[component(default)]
pub struct Footer;

#[component(children("header" => Header, "footer" => Footer))]
pub struct Parent { title: String }
```

## Notes

- Prefer using `weavetui::prelude::*` in your app to access common types and the macro re-export.
- The macro also generates a `Default` impl that initializes `_ctx` and declared children.

## Contributing

Please see the top-level [CONTRIBUTING.md](../CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md).

## License

MIT. See [LICENSE](../LICENSE).
