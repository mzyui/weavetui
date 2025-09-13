use std::collections::BTreeMap;
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;
use crate::event::Action;
use crate::theme::ThemeManager;
use crate::Component;

/// A struct to hold the internal state of a component, injected by the `#[component]` macro.
#[derive(Debug, Default)]
pub struct ComponentContext {
    pub children: BTreeMap<String, Box<dyn Component>>,
    pub area: Option<Rect>,
    pub active: bool,
    pub action_tx: Option<UnboundedSender<Action>>,
    pub theme_manager: ThemeManager,
}