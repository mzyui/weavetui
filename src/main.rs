use weavetui_core::{Component, ComponentAccessor};
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
struct Home {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut home = Home::default().as_active();
    home.child_mut("footer").unwrap().set_active(false);

    dbg!(home);
    Ok(())
}
