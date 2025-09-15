//! This module contains internal structures and types used within the `weavetui_core` crate, primarily for managing component state and context.
//! These types are generally not intended for direct public consumption but are essential for the framework's internal workings.

use std::collections::BTreeMap;
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;
use crate::event::Action;
use crate::theme::ThemeManager;
use crate::Component;

/// A struct to hold the internal state of a component, injected by the `#[component]` macro.
#[derive(Debug)]
pub struct ComponentContext {
    /// A map of child components, keyed by their names.
    pub children: BTreeMap<String, Box<dyn Component>>,
    /// The rectangular area assigned to the component for rendering.
    pub area: Option<Rect>,
    /// A flag indicating whether the component is currently active.
    pub active: bool,
    /// An optional sender for dispatching actions to the application's event loop.
    pub action_tx: Option<UnboundedSender<Action>>,
    /// The theme manager instance for the component.
    pub theme_manager: ThemeManager,
}

impl Default for ComponentContext {
    fn default() -> Self {
        Self {
            children: BTreeMap::new(),
            area: None,
            active: true, // Default to true
            action_tx: None,
            theme_manager: ThemeManager::default(),
        }
    }
}