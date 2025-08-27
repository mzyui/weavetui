use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use tokio::sync::mpsc::{self, error::TryRecvError};

use crate::{
    event::{Action, ActionKind, Event},
    keyboard::KeyBindings,
    tui::Tui,
    Component, ComponentHandler,
};

#[derive(Debug)]
pub struct App {
    tick_rate: f64,
    frame_rate: f64,
    should_quit: bool,
    // pub should_suspend: bool,
    keybindings: KeyBindings,
    last_tick_key_events: Vec<KeyEvent>,
    mouse: bool,
    paste: bool,
    component_handlers: Vec<ComponentHandler>,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
}

impl Default for App {
    fn default() -> Self {
        let (action_tx, action_rx) = mpsc::unbounded_channel::<Action>();
        Self {
            last_tick_key_events: Vec::default(),
            keybindings: KeyBindings::default(),
            component_handlers: Vec::new(),
            frame_rate: 24.into(),
            tick_rate: 1.into(),
            should_quit: false,
            // should_suspend: false,
            mouse: false,
            paste: false,
            action_tx,
            action_rx,
        }
    }
}

impl App {
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

    /// Set the components
    pub fn with_components(mut self, components: Vec<Box<dyn Component>>) -> Self {
        self.component_handlers
            .extend(components.into_iter().map(ComponentHandler::for_));
        self
    }

    // Set the keybindings
    pub fn with_keybindings<const N: usize>(
        mut self,
        kb: [(&str, impl Into<ActionKind>); N],
    ) -> Self {
        self.keybindings = KeyBindings::new(kb);
        self
    }

    /// Set the tick rate
    pub fn with_tick_rate(mut self, tick_rate: impl Into<f64>) -> Self {
        self.tick_rate = tick_rate.into();
        self
    }

    /// Set the frame rate
    pub fn with_frame_rate(mut self, frame_rate: impl Into<f64>) -> Self {
        self.frame_rate = frame_rate.into();
        self
    }

    /// Set the mouse
    pub fn with_mouse(mut self, mouse: bool) -> Self {
        self.mouse = mouse;
        self
    }

    /// Set the paste
    pub fn with_paste(mut self, paste: bool) -> Self {
        self.paste = paste;
        self
    }

    fn send(&self, action: Action) -> Result<()> {
        self.action_tx.send(action)?;

        // match action {
        //     Action::AppAction(cmd) => self.action_tx.send(cmd)?,
        //     Action::Key(key) => self.action_tx.send(key)?,
        //     action => self.action_tx.send(action.to_string())?,
        // };
        Ok(())
    }

    fn try_recv(&mut self) -> Result<Action, TryRecvError> {
        self.action_rx.try_recv()
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate)
            .mouse(self.mouse)
            .paste(self.paste);

        tui.enter()?;

        for handler in self.component_handlers.iter_mut() {
            handler.receive_action_handler(self.action_tx.clone());
        }

        for handler in self.component_handlers.iter_mut() {
            handler.handle_init(tui.size()?);
        }

        loop {
            if let Some(e) = tui.next().await {
                match e {
                    // Event::Resize(x, y) => self.send(Action::Resize(x, y))?,
                    Event::Render => self.send(Action::Render)?,
                    Event::Tick => self.send(Action::Tick)?,
                    Event::Quit => self.send(Action::Quit)?,
                    Event::Key(key) => {
                        if let Some(action) = self.keybindings.get(&[key]) {
                            self.send(action.clone())?;
                        } else {
                            // If the key was not handled as a single key action,
                            // then consider it for multi-key combinations.
                            self.last_tick_key_events.push(key);

                            // Check for multi-key combinations
                            if let Some(action) = self.keybindings.get(&self.last_tick_key_events) {
                                self.send(action.clone())?;
                            }
                        }

                        // send the key event as simple key event too (not as action) if it's a
                        // single alphanumeric char key
                        if let KeyCode::Char(c) = key.code {
                            if c.is_alphanumeric() {
                                self.send(Action::Key(c.to_string()))?;
                            }
                        }
                    }
                    _ => {}
                }
                let mut actions = Vec::new();

                for handler in self.component_handlers.iter_mut() {
                    let component_actions = handler.handle_events(&Some(e.clone()));
                    actions.extend(component_actions);
                }

                for action in actions {
                    self.send(action)?;
                }
            }

            while let Ok(action) = self.try_recv() {
                match action {
                    Action::Quit => self.should_quit = true,
                    Action::Render => {
                        tui.draw(|f| {
                            for handler in self.component_handlers.iter_mut() {
                                handler.handle_draw(f, f.area());
                            }
                        })?;
                    }
                    Action::Tick => {
                        self.last_tick_key_events.drain(..);
                    }

                    Action::AppAction(ref m) => {
                        for handler in self.component_handlers.iter_mut() {
                            handler.handle_message(m.as_str());
                        }
                    }

                    _ => {}
                }

                for handler in self.component_handlers.iter_mut() {
                    handler.handle_update(&action);
                }
            }

            if self.should_quit {
                tui.stop()?;
                break;
            }
        }

        tui.exit()?;

        Ok(())
    }
}
