use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};
use std::collections::BTreeMap;
use tokio::sync::mpsc::UnboundedSender;
use weavetui_core::{Children, Component, ComponentAccessor, event::Action};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub struct CounterComponent {
    name: String,
    active: bool,
    children: Children,
    action_tx: Option<UnboundedSender<Action>>,
    count: u32,
}

impl Default for CounterComponent {
    fn default() -> Self {
        Self {
            name: "CounterComponent".to_string(),
            active: true,
            children: BTreeMap::new(),
            action_tx: None,
            count: 0,
        }
    }
}

impl ComponentAccessor for CounterComponent {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) {
        self.action_tx = Some(tx);
    }

    fn send(&self, action: &str) {
        if let Some(tx) = &self.action_tx {
            tx.send(Action::AppAction(action.to_string()))
                .expect("Failed to send action from CounterComponent");
        }
    }

    fn send_action(&self, action: Action) {
        if let Some(tx) = &self.action_tx {
            tx.send(action)
                .expect("Failed to send action from CounterComponent");
        }
    }

    fn as_active(mut self) -> Self
    where
        Self: Sized,
    {
        self.active = true;
        self
    }

    fn get_children(&mut self) -> &mut Children {
        &mut self.children
    }
}

impl Component for CounterComponent {
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        if key.code == KeyCode::Char('c') {
            self.count += 1;
            if self.count >= 5 {
                self.send("count_threshold_reached");
            }
        }
        None
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .title("Counter Component")
            .borders(Borders::ALL);
        let paragraph =
            Paragraph::new(format!("Count: {}", self.count)).block(block);
        f.render_widget(paragraph, area);
    }
}
