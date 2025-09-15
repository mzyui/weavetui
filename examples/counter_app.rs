use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph},
};
use weavetui_core::{Component, app::App, components, event::Action, kb};
use weavetui_derive::component;

const INCREMENT_EVENT: &str = "app:increment";
const DECREMENT_EVENT: &str = "app:decrement";
const RESET_EVENT: &str = "app:reset";

#[component]
pub struct Counter {
    pub counter: u32,
}

impl Counter {
    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }

    pub fn reset_counter(&mut self) {
        self.counter = 0;
    }
}

impl Component for Counter {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let block = Block::bordered()
            .title(" WeaveTUI ")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text = format!(
            "\n\n\n\n\
            This is a tui template.\n\n\
            Press `Ctrl-C` to stop running.\n\
            Press left and right to increment and decrement the counter respectively\n\
            Press `r` to reset the counter\n\n\
            Counter: {}",
            self.counter
        );

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        f.render_widget(paragraph, area);
    }

    fn on_event(&mut self, message: &str) {
        match message {
            INCREMENT_EVENT => self.increment_counter(),
            DECREMENT_EVENT => self.decrement_counter(),
            RESET_EVENT => self.reset_counter(),
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let simple_component = Counter::default();

    let mut app = App::default()
        .with_components(components![simple_component])
        .with_keybindings(kb![
            "<ctrl-c>" => Action::Quit,
            "<right>" => INCREMENT_EVENT,
            "<left>" => DECREMENT_EVENT,
            "<r>" => RESET_EVENT
        ]);

    app.run().await?;

    dbg!(app);

    Ok(())
}
