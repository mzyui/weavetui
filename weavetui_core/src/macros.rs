/// Creates an array of keybindings.
///
/// Each action will be converted into an `ActionKind`.
///
/// This macro accepts two syntaxes:
///
/// 1. `<key> => <action>` syntax:
///
/// ```rust
/// # use weavetui::{kb, event::Action};
/// let keybindings = kb![
///     "<q>" => Action::Quit,
///     "d" => "app:drink-mate"
/// ];
/// ```
///
/// 2. `(<key>, <action>)` syntax:
///
/// ```rust
/// # use weavetui::{kb, event::Action};
/// let keybindings = kb![
///     ("<q>", Action::Quit),
///     ("d", "app:drink-mate")
/// ];
/// ```
#[macro_export]
macro_rules! kb {
    // Accepts "<key>" => <action> syntax
    ($($key:expr => $action:expr),* $(,)?) => {
        [
            $(($key, $crate::event::ActionKind::from($action))),*
        ]
    };

    // Accepts ("<key>", <action>) syntax
    ($(($key:expr, $action:expr)),* $(,)?) => {
        [
            $(($key, $crate::event::ActionKind::from($action))),*
        ]
    };
}

/// Creates a vector of components from a list of component instances.
///
/// Each component will be boxed as a `Box<dyn Component>`.
///
/// ## Usage
///
/// ```rust,ignore
/// use weavetui::{components, Component};
///
/// #[derive(Default)]
/// struct MyComponent1;
/// #[derive(Default)]
/// struct MyComponent2;
///
/// // impl Component for MyComponent1 ...
/// // impl Component for MyComponent2 ...
///
/// let my_components: Vec<Box<dyn Component>> = components![
///     MyComponent1::default(),
///     MyComponent2::default()
/// ];
/// ```
#[macro_export]
macro_rules! components {
    ( $( $x:expr $( => $t:ty )* ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(
                    Box::new($x)
                        as Box<dyn $crate::Component $( $t + )* >
                );
            )*
            temp_vec
        }
    };
}
