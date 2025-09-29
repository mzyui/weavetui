//! Internal structures and types for component management.

use std::collections::BTreeMap;
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;
use crate::event::Action;
use crate::theme::ThemeManager;
use crate::Component;

#[derive(Debug)]
pub struct ComponentContext {
    pub children: BTreeMap<String, Box<dyn Component>>,
    pub area: Option<Rect>,
    pub active: bool,
    pub action_tx: Option<UnboundedSender<Action>>,
    pub theme_manager: ThemeManager,
}

impl Default for ComponentContext {
    fn default() -> Self {
        Self {
            children: BTreeMap::new(),
            area: None,
            active: true,
            action_tx: None,
            theme_manager: ThemeManager::default(),
        }
    }
}