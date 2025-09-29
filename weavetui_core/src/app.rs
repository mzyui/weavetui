//! Application module for `weavetui`.
//!
//! This module defines the core `App` structure, which manages the application's lifecycle,
//! event handling, and component interactions within the TUI environment.

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use tokio::sync::mpsc::{self, error::TryRecvError};

use crate::{
    event::{Action, ActionKind, Event},
    keyboard::KeyBindings,
    theme::{Theme, ThemeManager},
    tui::Tui,
    Component, ComponentHandler,
};

/// Configuration structure for the application settings.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub mouse: bool,
    pub paste: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            tick_rate: 1.0,
            frame_rate: 24.0,
            mouse: false,
            paste: false,
        }
    }
}

/// `App` is the main application structure that orchestrates the TUI.
///
/// It manages the event loop, handles user input, dispatches actions to components,
/// and renders the UI.
#[derive(Debug)]
pub struct App {
    config: AppConfig,
    should_quit: bool,
    keybindings: KeyBindings,
    last_tick_key_events: Vec<KeyEvent>,
    component_handlers: Vec<ComponentHandler>,
    theme_manager: ThemeManager,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
}

impl Default for App {
    /// Creates a new `App` instance with default settings. This includes an unbounded MPSC channel for actions, default tick and frame rates, and no initial components or keybindings.
    fn default() -> Self {
        let (action_tx, action_rx) = mpsc::unbounded_channel::<Action>();
        Self {
            config: AppConfig::default(),
            last_tick_key_events: Vec::default(),
            keybindings: KeyBindings::default(),
            component_handlers: Vec::new(),
            theme_manager: ThemeManager::default(),
            should_quit: false,
            action_tx,
            action_rx,
        }
    }
}

impl App {
    /// Creates a new `App` instance with specified keybindings and initial components.
    ///
    /// # Arguments
    ///
    /// * `kb` - An array of keybinding tuples, mapping key combinations to action kinds.
    /// * `components` - A vector of boxed `Component` traits to be managed by the app.
    pub fn new<const N: usize>(kb: [(&str, &str); N], components: Vec<Box<dyn Component>>) -> Self {
        let keybindings = KeyBindings::new(kb);

        let component_handlers = components
            .into_iter()
            .map(ComponentHandler::for_)
            .collect::<Vec<_>>();

        Self {
            component_handlers,
            keybindings,
            ..Self::default()
        }
    }

    /// Adds a collection of components to the application.
    ///
    /// # Arguments
    ///
    /// * `components` - A vector of boxed `Component` traits to be added.
    ///
    /// # Returns
    ///
    /// The modified `App` instance.
    pub fn with_components(mut self, components: Vec<Box<dyn Component>>) -> Self {
        self.component_handlers
            .extend(components.into_iter().map(ComponentHandler::for_));
        self
    }

    /// Sets the keybindings for the application.
    ///
    /// # Arguments
    ///
    /// * `kb` - An array of keybinding tuples, mapping key combinations to action kinds.
    ///
    /// # Returns
    ///
    /// The modified `App` instance.
    pub fn with_keybindings<const N: usize>(
        mut self,
        kb: [(&str, impl Into<ActionKind>); N],
    ) -> Self {
        self.keybindings = KeyBindings::new(kb);
        self
    }

    /// Sets the tick rate for the application's event loop.
    ///
    /// The tick rate determines how often the application processes events and updates its state.
    ///
    /// # Arguments
    ///
    /// * `tick_rate` - The desired tick rate in Hertz (Hz).
    ///
    /// # Returns
    ///
    /// The modified `App` instance.
    pub fn with_tick_rate(mut self, tick_rate: impl Into<f64>) -> Self {
        self.config.tick_rate = tick_rate.into();
        self
    }

    /// Sets the frame rate for rendering the application's UI.
    ///
    /// The frame rate determines how often the application redraws the terminal screen.
    ///
    /// # Arguments
    ///
    /// * `frame_rate` - The desired frame rate in frames per second (fps).
    ///
    /// # Returns
    ///
    /// The modified `App` instance.
    pub fn with_frame_rate(mut self, frame_rate: impl Into<f64>) -> Self {
        self.config.frame_rate = frame_rate.into();
        self
    }

    /// Enables or disables mouse event handling for the application.
    ///
    /// # Arguments
    ///
    /// * `mouse` - `true` to enable mouse support, `false` to disable.
    ///
    /// # Returns
    ///
    /// The modified `App` instance.
    pub fn with_mouse(mut self, mouse: bool) -> Self {
        self.config.mouse = mouse;
        self
    }

    /// Enables or disables paste event handling for the application.
    ///
    /// # Arguments
    ///
    /// * `paste` - `true` to enable paste support, `false` to disable.
    ///
    /// # Returns
    ///
    /// The modified `App` instance.
    pub fn with_paste(mut self, paste: bool) -> Self {
        self.config.paste = paste;
        self
    }

    /// Adds a theme to the application.
    ///
    /// If no active theme is set, this theme will be set as the active theme.
    ///
    /// # Arguments
    ///
    /// * `theme` - The `Theme` to add.
    ///
    /// # Returns
    ///
    /// The modified `App` instance.
    pub fn add_theme(mut self, theme: Theme) -> Self {
        if !self.theme_manager.has_active_theme() {
            self.theme_manager.set_active_theme(&theme.name);
        }

        self.theme_manager.add_theme(theme);
        self
    }

