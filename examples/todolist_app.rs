use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Paragraph},
};
use weavetui::Component;
use weavetui_core::{ComponentAccessor, app::App, event::Action, kb};
use weavetui_derive::component;

#[component]
pub struct TodoItem {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

impl TodoItem {
    pub fn new(id: usize, description: String, completed: bool) -> Self {
        Self {
            id,
            description,
            completed,
            ..Default::default()
        }
    }
}

impl Component for TodoItem {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let status = if self.completed { "[x]" } else { "[ ]" };
        let text = format!("{} {}", status, self.description);
        let style = if self.completed {
            Style::default().fg(Color::DarkGray).italic()
        } else {
            Style::default().fg(Color::White)
        };

        let paragraph = Paragraph::new(text).style(style).alignment(Alignment::Left);

        f.render_widget(paragraph, area);
    }

    fn on_event(&mut self, message: &str) {
        match message {
            "toggle" => {
                self.completed = !self.completed;
                // Send an action to notify parent about the change, if needed
                self.send_action(Action::AppAction(format!("todo:toggled:{}", self.id)));
            }
            "delete" => {
                // Send an action to notify parent to delete this item
                self.send_action(Action::AppAction(format!("todo:delete:{}", self.id)));
            }
            _ => {}
        }
    }
}

#[component]
pub struct TodoApp {
    pub next_id: usize,
    pub selected_todo_id: Option<usize>,
}

impl TodoApp {
    pub fn new() -> Self {
        let mut app = Self {
            next_id: 0,
            selected_todo_id: None,
            ..Default::default()
        };
        app.add_todo("Learn WeaveTUI".to_string());
        app.add_todo("Build a cool app".to_string());
        app.add_todo("Deploy to production".to_string());
        app.selected_todo_id = app
            .children
            .keys()
            .next()
            .and_then(|s| s.parse::<usize>().ok()); // Select first item
        app
    }

    fn add_todo(&mut self, description: String) {
        let id = self.next_id;
        let todo_item = TodoItem::new(id, description, false).as_active();
        self.children.insert(id.to_string(), Box::new(todo_item));
        self.next_id += 1;
        if self.selected_todo_id.is_none() {
            self.selected_todo_id = Some(id);
        }
    }

    fn delete_todo(&mut self, id: usize) {
        self.children.remove(&id.to_string());
        // Adjust selection if the deleted item was selected
        if self.selected_todo_id == Some(id) {
            self.selected_todo_id = self
                .children
                .keys()
                .next()
                .and_then(|s| s.parse::<usize>().ok());
        }
    }

    fn toggle_todo(&mut self, id: usize) {
        if let Some(todo_item) = self.children.get_mut(&id.to_string()) {
            if let Some(item) = todo_item.downcast_mut::<TodoItem>() {
                item.completed = !item.completed;
            }
        }
    }

    fn select_next_todo(&mut self) {
        let current_id = self.selected_todo_id.unwrap_or(0);
        let mut sorted_ids: Vec<usize> = self
            .children
            .keys()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();
        sorted_ids.sort_unstable();

        if let Some(index) = sorted_ids.iter().position(|&id| id == current_id) {
            if index + 1 < sorted_ids.len() {
                self.selected_todo_id = Some(sorted_ids[index + 1]);
            } else {
                self.selected_todo_id = Some(sorted_ids[0]); // Wrap around
            }
        } else if !sorted_ids.is_empty() {
            self.selected_todo_id = Some(sorted_ids[0]); // Select first if nothing was selected
        } else {
            self.selected_todo_id = None;
        }
    }

    fn select_prev_todo(&mut self) {
        let current_id = self.selected_todo_id.unwrap_or(0);
        let mut sorted_ids: Vec<usize> = self
            .children
            .keys()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();
        sorted_ids.sort_unstable();

        if let Some(index) = sorted_ids.iter().position(|&id| id == current_id) {
            if index > 0 {
                self.selected_todo_id = Some(sorted_ids[index - 1]);
            } else {
                self.selected_todo_id = Some(sorted_ids[sorted_ids.len() - 1]); // Wrap around
            }
        } else if !sorted_ids.is_empty() {
            self.selected_todo_id = Some(sorted_ids[0]); // Select first if nothing was selected
        } else {
            self.selected_todo_id = None;
        }
    }
}

impl Component for TodoApp {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        let block = Block::bordered()
            .title(" WeaveTUI Todolist ")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        f.render_widget(&block, area);

        let inner_area = block.inner(area);
        let num_todos = self.children.len();

        if num_todos == 0 {
            let paragraph = Paragraph::new("No todos yet! Press 'a' to add one.")
                .alignment(Alignment::Center)
                .fg(Color::DarkGray);
            f.render_widget(paragraph, inner_area);
            return;
        }

        let constraints: Vec<Constraint> = (0..num_todos).map(|_| Constraint::Length(1)).collect();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_area);

        let mut sorted_children: Vec<(&String, &mut Box<dyn Component>)> =
            self.children.iter_mut().collect();
        sorted_children.sort_by_key(|(key, _)| key.parse::<usize>().unwrap_or(0));

        for (i, (id_str, child_component)) in sorted_children.into_iter().enumerate() {
            let chunk = chunks[i];
            let id = id_str.parse::<usize>().unwrap_or(0);

            if let Some(todo_item) = child_component.downcast_mut::<TodoItem>() {
                let mut style = Style::default();
                if self.selected_todo_id == Some(id) {
                    style = style.bg(Color::DarkGray);
                }
                let paragraph = Paragraph::new(format!(
                    "{} {}",
                    if todo_item.completed { "[x]" } else { "[ ]" },
                    todo_item.description
                ))
                .style(style)
                .alignment(Alignment::Left);
                f.render_widget(paragraph, chunk);
            }
        }
    }

    fn on_event(&mut self, message: &str) {
        let processed_message = message.trim_start_matches("app:"); // Remove "app:" prefix

        if processed_message.starts_with("todo:toggled:") {
            let _id_str = processed_message.trim_start_matches("todo:toggled:");
            // This event is just for notification, the toggle already happened in TodoItem
        } else if processed_message.starts_with("todo:delete:") {
            let id_str = processed_message.trim_start_matches("todo:delete:");
            if let Ok(id) = id_str.parse::<usize>() {
                self.delete_todo(id);
            }
        } else {
            match processed_message {
                "add_todo" => {
                    self.add_todo(format!("New Todo #{}", self.next_id));
                }
                "toggle_selected_todo" => {
                    if let Some(id) = self.selected_todo_id {
                        self.toggle_todo(id);
                    }
                }
                "delete_selected_todo" => {
                    if let Some(id) = self.selected_todo_id {
                        self.delete_todo(id);
                    }
                }
                "select_next_todo" => self.select_next_todo(),
                "select_prev_todo" => self.select_prev_todo(),
                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let todo_app = TodoApp::new().as_active();

    let mut app = App::default()
        .with_components(vec![Box::new(todo_app)])
        .with_keybindings(kb![
            "<ctrl-c>" => Action::Quit,
            "a" => "app:add_todo",
            " " => "app:toggle_selected_todo",
            "d" => "app:delete_selected_todo",
            "j" => "app:select_next_todo",
            "<down>" => "app:select_next_todo",
            "k" => "app:select_prev_todo",
            "<up>" => "app:select_prev_todo"
        ]);

    app.run().await?;

    dbg!(app);

    Ok(())
}
