//! Application module for `weavetui`.

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use tokio::sync::mpsc::{self, error::TryRecvError};
use std::time::{Duration, Instant};

use crate::{
    event::{Action, ActionKind, Event},
    keyboard::KeyBindings,
    theme::{Theme, ThemeManager},
    tui::Tui,
    Component, ComponentHandler,
};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub mouse: bool,
    pub paste: bool,
    pub max_events_per_batch: usize,
    pub max_actions_per_batch: usize,
    pub enable_performance_monitoring: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    pub events_processed: u64,
    pub actions_processed: u64,
    pub average_event_batch_size: f64,
    pub average_action_batch_size: f64,
    pub total_render_time: Duration,
    pub total_event_processing_time: Duration,
    pub last_fps: f64,
    last_frame_time: Option<Instant>,
    frame_count: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            tick_rate: 1.0,
            frame_rate: 24.0,
            mouse: false,
            paste: false,
            max_events_per_batch: 32,
            max_actions_per_batch: 64,
            enable_performance_monitoring: false,
        }
    }
}

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
    event_batch: Vec<Event>,
    action_batch: Vec<Action>,
    metrics: PerformanceMetrics,
}

impl Default for App {
    fn default() -> Self {
        let (action_tx, action_rx) = mpsc::unbounded_channel::<Action>();
        let config = AppConfig::default();
        Self {
            last_tick_key_events: Vec::default(),
            keybindings: KeyBindings::default(),
            component_handlers: Vec::new(),
            theme_manager: ThemeManager::default(),
            should_quit: false,
            action_tx,
            action_rx,
            event_batch: Vec::with_capacity(config.max_events_per_batch),
            action_batch: Vec::with_capacity(config.max_actions_per_batch),
            metrics: PerformanceMetrics::default(),
            config,
        }
    }
}

impl App {
    /// Create an app with custom keybindings and components
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

    /// Add components to your app
    pub fn with_components(mut self, components: Vec<Box<dyn Component>>) -> Self {
        self.component_handlers
            .extend(components.into_iter().map(ComponentHandler::for_));
        self
    }

    /// Set keyboard shortcuts
    pub fn with_keybindings<const N: usize>(
        mut self,
        kb: [(&str, impl Into<ActionKind>); N],
    ) -> Self {
        self.keybindings = KeyBindings::new(kb);
        self
    }

    /// Control how often the app updates (higher = more responsive)
    pub fn with_tick_rate(mut self, tick_rate: impl Into<f64>) -> Self {
        self.config.tick_rate = tick_rate.into();
        self
    }

    /// Control how smooth the animations are (higher = smoother)
    pub fn with_frame_rate(mut self, frame_rate: impl Into<f64>) -> Self {
        self.config.frame_rate = frame_rate.into();
        self
    }

    /// Enable mouse support
    pub fn with_mouse(mut self, mouse: bool) -> Self {
        self.config.mouse = mouse;
        self
    }

    /// Enable clipboard paste support
    pub fn with_paste(mut self, paste: bool) -> Self {
        self.config.paste = paste;
        self
    }

    /// Add a theme to your app
    pub fn add_theme(mut self, theme: Theme) -> Self {
        if !self.theme_manager.has_active_theme() {
            self.theme_manager.set_active_theme(&theme.name);
        }

        self.theme_manager.add_theme(theme);
        self
    }

    /// Turn on performance monitoring to see how fast your app runs
    pub fn with_performance_monitoring(mut self, enabled: bool) -> Self {
        self.config.enable_performance_monitoring = enabled;
        self
    }

