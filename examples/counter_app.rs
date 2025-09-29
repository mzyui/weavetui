use ratatui::{
    Frame,
    layout::{Alignment, Rect, Layout, Direction, Constraint},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Gauge},
};
use weavetui_core::{Component, app::App, components, event::Action, kb};
use weavetui_derive::component;
use std::time::Instant;

const INCREMENT_EVENT: &str = "app:increment";
const DECREMENT_EVENT: &str = "app:decrement";
const RESET_EVENT: &str = "app:reset";

#[component]
pub struct Counter {
    pub counter: u32,
    events_processed: u64,
    last_event_time: Option<Instant>,
    event_rate: f64,
    peak_event_rate: f64,
}

impl Counter {
    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
        self.track_event();
    }

    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
        self.track_event();
    }

    pub fn reset_counter(&mut self) {
        self.counter = 0;
        self.track_event();
    }

    fn track_event(&mut self) {
        let now = Instant::now();
        self.events_processed += 1;

        if let Some(last_time) = self.last_event_time {
            let duration = now.duration_since(last_time);
            if !duration.is_zero() {
                self.event_rate = 1.0 / duration.as_secs_f64();
                if self.event_rate > self.peak_event_rate {
                    self.peak_event_rate = self.event_rate;
                }
            }
        }

        self.last_event_time = Some(now);
    }
}

impl Component for Counter {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ])
            .split(area);

        let main_block = Block::bordered()
            .title(" WeaveTUI - Optimized Counter ")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let main_text = format!(
            "\n\n\
            ⚡ High-Performance Counter App ⚡\n\n\
            Press `Ctrl-C` to stop running.\n\
            Press ← → to increment/decrement (try rapid keypresses!)\n\
            Press `r` to reset the counter\n\n\
            Counter: {} 🎯",
            self.counter
        );

        let main_paragraph = Paragraph::new(main_text)
            .block(main_block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        f.render_widget(main_paragraph, chunks[0]);

        let perf_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(chunks[1]);

        let stats_block = Block::bordered()
            .title(" 📊 Performance Stats ")
            .border_type(BorderType::Rounded);

        let stats_text = format!(
            "Events Processed: {}\n\
            Current Rate: {:.1} events/sec\n\
            Peak Rate: {:.1} events/sec",
            self.events_processed,
            self.event_rate,
            self.peak_event_rate
        );

        let stats_paragraph = Paragraph::new(stats_text)
            .block(stats_block)
            .fg(Color::Green);

        f.render_widget(stats_paragraph, perf_chunks[0]);

        let gauge_block = Block::bordered()
            .title(" ⚡ Event Rate ")
            .border_type(BorderType::Rounded);

        let rate_percentage = if self.peak_event_rate > 0.0 {
            ((self.event_rate / self.peak_event_rate) * 100.0).min(100.0) as u16
        } else {
            0
        };

        let gauge = Gauge::default()
            .block(gauge_block)
            .gauge_style(if rate_percentage > 80 {
                Color::Red
            } else if rate_percentage > 50 {
                Color::Yellow
            } else {
                Color::Green
            })
            .percent(rate_percentage)
            .label(format!("{:.1} evt/s", self.event_rate));

        f.render_widget(gauge, perf_chunks[1]);
    }

    fn on_event(&mut self, message: &str) {
        match message {
            INCREMENT_EVENT => self.increment_counter(),
            DECREMENT_EVENT => self.decrement_counter(),
            RESET_EVENT => self.reset_counter(),
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let simple_component = Counter::default();

    let mut app = App::default()
        .with_components(components![simple_component])
        .with_keybindings(kb![
            "<ctrl-c>" => Action::Quit,
            "<right>" => INCREMENT_EVENT,
            "<left>" => DECREMENT_EVENT,
            "<r>" => RESET_EVENT
        ])
        .with_mouse(true)
        .with_performance_monitoring(true)
        .with_tick_rate(60.0)
        .with_frame_rate(60.0);

    println!("🚀 Starting optimized counter app with performance monitoring...");
    println!("📊 Try rapid key presses to see the performance metrics!");

    app.run().await?;

    let metrics = app.get_metrics();
    println!("\n📈 Final Performance Report:");
    println!("  Events Processed: {}", metrics.events_processed);
    println!("  Actions Processed: {}", metrics.actions_processed);
    println!("  Average Event Batch: {:.2}", metrics.average_event_batch_size);
    println!("  Average Action Batch: {:.2}", metrics.average_action_batch_size);
    println!("  Total Render Time: {:?}", metrics.total_render_time);
    println!("  Total Event Processing Time: {:?}", metrics.total_event_processing_time);
    println!("  Final FPS: {:.1}", metrics.last_fps);

    Ok(())
}
