<div align="center">
  <a href="https://git.io/typing-svg">
    <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&duration=2000&pause=1000&color=36BCF7&center=true&vCenter=true&width=500&lines=weavetui;A+Modern%2C+Robust%2C+and+Modular+TUI+Framework" alt="Typing SVG" />
  </a>
</div>

<div align="center">

| **Crate**           | **Version**                                                                                             | **Docs**                                                                                              |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `weavetui`          | [![crates.io](https://img.shields.io/crates/v/weavetui.svg)](https://crates.io/crates/weavetui)             | [![docs.rs](https://img.shields.io/docsrs/weavetui)](https://docs.rs/weavetui)                         |
| `weavetui_core`     | [![crates.io](https://img.shields.io/crates/v/weavetui_core.svg)](https://crates.io/crates/weavetui_core) | [![docs.rs](https://img.shields.io/docsrs/weavetui_core)](https://docs.rs/weavetui_core)             |
| `weavetui_derive`   | [![crates.io](https://img.shields.io/crates/v/weavetui_derive.svg)](https://crates.io/crates/weavetui_derive) | [![docs.rs](https://img.shields.io/docsrs/weavetui_derive)](https://docs.rs/weavetui_derive)         |

<br>

[![Build Status](https://github.com/mzyui/weavetui/actions/workflows/rust.yml/badge.svg)](https://github.com/mzyui/weavetui/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

</div>

---

`weavetui` is a modern, robust, and modular **Text User Interface (TUI) framework** for Rust, designed to simplify the development of sophisticated and interactive terminal applications. This repository serves as both the primary application showcasing the framework's capabilities and the foundational crates that enable its powerful component-based architecture.

## ✨ Features

*   **Component-Driven Architecture:** Leverage a declarative, component-based model for building complex UIs with ease.
*   **Automatic Trait Implementation:** Utilize the `weavetui_derive` procedural macro for automatic implementation of `Component` and `ComponentAccessor` traits, significantly reducing boilerplate.
*   **Modular Design:** Built with `weavetui_core` as its foundation, ensuring high modularity, maintainability, and extensibility.
*   **Event Handling System:** A flexible event system for managing keyboard, mouse, and custom events within your TUI applications.
*   **Real-world Application Example:** The `weavetui` application itself serves as a comprehensive example, demonstrating best practices for component composition, state management, and event handling.
*   **Interactive User Experience:** Designed to deliver responsive and engaging user experiences directly within the terminal environment.

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

## ⚙️ How `weavetui` Works

`weavetui` is designed to simplify the development of complex Text User Interface (TUI) applications in Rust. Its essence lies in its component-based architecture and the use of procedural macros to reduce boilerplate. Here's a detailed overview of how it works:

```mermaid
graph TD
    subgraph " Phase 1: Compile Time"
        A[Developer writes Component Struct & adds `#[component]`] --> B{Rust Compiler};
        B --> C[`weavetui_derive` Crate];
        C --"Macro `#[component]` active"--> D[Implementation of `Component` & `ComponentAccessor` traits is automatically generated];
        D --> E[Complete Component Code];
    end

    subgraph "Phase 2: Runtime"
        F[`weavetui` Application starts] --> G[Terminal Initialization & `ComponentManager`];
        G --> H{Main Event Loop};

        subgraph "Event & Render Cycle"
            H --"Waiting for input..."--> I[User provides input (Keyboard/Mouse)];
            I --> J[`EventHandler` from `weavetui_core`];
            J --"Event distributed"--> K[`ComponentManager`];
            K --"Finds active component & calls `handle_key_events`"--> L[Active Component];
            L --"1. Internal state is modified"--> L;
            L --"2. Returns 'Action' (optional)"--> M{Action Handling};
            M --"Action processed (e.g., 'quit', 'submit')"--> H;
            K --"Triggers render process after event"--> N[Render Engine];
            N --"Calls `draw()` on each visible component"--> O[Each Component];
            O --"Renders view to buffer"--> P[Terminal Buffer];
            P --> Q[Terminal Display updated];
            Q --> H;
        end
    end

    style A fill:#f9f,stroke:#333,stroke-width:2px
    style C fill:#f9f,stroke:#333,stroke-width:2px
    style F fill:#ccf,stroke:#333,stroke-width:2px
    style I fill:#ccf,stroke:#333,stroke-width:2px
```

### Detailed Explanation

1.  **Developer Writes Component**: You define your UI as a `struct` in Rust and add the `#[component]` attribute above it. This attribute is the main key to simplifying the process.

2.  **Compile Time (Macro Magic)**:
    *   When you compile your project (`cargo build`), `weavetui_derive` (a *procedural macro*) becomes active.
    *   This macro automatically writes boilerplate code for you. It implements two important traits from `weavetui_core`:
        *   `Component`: Handles core logic such as `handle_key_events`, `handle_mouse_events`, and `draw`.
        *   `ComponentAccessor`: Provides methods to access common properties like component name and active status.

3.  **Runtime (Application Running)**:
    *   **Initialization**: Your main application starts, setting up the terminal for TUI rendering and creating a `ComponentManager`. This `ComponentManager` is responsible for managing the entire state and lifecycle of all components.
    *   **Event Loop**: The application enters an infinite loop that continuously waits for three things: user input, internal events, and signals to re-render the UI.

4.  **Event Cycle**:
    *   **User Input**: When you press a key (e.g., `j` for down), the `EventHandler` captures it.
    *   **Event Distribution**: This event is passed to the `ComponentManager`.
    *   **Event Handling**: The `ComponentManager` determines which component is currently active, then calls the `handle_key_events` method on that component. This is where your logic (within `impl Component`) is executed. The component can modify its internal state (e.g., change the selected item in a list).
    *   **Actions**: After handling an event, a component can return an `Action` (e.g., as a `String` like `"quit"` or `"save"`). This `Action` is how components communicate back to the `ComponentManager` or other components to trigger larger changes.

5.  **Render Cycle**:
    *   After an event is handled, the `ComponentManager` will trigger the render process to update the display.
    *   The render engine will traverse your component tree and call the `draw()` method on each visible component.
    *   Each component then "draws" itself to a buffer in memory.
    *   Once all components have been drawn to the buffer, the buffer is displayed to the terminal, and you will see the changes on the screen.

6.  **Back to Loop**: The process returns to the beginning, waiting for the next user input. This cycle repeats rapidly, giving the illusion of an interactive and responsive application.

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
