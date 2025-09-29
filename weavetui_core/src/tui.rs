//! Terminal user interface management.

use {
    super::event::Event,
    crossterm::{
        cursor,
        event::{
            DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
            Event as CrosstermEvent, KeyEventKind,
        },
        terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    },
    futures_util::{FutureExt, StreamExt},
    ratatui::backend::CrosstermBackend as Backend,
    std::{
        ops::{Deref, DerefMut},
        time::Duration,
    },
    tokio::{
        sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
        task::JoinHandle,
    },
    tokio_util::sync::CancellationToken,
};

pub type IO = std::io::Stdout;

/// Returns a handle to the standard output.
fn io() -> IO {
    std::io::stdout()
}

/// TUI wrapper around ratatui terminal
pub struct Tui {
    pub terminal: ratatui::Terminal<Backend<IO>>,
    pub task: JoinHandle<()>,
    pub cancellation_token: CancellationToken,
    pub event_rx: UnboundedReceiver<Event>,
    pub event_tx: UnboundedSender<Event>,
    pub frame_rate: f64,
    pub tick_rate: f64,
    pub mouse: bool,
    pub paste: bool,
}

impl Tui {
    /// Create a new TUI instance
    pub fn new() -> anyhow::Result<Self> {
        let tick_rate = 4.0;
        let frame_rate = 60.0;
        let terminal = ratatui::Terminal::new(Backend::new(io())).map_err(anyhow::Error::from)?;
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let cancellation_token = CancellationToken::new();
        let task = tokio::task::spawn(async {});
        let mouse = false;
        let paste = false;
        Ok(Self {
            terminal,
            task,
            cancellation_token,
            event_rx,
            event_tx,
            frame_rate,
            tick_rate,
            mouse,
            paste,
        })
    }

    /// Set how often the app updates per second
    pub fn tick_rate(mut self, tick_rate: f64) -> Self {
        self.tick_rate = tick_rate;
        self
    }

    /// Set how often to redraw the screen per second
    pub fn frame_rate(mut self, frame_rate: f64) -> Self {
        self.frame_rate = frame_rate;
        self
    }

    /// Enable mouse capture
    pub fn mouse(mut self, mouse: bool) -> Self {
        self.mouse = mouse;
        self
    }

    /// Enable paste events
    pub fn paste(mut self, paste: bool) -> Self {
        self.paste = paste;
        self
    }

    /// Start the event loop
    pub fn start(&mut self) {
        let tick_delay = std::time::Duration::from_secs_f64(1.0 / self.tick_rate);
        let render_delay = std::time::Duration::from_secs_f64(1.0 / self.frame_rate);
        self.cancel();
        self.cancellation_token = CancellationToken::new();
        let _cancellation_token = self.cancellation_token.clone();
        let _event_tx = self.event_tx.clone();
        self.task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_delay);
            let mut render_interval = tokio::time::interval(render_delay);
            _event_tx
                .send(Event::Init)
                .expect("Failed to send Init event");
            loop {
                let tick_delay = tick_interval.tick();
                let render_delay = render_interval.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    _ = _cancellation_token.cancelled() => {
                        println!("Tui task cancelled");
                        break;
                    }
                    maybe_event = crossterm_event => {
                        match maybe_event {
                        Some(Ok(evt)) => {
                            match evt {
                                CrosstermEvent::Key(key) => {
                                    if key.kind == KeyEventKind::Press {
                                        _event_tx.send(Event::Key(key)).expect("Failed to send Key event");
                                    }
                                },
                                CrosstermEvent::Mouse(mouse) => {
                                    _event_tx.send(Event::Mouse(mouse)).expect("Failed to send Mouse event");
                                },
                                CrosstermEvent::Resize(x, y) => {
                                    _event_tx.send(Event::Resize(x, y)).expect("Failed to send Resize event");
                                },
                                CrosstermEvent::FocusLost => {
                                    _event_tx.send(Event::FocusLost).expect("Failed to send FocusLost event");
                                },
                                CrosstermEvent::FocusGained => {
                                    _event_tx.send(Event::FocusGained).expect("Failed to send FocusGained event");
                                },
                                CrosstermEvent::Paste(s) => {
                                    _event_tx.send(Event::Paste(s)).expect("Failed to send Paste event");
                                },

                            }
                        }
                        Some(Err(_)) => {
                            _event_tx.send(Event::Error).expect("Failed to send Error event");
                        }
                        None => {},
                        }
                    },
                    _ = tick_delay => {
                        _event_tx.send(Event::Tick).expect("Failed to send Tick event");
                    },
                    _ = render_delay => {
                        _event_tx.send(Event::Render).expect("Failed to send Render event");
                    },
                }
            }
        });
    }

    /// Stop the event loop
    pub fn stop(&self) -> anyhow::Result<()> {
        self.cancel();
        let mut counter = 0;
        while !self.task.is_finished() {
            std::thread::sleep(Duration::from_millis(1));
            counter += 1;
            if counter > 50 {
                self.task.abort();
                return Err(anyhow::anyhow!("Tui task did not finish in time, aborted."));
            }
            if counter > 100 {
                return Err(anyhow::anyhow!(
                    "Tui task did not finish in time, giving up."
                ));
            }
        }
        Ok(())
    }

    /// Switch to fullscreen mode
    pub fn enter(&mut self) -> anyhow::Result<()> {
        crossterm::terminal::enable_raw_mode().map_err(anyhow::Error::from)?;
        crossterm::execute!(io(), EnterAlternateScreen, cursor::Hide)
            .map_err(anyhow::Error::from)?;
        if self.mouse {
            crossterm::execute!(io(), EnableMouseCapture).map_err(anyhow::Error::from)?;
        }
        if self.paste {
            crossterm::execute!(io(), EnableBracketedPaste).map_err(anyhow::Error::from)?;
        }
        self.start();
        Ok(())
    }

    /// Return to normal terminal mode
    pub fn exit(&mut self) -> anyhow::Result<()> {
        self.stop()?;
        if crossterm::terminal::is_raw_mode_enabled().map_err(anyhow::Error::from)? {
            self.flush().map_err(anyhow::Error::from)?;
            if self.paste {
                crossterm::execute!(io(), DisableBracketedPaste).map_err(anyhow::Error::from)?;
            }
            if self.mouse {
                crossterm::execute!(io(), DisableMouseCapture).map_err(anyhow::Error::from)?;
            }
            crossterm::execute!(io(), LeaveAlternateScreen, cursor::Show)
                .map_err(anyhow::Error::from)?;
            crossterm::terminal::disable_raw_mode().map_err(anyhow::Error::from)?;
        }
        Ok(())
    }

    /// Cancel the background task
    pub fn cancel(&self) {
        self.cancellation_token.cancel();
    }

    /// Temporarily exit fullscreen mode
    pub fn suspend(&mut self) -> anyhow::Result<()> {
        self.exit()
    }

    /// Return to fullscreen mode after suspend
    pub fn resume(&mut self) -> anyhow::Result<()> {
        self.enter()
    }

    /// Get the next event from the queue
    pub async fn next(&mut self) -> Option<Event> {
        self.event_rx.recv().await
    }
}

impl Deref for Tui {
    type Target = ratatui::Terminal<Backend<IO>>;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Tui {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.exit().expect("Failed to exit Tui cleanly during drop");
    }
}
