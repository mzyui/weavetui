# weavetui_derive

![Crates.io](https://img.shields.io/crates/v/weavetui_derive) ![Docs.rs](https://docs.rs/weavetui_derive/badge.svg)

`weavetui_derive` is the procedural macro companion for the `weavetui` Text User Interface (TUI) framework. It provides the powerful `#[component]` attribute macro, significantly reducing boilerplate code and streamlining the development of `weavetui` components.

## ✨ Features

*   **Automatic Trait Implementation:** The `#[component]` attribute automatically implements `weavetui_core::Component` and `weavetui_core::ComponentAccessor` for your structs. This includes injecting a `_ctx` field (of type `weavetui_core::ComponentContext`) to manage internal component state like children, area, active status, action sender, and theme manager.
*   **Declarative Child Management:** Easily define and manage child components directly within your component's attribute using `children = [...]`, fostering a clear and hierarchical UI structure. The macro handles the creation and initialization of these children.
*   **Reduced Boilerplate:** Drastically cuts down on repetitive code for trait implementations and common component setup, making component creation faster and less error-prone, allowing you to focus on custom logic.
*   **Integration with `weavetui_core`:** Seamlessly integrates with the core traits and types defined in `weavetui_core`, providing a cohesive and powerful development experience.

## 🚀 Getting Started

To use the `#[component]` macro in your `weavetui` project, add `weavetui_derive` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
weavetui_derive = { version = "0.1.1" } # Or specify a path/git dependency for development
```

## 📚 Usage

Apply the `#[component]` attribute to your struct definitions. The macro will automatically generate the necessary trait implementations for `weavetui_core::Component` and `weavetui_core::ComponentAccessor`.

### Injected Fields

When you use the `#[component]` attribute, a `pub _ctx: weavetui_core::ComponentContext` field is automatically added to your struct (if not already present). This `_ctx` field encapsulates essential component state:

*   `children: BTreeMap<String, Box<dyn Component>>`: A map to hold child components, allowing for nested UI structures.
*   `area: Option<ratatui::layout::Rect>`: Stores the rendering area assigned to the component by its parent.
*   `active: bool`: A flag indicating whether the component is currently active and should respond to events.
*   `action_tx: Option<UnboundedSender<Action>>`: A channel sender for dispatching actions to the application's central event loop.
*   `theme_manager: weavetui_core::theme::ThemeManager`: Manages the theme and styles for the component and its children.

### Basic Component

```rust
use weavetui_derive::component;

#[component(default)]
struct MySimpleComponent {
    // Your component's custom fields
}

// MySimpleComponent now implements weavetui_core::Component and weavetui_core::ComponentAccessor
// and has a `_ctx` field for internal management.
```

### Component with Children

You can declare child components directly within the `#[component]` attribute using the `children = [...]` syntax. The macro will automatically initialize these children within the `_ctx.children` map.

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

// ParentComponent will have its `_ctx.children` field initialized with instances
// of HeaderComponent, FooterComponent, and ButtonComponent.
```

A practical example of its usage can be found in the `counter_app.rs` example within the main `weavetui` repository.

## 🤝 Contributing

We welcome contributions to `weavetui_derive`! Please refer to the main `weavetui` project's [CONTRIBUTING.md](../../CONTRIBUTING.md) for detailed guidelines on how to get involved, report issues, and submit pull requests.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](../../LICENSE) file for details.