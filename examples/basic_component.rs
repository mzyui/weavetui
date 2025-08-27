use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};
use std::collections::BTreeMap;
use tokio::sync::mpsc::UnboundedSender;
use weavetui_core::{Children, Component, ComponentAccessor, event::Action};

#[derive(Debug)]
pub struct BasicComponent {
    name: String,
    active: bool,
    children: Children,
    action_tx: Option<UnboundedSender<Action>>,
}

impl Default for BasicComponent {
    fn default() -> Self {
        Self {
            name: "BasicComponent".to_string(),
            active: true,
            children: BTreeMap::new(),
            action_tx: None,
        }
    }
}

impl ComponentAccessor for BasicComponent {
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
                .expect("Failed to send action from BasicComponent");
        }
    }

    fn send_action(&self, action: Action) {
        if let Some(tx) = &self.action_tx {
            tx.send(action)
                .expect("Failed to send action from BasicComponent");
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

impl Component for BasicComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .title("Basic Component")
            .borders(Borders::ALL);
        let paragraph =
            Paragraph::new("This is a basic component implemented manually.").block(block);
        f.render_widget(paragraph, area);
    }
}

fn main() {
    // This example is meant to be used within the weavetui application context.
    // It demonstrates a basic component implementation.
    // To run this, you would typically integrate it into an App.
    println!(
        "This is a basic component example. It needs to be run within a weavetui application."
    );
}
