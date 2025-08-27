use ratatui::widgets::Paragraph;
use weavetui_core::{Component, ComponentAccessor, app::App, components, event::Action, kb};
use weavetui_derive::component;

#[component()]
struct Content {
    content: String,
}

impl Component for Content {
    fn draw(&mut self, f: &mut ratatui::Frame<'_>, area: ratatui::prelude::Rect) {
        f.render_widget(
            Paragraph::new(self.content.as_str()),
            area.inner(ratatui::layout::Margin {
                horizontal: 1,
                vertical: 1,
            }),
        );
    }

    fn on_event(&mut self, message: &str) {
        self.content = format!("{message:#?}");
    }
}

#[component(children(
    "content" => Content,
), default)]
struct Home {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let home = Home::default().as_active();

    let mut app = App::default()
        .with_components(components![home])
        .with_keybindings(kb![
            "<ctrl-c>" => Action::Quit,
            "<b>" => "app:toggle_feature_x",
        ]);
    app.run().await?;

    dbg!(app);

    Ok(())
}
