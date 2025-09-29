//! Event and action definitions for the application's event-driven architecture.

use {
    crossterm::event::{KeyEvent, MouseEvent},
    std::fmt::{Display, Formatter, Result},
    strum::EnumString,
};

#[derive(Debug, PartialEq, Eq, Clone, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Quit,
    AppAction(String),
    Key(String),
}

impl Display for Action {
    /// Convert action to debug string
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let enum_str = write!(f, "{:?}", self);
        enum_str
    }
}

/// Events from terminal input and the app loop
#[derive(Clone, Debug)]
pub enum Event {
    Init,
    Quit,
    Error,
    Tick,
    Render,
    FocusGained,
    FocusLost,
    Paste(String),
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

/// Action type for keybinding flexibility
pub enum ActionKind {
    Stringified(String),
    Full(Action),
}

impl From<&str> for ActionKind {
    fn from(s: &str) -> Self {
        ActionKind::Stringified(s.to_string())
    }
}

impl From<String> for ActionKind {
    fn from(s: String) -> Self {
        ActionKind::Stringified(s)
    }
}

impl From<Action> for ActionKind {
    fn from(a: Action) -> Self {
        ActionKind::Full(a)
    }
}
