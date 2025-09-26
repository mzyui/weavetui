# weavetui_core

Core library for the `weavetui` Text User Interface (TUI) framework. It provides the essential traits, types, and utilities for building interactive terminal applications with a component-driven architecture.

<p align="center">
  <a href="https://crates.io/crates/weavetui_core"><img alt="crates.io" src="https://img.shields.io/crates/v/weavetui_core.svg"></a>
  <a href="https://docs.rs/weavetui_core"><img alt="docs.rs" src="https://docs.rs/weavetui_core/badge.svg"></a>
</p>

## Features

- Traits: `Component`, `ComponentAccessor` for rendering, events, and component metadata
- Runtime primitives: `App`, `ComponentManager`, `Event`, `Action`
- Input: `KeyBindings` and `kb!` macro; keyboard/mouse/paste handling
- Theming: `Theme` and `ThemeManager` with convenience accessors
- Terminal I/O: `Tui` abstraction over `crossterm` and `ratatui`

## Installation

Add to your crate:
```toml
[dependencies]
weavetui_core = "0.1.2"
```

## Usage

Implement components directly or via `weavetui_derive::#[component]` for reduced boilerplate.

```rust
use weavetui_core::{Component, ComponentAccessor, event::Action};
use ratatui::{Frame, layout::Rect, widgets::{Paragraph, Block, Borders}, style::{Color, Style}};
use tokio::sync::mpsc::UnboundedSender;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct MyCoreComponent {
    message: String,
    action_tx: Option<UnboundedSender<Action>>,
}

impl ComponentAccessor for MyCoreComponent {
    fn name(&self) -> String { "MyCoreComponent".into() }
    fn is_active(&self) -> bool { true }
    fn set_active(&mut self, _active: bool) {}
    fn area(&self) -> Option<Rect> { None }
    fn set_area(&mut self, _area: Rect) {}
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) { self.action_tx = Some(tx); }
    fn send(&self, action_str: &str) { if let Some(tx) = &self.action_tx { let _ = tx.send(Action::AppAction(action_str.into())); } }
    fn send_action(&self, action: Action) { if let Some(tx) = &self.action_tx { let _ = tx.send(action); } }
    fn get_children(&mut self) -> &mut BTreeMap<String, Box<dyn Component>> { static mut C: BTreeMap<String, Box<dyn Component>> = BTreeMap::new(); unsafe { &mut C } }
    fn get_theme_manager(&self) -> &weavetui_core::theme::ThemeManager { static TM: weavetui_core::theme::ThemeManager = weavetui_core::theme::ThemeManager::new(); &TM }
    fn set_theme_manager(&mut self, _tm: weavetui_core::theme::ThemeManager) {}
}

impl Component for MyCoreComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let block = Block::default().borders(Borders::ALL).title("My Core Component").border_style(Style::default().fg(Color::Blue));
        f.render_widget(Paragraph::new(self.message.clone()).block(block), area);
    }
}
```

## Notes

- Prefer using `weavetui_derive` for components to automatically inject `_ctx: ComponentContext` and implement `ComponentAccessor`.
- The `App` runtime requires a quit binding; bind `Action::Quit` (e.g., `<ctrl-c>`) for graceful exit.

## Contributing

Please see the top-level [CONTRIBUTING.md](../CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md).

## License

MIT. See [LICENSE](../LICENSE).