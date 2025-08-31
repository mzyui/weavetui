[![Typing SVG](https://readme-typing-svg.demolab.com?font=Fira+Code&duration=1500&pause=1000&center=true&vCenter=true&width=460&lines=weavetui;A+Modern+TUI+Framework+and+Application)](https://git.io/typing-svg)


[![crates.io](https://img.shields.io/crates/v/weavetui_core.svg)](https://crates.io/crates/weavetui_core)
[![crates.io](https://img.shields.io/crates/v/weavetui_derive.svg)](https://crates.io/crates/weavetui_derive)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

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
