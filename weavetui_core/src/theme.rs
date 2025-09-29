//! Theme management for the `weavetui` framework.

use ratatui::style::{Color, Style};
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Theme {
    pub name: String,
    pub styles: HashMap<String, Style>,
    pub colors: HashMap<String, Color>,
}

impl Theme {
    /// Create a new theme with a name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            styles: HashMap::new(),
            colors: HashMap::new(),
        }
    }

    /// Add a style to this theme (chainable)
    pub fn add_style(mut self, name: &str, style: Style) -> Self {
        self.styles.insert(name.to_string(), style);
        self
    }

    /// Add a color to this theme (chainable)
    pub fn add_color(mut self, name: &str, color: Color) -> Self {
        self.colors.insert(name.to_string(), color);
        self
    }

    /// Get a style by name (returns default if not found)
    pub fn get_style(&self, key: &str) -> Style {
        self.styles.get(key).cloned().unwrap_or_default()
    }

    /// Get a color by name (returns Reset if not found)
    pub fn get_color(&self, key: &str) -> Color {
        self.colors.get(key).cloned().unwrap_or(Color::Reset)
    }
}

#[derive(Debug, Default, Clone)]
pub struct ThemeManager {
    themes: HashMap<String, Theme>,
    active_theme_name: Option<String>,
}

impl ThemeManager {
    /// Creates a new, empty theme manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a theme to the manager
    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.insert(theme.name.clone(), theme);
    }

    /// Set which theme is currently active
    pub fn set_active_theme(&mut self, name: &str) {
        if !self.themes.contains_key(name) {
            eprintln!("Warning: Theme '{}' not found.", name);
        }
        self.active_theme_name = Some(name.to_string());
    }

    /// Get the currently active theme
    pub fn get_active_theme(&self) -> Option<&Theme> {
        self.active_theme_name
            .as_ref()
            .and_then(|name| self.themes.get(name))
    }

    /// Get a style from the current theme
    pub fn get_current_style(&self, key: &str) -> Style {
        self.get_active_theme()
            .map(|theme| theme.get_style(key))
            .unwrap_or_default()
    }

    /// Get a color from the current theme
    pub fn get_current_color(&self, key: &str) -> Color {
        self.get_active_theme()
            .map(|theme| theme.get_color(key))
            .unwrap_or(Color::Reset)
    }

    /// Check if there's an active theme set
    pub fn has_active_theme(&self) -> bool {
        self.active_theme_name.is_some()
    }
}
