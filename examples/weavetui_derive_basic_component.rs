use weavetui_derive::component;
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

#[component(default)]
struct BasicComponent {
    message: String,
}

impl BasicComponent {
    fn new(message: String) -> Self {
        Self { message, children: Default::default(), _active: Default::default(), _action_tx: Default::default() }
    }
}

#[component(default, children("basic" => BasicComponent))]
struct ParentComponent {
    title: String,
}

impl ParentComponent {
    fn new(title: String) -> Self {
        let mut children = std::collections::BTreeMap::new();
        children.insert("basic".to_string(), Box::new(BasicComponent::new("Hello from Parent!".to_string())) as Box<dyn Component>);
        Self { title, children, _active: Default::default(), _action_tx: Default::default() }
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

    let mut parent_component = ParentComponent::new("My App".to_string());

    loop {
        tui.draw(|f| {
            parent_component.draw(f, f.size());
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
