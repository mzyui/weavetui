use weavetui_core::{
    Component,
    ComponentAccessor,
    event::Action,
    tui::Tui,
};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Paragraph, Block, Borders},
};

#[derive(Debug)]
pub struct SimpleComponent {
    message: String,
}

impl Default for SimpleComponent {
    fn default() -> Self {
        Self { message: "Hello from SimpleComponent!".to_string() }
    }
}

impl ComponentAccessor for SimpleComponent {
    fn name(&self) -> String { "SimpleComponent".to_string() }
    fn is_active(&self) -> bool { true }
    fn set_active(&mut self, _active: bool) { /* ... */ }
    fn register_action_handler(&mut self, _tx: tokio::sync::mpsc::UnboundedSender<String>) { /* ... */ }
    fn send(&self, _action: &str) { /* ... */ }
    fn send_action(&self, _action: Action) { /* ... */ }
    fn as_active(self) -> Self { self }
    fn get_children(&mut self) -> &mut std::collections::BTreeMap<String, Box<dyn Component>> {
        // Return a mutable reference to an empty BTreeMap if no children
        static mut CHILDREN: Option<std::collections::BTreeMap<String, Box<dyn Component>>> = None;
        unsafe {
            CHILDREN.get_or_insert_with(std::collections::BTreeMap::new)
        }
    }
}

impl Component for SimpleComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Simple Component Example");
        f.render_widget(Paragraph::new(self.message.clone()).block(block), area);
    }

    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) -> Option<Action> {
        // Handle key events
        None
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut tui = Tui::new()?
        .tick_rate(4.0)
        .frame_rate(30.0)
        .mouse(false)
        .paste(false);
    tui.enter()?;

    let mut component = SimpleComponent::default();

    loop {
        tui.draw(|f| {
            component.draw(f, f.size());
        })?;

        if let Some(event) = tui.next().await {
            match event {
                weavetui_core::event::Event::Key(key) => {
                    if key.code == crossterm::event::KeyCode::Char('q') {
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    tui.exit()?;
    Ok(())
}