    /// Sends an `Action` to the application's action channel.
    ///
    /// This method is used internally to dispatch actions that need to be processed by the `App`
    /// or its components.
    ///
    /// # Arguments
    ///
    /// * `action` - The `Action` to send.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn send(&self, action: Action) -> Result<()> {
        self.action_tx.send(action)?;
        Ok(())
    }

    /// Attempts to receive an `Action` from the application's action channel without blocking.
    ///
    /// # Returns
    ///
    /// A `Result` containing the received `Action` or a `TryRecvError` if no action is available.
    fn try_recv(&mut self) -> Result<Action, TryRecvError> {
        self.action_rx.try_recv()
    }

    /// Handles keyboard events and converts them to actions.
    ///
    /// # Arguments
    ///
    /// * `key` - The key event to handle.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        if let Some(action) = self.keybindings.get(&[key]) {
            self.send(action.clone())?;
        } else {
            self.last_tick_key_events.push(key);
            if let Some(action) = self.keybindings.get(&self.last_tick_key_events) {
                self.send(action.clone())?;
            }
        }

        if let KeyCode::Char(c) = key.code {
            if c.is_alphanumeric() {
                self.send(Action::Key(c.to_string()))?;
            }
        }

        Ok(())
    }

    /// Processes actions received from the action channel.
    ///
    /// # Arguments
    ///
    /// * `action` - The action to process.
    /// * `tui` - The TUI instance for rendering.
    /// * `initialize` - A mutable reference to the initialization flag.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn process_action(&mut self, action: Action, tui: &mut Tui, initialize: &mut bool) -> Result<()> {
        match action {
            Action::Quit => self.should_quit = true,
            Action::Render => {
                tui.draw(|f| {
                    for handler in self.component_handlers.iter_mut() {
                        let area = f.area();
                        if !*initialize {
                            handler.handle_init(area);
                            *initialize = true;
                        }
                        handler.c.set_area(area);
                        handler.handle_draw(f);
                    }
                })?;
            }
            Action::Tick => {
                self.last_tick_key_events.drain(..);
            }
            Action::AppAction(ref m) => {
                for handler in self.component_handlers.iter_mut() {
                    if handler.c.is_active() {
                        handler.handle_message(m.as_str());
                    }
                }
            }
            _ => {}
        }

        for handler in self.component_handlers.iter_mut() {
            handler.handle_update(&action);
        }

        Ok(())
    }

    /// Initializes the TUI and sets up component handlers.
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized TUI or an error.
    fn initialize_tui(&mut self) -> Result<Tui> {
        let mut tui = Tui::new()?
            .tick_rate(self.config.tick_rate)
            .frame_rate(self.config.frame_rate)
            .mouse(self.config.mouse)
            .paste(self.config.paste);

        tui.enter()?;

        for handler in self.component_handlers.iter_mut() {
            handler.receive_action_handler(self.action_tx.clone());
            handler.handle_theme(self.theme_manager.clone());
            handler.handle_custom_keybindings(&mut self.keybindings);
        }

        if !self
            .keybindings
            .0
            .iter()
            .any(|(_, action)| *action == Action::Quit)
        {
            anyhow::bail!("Action::Quit is not bound to any key. Consider binding it for graceful exit (e.g., <ctrl-c>).");
        }

        Ok(tui)
    }

    /// Runs the main application loop.
    ///
    /// This asynchronous function initializes the TUI, sets up event handling, and continuously
    /// processes events (keyboard, mouse, tick, render) and dispatches actions to registered
    /// components. The loop continues until a `Quit` action is received.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the application execution.
    pub async fn run(&mut self) -> Result<()> {
        let mut tui = self.initialize_tui()?;

        let mut initialize = false;
        loop {
            if let Some(e) = tui.next().await {
                if let Err(err) = self.handle_event(e.clone()) {
                    eprintln!("Error handling event: {}", err);
                    continue;
                }
            }

            while let Ok(action) = self.try_recv() {
                if let Err(err) = self.process_action(action, &mut tui, &mut initialize) {
                    eprintln!("Error processing action: {}", err);
                    continue;
                }
            }

            if self.should_quit {
                if let Err(err) = tui.stop() {
                    eprintln!("Error stopping TUI: {}", err);
                }
                break;
            }
        }

        if let Err(err) = tui.exit() {
            eprintln!("Error exiting TUI: {}", err);
        }

        Ok(())
    }

    /// Handles a single event and dispatches appropriate actions.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to handle.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn handle_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Resize(x, y) => self.send(Action::Resize(x, y))?,
            Event::Render => self.send(Action::Render)?,
            Event::Tick => self.send(Action::Tick)?,
            Event::Quit => self.send(Action::Quit)?,
            Event::Key(key) => self.handle_key_event(key)?,
            _ => {}
        }

        let mut actions = Vec::new();
        for handler in self.component_handlers.iter_mut() {
            let component_actions = handler.handle_events(&Some(event.clone()));
            actions.extend(component_actions);
        }

        for action in actions {
            self.send(action)?;
        }

        Ok(())
    }
}