    /// Get performance stats (events processed, FPS, etc.)
    pub fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.clone()
    }

    fn send(&self, action: Action) -> Result<()> {
        self.action_tx.send(action)?;
        Ok(())
    }

    fn try_recv(&mut self) -> Result<Action, TryRecvError> {
        self.action_rx.try_recv()
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        if let Some(action) = self.keybindings.get(&[key]) {
            return self.send(action.clone());
        }

        self.last_tick_key_events.push(key);
        if let Some(action) = self.keybindings.get(&self.last_tick_key_events) {
            self.send(action.clone())?;
        }

        if let KeyCode::Char(c) = key.code {
            if c.is_alphanumeric() {
                let mut char_buf = [0u8; 4];
                let char_str = c.encode_utf8(&mut char_buf);
                self.send(Action::Key(char_str.to_string()))?;
            }
        }

        Ok(())
    }

    fn process_action_batch(&mut self, tui: &mut Tui, initialize: &mut bool) -> Result<()> {
        let start_time = if self.config.enable_performance_monitoring {
            Some(Instant::now())
        } else {
            None
        };

        let batch_size = self.action_batch.len();
        let mut needs_render = false;

        for action in self.action_batch.drain(..) {
            match action {
                Action::Quit => self.should_quit = true,
                Action::Render => needs_render = true,
                Action::Tick => {
                    self.last_tick_key_events.clear();
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
        }

        if needs_render {
            let render_start = Instant::now();

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

            if self.config.enable_performance_monitoring {
                let render_duration = render_start.elapsed();
                self.metrics.total_render_time += render_duration;
                self.metrics.frame_count += 1;

                if let Some(last_frame) = self.metrics.last_frame_time {
                    let frame_duration = render_start.duration_since(last_frame);
                    if !frame_duration.is_zero() {
                        self.metrics.last_fps = 1.0 / frame_duration.as_secs_f64();
                    }
                }
                self.metrics.last_frame_time = Some(render_start);
            }
        }

        if let Some(_start) = start_time {
            self.metrics.actions_processed += batch_size as u64;
            self.metrics.average_action_batch_size =
                (self.metrics.average_action_batch_size * (self.metrics.actions_processed - batch_size as u64) as f64
                + batch_size as f64) / self.metrics.actions_processed as f64;
        }

        Ok(())
    }

    fn process_event_batch(&mut self) -> Result<()> {
        let start_time = if self.config.enable_performance_monitoring {
            Some(Instant::now())
        } else {
            None
        };

        let batch_size = self.event_batch.len();

        let events: Vec<Event> = self.event_batch.drain(..).collect();

        for event in events {
            match event {
                Event::Resize(x, y) => self.send(Action::Resize(x, y))?,
                Event::Render => self.send(Action::Render)?,
                Event::Tick => self.send(Action::Tick)?,
                Event::Quit => self.send(Action::Quit)?,
                Event::Key(key) => self.handle_key_event(key)?,
                _ => {}
            }

            let mut component_actions = Vec::new();
            for handler in self.component_handlers.iter_mut() {
                let actions = handler.handle_events(&Some(event.clone()));
                component_actions.extend(actions);
            }

            for action in component_actions {
                self.send(action)?;
            }
        }

        if let Some(_start) = start_time {
            let processing_time = _start.elapsed();
            self.metrics.total_event_processing_time += processing_time;
            self.metrics.events_processed += batch_size as u64;
            self.metrics.average_event_batch_size =
                (self.metrics.average_event_batch_size * (self.metrics.events_processed - batch_size as u64) as f64
                + batch_size as f64) / self.metrics.events_processed as f64;
        }

        Ok(())
    }

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

    /// Start your app and run until the user quits
    pub async fn run(&mut self) -> Result<()> {
        let mut tui = self.initialize_tui()?;

        let mut initialize = false;
        loop {
            while let Some(event) = tui.next().await {
                self.event_batch.push(event);

                // Process batch when full or if we get a critical event
                if self.event_batch.len() >= self.config.max_events_per_batch
                    || matches!(self.event_batch.last(), Some(Event::Quit) | Some(Event::Render)) {
                    break;
                }
            }

            if !self.event_batch.is_empty() {
                if let Err(err) = self.process_event_batch() {
                    eprintln!("Error processing event batch: {}", err);
                }
            }

            while let Ok(action) = self.try_recv() {
                self.action_batch.push(action);

                if self.action_batch.len() >= self.config.max_actions_per_batch
                    || matches!(self.action_batch.last(), Some(Action::Quit) | Some(Action::Render)) {
                    break;
                }
            }

            if !self.action_batch.is_empty() {
                if let Err(err) = self.process_action_batch(&mut tui, &mut initialize) {
                    eprintln!("Error processing action batch: {}", err);
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

}
