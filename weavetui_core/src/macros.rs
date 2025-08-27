/// Creates an array of keybindings.
///
/// Each action will be converted into an `ActionKind`.
///
/// This macro accepts two syntaxes:
///
/// 1. `<key> => <action>` syntax:
///
/// ```rust
/// # use matetui::{kb, Action};
/// let keybindings = kb![
///     "<q>" => Action::Quit,
///     "<d>" => "app:drink-mate"
/// ];
/// ```
///
/// 2. `(<key>, <action>)` syntax:
///
/// ```rust
/// # use matetui::{kb, Action};
/// let keybindings = kb![
///     ("<q>", Action::Quit),
///     ("<d>", "app:drink-mate")
/// ];
/// ```
///
/// Each action will be converted into an `ActionKind`.
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

/// Creates a vector of components.
///
/// Each component will be converted into a `Box<dyn Component>`.
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
