# weavetui_derive

![Crates.io](https://img.shields.io/crates/v/weavetui_derive) ![Docs.rs](https://docs.rs/weavetui_derive/badge.svg)

`weavetui_derive` is the procedural macro companion for the `weavetui` Text User Interface (TUI) framework. It provides the powerful `#[component]` attribute macro, significantly reducing boilerplate code and streamlining the development of `weavetui` components.

## ✨ Features

*   **Automatic Trait Implementation:** Automatically implements `weavetui_core::Component` and `weavetui_core::ComponentAccessor` for your structs, handling the necessary boilerplate.
*   **Declarative Child Management:** Easily define and manage child components directly within your component's attribute, fostering a clear and hierarchical UI structure.
*   **Default Implementations:** Provides sensible default implementations for component methods, allowing you to focus on custom logic.
*   **Reduced Boilerplate:** Drastically cuts down on repetitive code, making component creation faster and less error-prone.

## 🚀 Getting Started

To use the `#[component]` macro in your `weavetui` project, add `weavetui_derive` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
weavetui_derive = { version = "0.1.1" } # Or specify a path/git dependency for development
```

## 📚 Usage

Apply the `#[component]` attribute to your struct definitions. The macro will automatically generate the necessary trait implementations. A practical example of its usage can be found in the `counter_app.rs` example within the main `weavetui` repository.

### Basic Component

```rust
use weavetui_derive::component;

#[component(default)]
struct MySimpleComponent {
    // Your component's fields
}

// MySimpleComponent now implements weavetui_core::Component and weavetui_core::ComponentAccessor
```

### Component with Children

You can declare child components directly within the `#[component]` attribute. The macro will automatically create a `children` field (if not present) and initialize the specified children.

```rust
use weavetui_derive::component;
use weavetui_core::Component; // This import might not be strictly necessary for the example, but good for context

#[component(default)]
struct HeaderComponent;

#[component(default)]
struct FooterComponent;

#[component(default)]
struct ButtonComponent;

#[component(default, children(
    "header" => HeaderComponent,
    "footer" => FooterComponent,
    "button_area" => ButtonComponent,
))]
struct ParentComponent {
    title: String,
}

// ParentComponent will have a `children` field (BTreeMap<String, Box<dyn weavetui_core::Component>>)
// initialized with instances of HeaderComponent, FooterComponent, and ButtonComponent.
```

## 🤝 Contributing

We welcome contributions to `weavetui_derive`! Please refer to the main `weavetui` project's [CONTRIBUTING.md](../../CONTRIBUTING.md) for detailed guidelines on how to get involved, report issues, and submit pull requests.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](../../LICENSE) file for details.