<div align="center">
  <a href="https://git.io/typing-svg">
    <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&duration=2000&pause=1000&color=36BCF7&center=true&vCenter=true&width=500&lines=weavetui;A+Modern%2C+Robust%2C+and+Modular+TUI+Framework" alt="Typing SVG" />
  </a>
</div>

<div align="center">

| **Crate**           | **Version**                                                                                             | **Docs**                                                                                              |
| --- | --- | --- |
| `weavetui`         | [![crates.io](https://img.shields.io/crates/v/weavetui.svg)](https://crates.io/crates/weavetui)             | [![docs.rs](https://img.shields.io/docsrs/weavetui)](https://docs.rs/weavetui)                         |
| [`weavetui_core`](https://github.com/mzyui/weavetui/tree/main/weavetui_core)     | [![crates.io](https://img.shields.io/crates/v/weavetui_core.svg)](https://crates.io/crates/weavetui_core) | [![docs.rs](https://img.shields.io/docsrs/weavetui_core)](https://docs.rs/weavetui_core)             |
| [`weavetui_derive`](https://github.com/mzyui/weavetui/tree/main/weavetui_derive)   | [![crates.io](https://img.shields.io/crates/v/weavetui_derive.svg)](https://crates.io/crates/weavetui_derive) | [![docs.rs](https://img.shields.io/docsrs/weavetui_derive)](https://docs.rs/weavetui_derive)         |

<br>

[![Build Status](https://github.com/mzyui/weavetui/actions/workflows/rust.yml/badge.svg)](https://github.com/mzyui/weavetui/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

</div>

---

`weavetui` is a modern, robust, and modular **Text User Interface (TUI) framework** for Rust, built on top of `ratatui` and `tokio`. It is designed to simplify the development of sophisticated and interactive terminal applications. This repository serves as both the primary application showcasing the framework's capabilities and the foundational crates that enable its powerful component-based architecture.

## ✨ Why `weavetui`?

`weavetui` empowers developers to build complex and interactive terminal applications with ease, offering:

*   **Component-Driven Development:** Build UIs using reusable, self-contained components that implement the `Component` and `ComponentAccessor` traits, making your code modular and maintainable.
*   **Reduced Boilerplate:** Leverage procedural macros (`weavetui_derive`) to automatically implement common traits, allowing you to focus on your application's unique logic rather than repetitive setup.
*   **Robust Event Handling:** A flexible and comprehensive event system, powered by `tokio`, handles keyboard, mouse, and custom `Action`s, ensuring a responsive user experience.
*   **Clear Architecture:** A well-defined separation of concerns between core functionalities (`weavetui_core`) and macro-based development (`weavetui_derive`) promotes clarity and extensibility.

*   **Interactive & Responsive:** Designed from the ground up to deliver engaging and fast-responding user interfaces directly within the terminal, with configurable tick and frame rates.

## 🚀 Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

*   Rust programming language (stable or beta channel recommended). You can install it via `rustup`:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
*   Cargo, Rust's package manager (comes with Rust installation).

### Installation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/weavetui/weavetui.git
    cd weavetui
    ```

2.  **Build the project:**
    This command will compile all crates within the workspace (`weavetui`, `weavetui_core`, `weavetui_derive`).
    ```bash
    cargo build --release
    ```
    Using `--release` is recommended for optimized performance.

### Running the Application

To launch the `weavetui` example application (which demonstrates the framework's capabilities):

```bash
cargo run --release
```

This command compiles and runs the main `weavetui` application, serving as a practical demonstration of the framework's features directly in your terminal.

## 📂 Project Structure

This repository is organized as a Rust workspace, containing the following crates:

*   `weavetui/` (root): The main application crate that orchestrates the UI and application logic, serving as a practical demonstration of the framework.
*   `weavetui_core/`: A foundational library defining core TUI traits (`Component`, `ComponentAccessor`), event handling mechanisms, and utility functions.
*   `weavetui_derive/`: A procedural macro crate providing the `#[component]` attribute for automatic trait implementation, simplifying component creation.

## ⚙️ How `weavetui` Works: A Simplified Overview

`weavetui` simplifies TUI development in Rust through a component-based architecture and powerful procedural macros. Here's a breakdown of its core mechanics:

```mermaid
graph TD
    subgraph " Phase 1: Compile Time"
        A[Developer defines Component Struct & adds &#91;component&#93;] --> B{Rust Compiler};
        B --> C[`weavetui_derive` Crate];
        C --"Macro &#91;component&#93; active"--> D[Implementation of `Component` & `ComponentAccessor` traits is automatically generated];
        D --> E[Complete Component Code];
    end

    subgraph "Phase 2: Runtime"
        F[`weavetui` Application starts App] --> G[Terminal Initialization Tui & ComponentManager];
        G --> H{Main Event Loop App.run()};

        subgraph "Event & Render Cycle"
            H --"Waiting for input/events..."--> I[User input KeyEvent, MouseEvent, Paste or internal events Tick, Render];
            I --> J[`Tui` polls `crossterm` events];
            J --"Emits `weavetui_core::event::Event`"--> K[`App` main application loop];
            K --"Dispatches `Event` to `ComponentManager`"--> L[`ComponentManager`];
            L --"Finds active component & calls `handle_key_events`, etc."--> M[Active Component];
            M --"1. Internal state is modified"--> M;
            M --"2. Returns `Action` (optional)"--> N{Action Handling};
            N --"`Action` processed by `App` or other components"--> O[State Update & Re-render Trigger];
            O --> P[Render Engine `Tui`];
            P --"Calls `draw()` on each visible component"--> Q[Each Component];
            Q --"Renders view to buffer"--> R[Terminal Buffer];
            R --> S[Terminal Display updated];
            S --> H;
        end
    end

    style A fill:#f9f,stroke:#333,stroke-width:2px
    style C fill:#f9f,stroke:#333,stroke-width:2px
    style F fill:#ccf,stroke:#333,stroke-width:2px
    style I fill:#ccf,stroke:#333,stroke-width:2px
```

### Detailed Explanation

1.  **Define Your UI as Components**: You start by defining your UI elements as Rust `struct`s. By adding the `#[component]` attribute from `weavetui_derive` to your struct, you tell the framework that this struct should behave as a UI component. This macro automatically injects essential fields like `_ctx` (containing `children`, `area`, `active`, `action_tx`, and `theme_manager`) into your component, managing its internal state and context.

2.  **Automatic Trait Implementation (Compile Time)**:
    *   During compilation, the `#[component]` procedural macro automatically generates the necessary boilerplate code.
    *   Specifically, it implements the `Component` and `ComponentAccessor` traits (defined in `weavetui_core`) for your struct. These traits provide the fundamental methods for handling events, drawing the UI, and managing component properties. This automation significantly reduces manual coding, allowing you to focus on the unique logic of your components.

3.  **Application Lifecycle (Runtime)**:
    *   **Initialization**: When your `weavetui` application starts (driven by the `App` struct), it sets up the terminal environment using the `Tui` utility. The `App` also initializes a `ComponentManager`, which is the central orchestrator responsible for holding all your UI components and managing their state and interactions.
    *   **Event Loop**: The `App` then enters a continuous asynchronous loop (within `App.run()`), constantly monitoring for user input (like key presses or mouse clicks) and internal events (like `Tick` for periodic updates or `Render` for UI redraws). The `Tui` component polls `crossterm` events and translates them into `weavetui_core::event::Event` enum variants.

4.  **Event Handling Flow**:
    *   **Input Capture & Dispatch**: When an `Event` occurs (e.g., a `KeyEvent` from a key press), the `Tui` emits it, and the `App`'s main loop receives it. The `App` then dispatches this `Event` to the `ComponentManager`.
    *   **Component Action**: The `ComponentManager` identifies the currently active component (and its children) and dispatches the event to the appropriate handler method (e.g., `handle_key_events`, `handle_mouse_events`, `on_event`). Here, your component's logic processes the event, potentially modifying its internal state (e.g., updating a counter, changing a selected item).
    *   **Actions & Communication**: After processing an event, a component can optionally return an `Action` (e.g., a command like `Action::Quit` or `Action::AppAction("submit")`). These `Action`s are a primary way for components to communicate with the `App` or other parts of the application, triggering broader changes or application-level responses. The `App` processes these `Action`s, which can lead to state updates or further event dispatches.

5.  **Rendering the UI**:
    *   After events and actions are processed, and if a re-render is triggered (e.g., by a `Render` action or a state change), the `ComponentManager` initiates a re-render of the UI.
    *   The `Tui`'s rendering engine iterates through all visible components, calling their `draw()` method.
    *   Each component draws its visual representation onto an in-memory buffer using `ratatui` primitives.
    *   Once the buffer is complete, its contents are efficiently sent to the terminal, updating what the user sees on the screen.

6.  **Continuous Interaction**: This entire cycle of event handling, action processing, and rendering repeats rapidly, driven by `tokio`'s asynchronous runtime, creating the illusion of a fluid, interactive, and responsive terminal application.

## 🧪 Running Tests

To ensure the stability and correctness of the framework and application, you can run the test suite:

```bash
cargo test
```

This command will execute all unit and integration tests across all workspace crates.

## 🤝 Contributing

We welcome and encourage contributions from the community! Whether you're looking to report a bug, suggest an enhancement, or contribute code, please refer to our [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines on how to get involved.

## 🗺️ Roadmap

*   Enhanced layout management system.
*   More sophisticated styling and theming capabilities.
*   Expanded set of pre-built UI components.
*   Improved accessibility features.
*   Comprehensive documentation and tutorials.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.