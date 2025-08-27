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
