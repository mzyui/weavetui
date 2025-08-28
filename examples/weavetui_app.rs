use ratatui::widgets::Paragraph;
use weavetui_core::{Component, ComponentAccessor, app::App, components, event::Action, kb};
use weavetui_derive::component;

mod counter_component;
use counter_component::CounterComponent;

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
    let counter_component = CounterComponent::default().as_active();

    let mut app = App::default()
        .with_components(components![home, counter_component])
        .with_keybindings(kb![
            "<ctrl-c>" => Action::Quit,
            "<b>" => "app:toggle_feature_x",
        ]);

    // The App's run method already handles Action::AppAction by calling handle_message on components.
    // The `Content` component's `on_event` method will receive the message.
    // So, no direct modification to the App's run loop is needed here, as the existing
    // mechanism for AppAction dispatching is sufficient.

    app.run().await?;

    dbg!(app);

    Ok(())
}
