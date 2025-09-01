use ratatui::style::{Color, Style};
use std::collections::HashMap;

/// Represents a collection of styles and colors that can be applied to the UI.
#[derive(Debug, Default, Clone)]
pub struct Theme {
    /// The name of the theme.
    pub name: String,
    /// A map of style names to `ratatui::style::Style`.
    pub styles: HashMap<String, Style>,
    /// A map of color names to `ratatui::style::Color`.
    pub colors: HashMap<String, Color>,
}

impl Theme {
    /// Creates a new, empty `Theme` with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name for the new theme.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            styles: HashMap::new(),
            colors: HashMap::new(),
        }
    }

    /// Adds a new style to the theme.
    ///
    /// # Arguments
    ///
    /// * `name` - The name to associate with the style.
    /// * `style` - The `Style` to add.
    pub fn add_style(mut self, name: String, style: Style) -> Self {
        self.styles.insert(name, style);
        self
    }

    /// Adds a new color to the theme.
    ///
    /// # Arguments
    ///
    /// * `name` - The name to associate with the color.
    /// * `color` - The `Color` to add.
    pub fn add_color(mut self, name: String, color: Color) -> Self {
        self.colors.insert(name, color);
        self
    }

    /// Retrieves a style by its name.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the style to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `Style` if found, otherwise `None`.
    pub fn get_style(&self, key: &str) -> Option<&Style> {
        self.styles.get(key)
    }

    /// Retrieves a color by its name.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the color to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `Color` if found, otherwise `None`.
    pub fn get_color(&self, key: &str) -> Option<&Color> {
        self.colors.get(key)
    }
}

/// Manages a collection of themes and the currently active theme.
#[derive(Debug, Default, Clone)]
pub struct ThemeManager {
    themes: HashMap<String, Theme>,
    active_theme_name: Option<String>,
}

impl ThemeManager {
    /// Creates a new, empty `ThemeManager`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a theme to the manager.
    ///
    /// # Arguments
    ///
    /// * `theme` - The `Theme` to add.
    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.insert(theme.name.clone(), theme);
    }

    /// Sets the active theme by name.
    ///
    /// If the theme name is not found, a warning is printed to stderr.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the theme to set as active.
    pub fn set_active_theme(&mut self, name: &str) {
        if !self.themes.contains_key(name) {
            eprintln!("Warning: Theme '{}' not found.", name);
        }
        self.active_theme_name = Some(name.to_string());
    }

    /// Retrieves the currently active theme.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the active `Theme` if one is set and found, otherwise `None`.
    pub fn get_active_theme(&self) -> Option<&Theme> {
        self.active_theme_name
            .as_ref()
            .and_then(|name| self.themes.get(name))
    }

    /// Gets a style from the active theme.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the style to retrieve.
    ///
    /// # Returns
    ///
    /// The requested `Style`, or a default `Style` if the theme or style is not found.
    pub fn get_current_style(&self, key: &str) -> Style {
        self.get_active_theme()
            .and_then(|theme| theme.get_style(key))
            .cloned()
            .unwrap_or_default()
    }

    /// Gets a color from the active theme.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the color to retrieve.
    ///
    /// # Returns
    ///
    /// The requested `Color`, or `Color::Reset` if the theme or color is not found.
    pub fn get_current_color(&self, key: &str) -> Color {
        self.get_active_theme()
            .and_then(|theme| theme.get_color(key))
            .cloned()
            .unwrap_or(Color::Reset)
    }
}
