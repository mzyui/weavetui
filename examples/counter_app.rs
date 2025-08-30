use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph},
};
use weavetui::Component;
use weavetui_core::{ComponentAccessor, app::App, components, event::Action, kb};
use weavetui_derive::component;

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
}

impl Component for Counter {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let block = Block::bordered()
            .title(" WeaveTUI ")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text = format!(
            "This is a tui template.\n\
            Press Ctrl-C` to stop running.\n\
            Press left and right to increment and decrement the counter respectively\n\n\
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
            "app:increment" => self.increment_counter(),
            "app:decrement" => self.decrement_counter(),
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let simple_component = Counter::default().as_active();

    let mut app = App::default()
        .with_components(components![simple_component])
        .with_keybindings(kb![
            "<ctrl-c>" => Action::Quit,
            "<right>" => "app:increment",
            "<left>" => "app:decrement"
        ]);

    app.run().await?;

    dbg!(app);

    Ok(())
}
