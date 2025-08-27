# Contributing to weavetui

We are thrilled that you're interested in contributing to `weavetui`! Your contributions help make this project better for everyone. This document outlines the guidelines and processes for contributing to the `weavetui` project.

## Code of Conduct

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md) (if you have one, otherwise omit or create one). By participating in this project, you agree to abide by its terms.

## How Can I Contribute?

There are many ways to contribute, not just by writing code:

*   **Reporting Bugs:** If you find a bug, please open an issue on our [GitHub Issues](https://github.com/your-username/weavetui/issues) page.
*   **Suggesting Enhancements:** Have an idea for a new feature or an improvement to an existing one? Open an issue to discuss it.
*   **Writing Code:** Implement new features, fix bugs, or improve existing code.
*   **Improving Documentation:** Help us improve our `README.md`, `CONTRIBUTING.md`, or any other documentation.
*   **Reviewing Pull Requests:** Provide feedback on pull requests from other contributors.

## Getting Started

1.  **Fork the repository:** Click the "Fork" button at the top right of the repository page.
2.  **Clone your forked repository:**
    ```bash
    git clone https://github.com/your-username/weavetui.git # Replace with your GitHub username
    cd weavetui
    ```
3.  **Set up the upstream remote:**
    ```bash
    git remote add upstream https://github.com/original-owner/weavetui.git # Replace with original owner's URL
    ```
4.  **Create a new branch:**
    ```bash
    git checkout -b feature/your-feature-name # For new features
    git checkout -b bugfix/your-bug-name     # For bug fixes
    ```

## Making Changes

*   **Code Style:** Adhere to the existing Rust code style. We use `rustfmt` for formatting.
    ```bash
    cargo fmt
    ```
*   **Linting:** Ensure your code passes lint checks.
    ```bash
    cargo clippy
    ```
*   **Testing:** Write unit and integration tests for your changes. Ensure all existing tests pass.
    ```bash
    cargo test
    ```
*   **Commit Messages:** Write clear, concise, and descriptive commit messages. Follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification if possible (e.g., `feat: add new component`, `fix: resolve rendering issue`).

## Submitting a Pull Request

1.  **Push your changes:**
    ```bash
    git push origin your-branch-name
    ```
2.  **Open a Pull Request:** Go to your forked repository on GitHub and click the "New pull request" button.
3.  **Describe your changes:** Provide a clear title and description for your pull request. Explain the problem it solves, the features it adds, or the improvements it makes. Reference any related issues.
4.  **Address feedback:** Be prepared to respond to feedback and make further changes if requested during the review process.

## Thank You!

Your contributions are highly valued and appreciated. Thank you for helping us build a better `weavetui`!
