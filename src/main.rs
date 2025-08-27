use weavetui_core::{self, kb, tui::Tui};
use weavetui_derive::component;

#[component(default, children("title"=>Header))]
struct Footer {}

#[component(default, children("footer"=>Footer))]
struct Button {}

#[component(default)]
struct Header {}

#[component(default, children("title"=>Header))]
struct Sidebar {}

#[component(default, children(
    "button" => Button,
))]
struct Content {}

#[component(children(
    "header" => Header,
    "sidebar" => Sidebar,
    "content" => Content,
    "footer" => Footer,
), default)]
struct Home {
    title: String,
    version: String,
    is_logged_in: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let mut tui = Tui::new()?
    //     .tick_rate(20.0)
    //     .frame_rate(30.0)
    //     .mouse(false)
    //     .paste(false);
    //
    // tui.enter()?;
    //
    //
    // tui.exit()?;

    let home = Home::default();
    dbg!(home);

    kb![
        "<ctrl-c>" => "exit"
    ];

    Ok(())
}
