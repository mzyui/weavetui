use weavetui_core::{ComponentAccessor, event::Action};
use weavetui_derive::component;

#[component(default)]
pub struct DerivedComponent {
    pub custom_message: String,
}

// The `#[component(default)]` macro automatically implements `Default` and `Component`.
// We can still add custom fields and initialize them in a `new` function or similar,
// but we should not re-implement `Default` or `Component` directly if using `default` attribute.

impl DerivedComponent {
    pub fn new(message: String) -> Self {
        Self {
            custom_message: message,
            ..Default::default()
        }
    }
}

fn main() {
    // This example is meant to be used within the weavetui application context.
    // It demonstrates a component created using the `#[component]` derive macro.
    // To run this, you would typically integrate it into an App.
    println!(
        "This is a derived component example. It needs to be run within a weavetui application."
    );
}
