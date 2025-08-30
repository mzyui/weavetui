use ratatui::{layout::Rect, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    event::{Action, Event},
    Component,
};

/// Handle a message for a specific component and its children, recursively.
pub fn handle_draw<T: Component + ?Sized>(c: &mut T, f: &mut Frame<'_>) {
    if let Some(area) = c.area() {
        if c.is_active() {
            c.draw(f, area);

            for child in c.get_children().values_mut() {
                if child.area().is_none() {
                    child.set_area(area);
                }
                handle_draw(child.as_mut(), f);
            }
        }
    }
}

/// Update the component and its childrend recursively, based on a received action.
pub fn update<T: Component + ?Sized>(c: &mut T, action: &Action) {
    if c.is_active() {
        c.update(action);

        for child in c.get_children().values_mut() {
            update(child.as_mut(), action);
        }
    }
}

/// Handle a message for a specific component and its children, recursively.
pub fn handle_message<T: Component + ?Sized>(c: &mut T, message: &str) {
    if c.is_active() {
        c.on_event(message);

        for child in c.get_children().values_mut() {
            handle_message(child.as_mut(), message);
        }
    }
}

/// Initialize a component and its children recursively.
pub fn init<T: Component + ?Sized>(c: &mut T, area: Rect) {
    c.init(area);

    for child in c.get_children().values_mut() {
        init(child.as_mut(), area);
    }
}

/// Set the action handler for a component and its children recursively.
pub fn receive_action_handler<T: Component + ?Sized>(c: &mut T, tx: UnboundedSender<Action>) {
    c.register_action_handler(tx.clone());

    for child in c.get_children().values_mut() {
        receive_action_handler(child.as_mut(), tx.clone());
    }
}

/// handle event for a specific component and its children, recursively.
pub fn handle_event_for<T: Component + ?Sized>(event: &Option<Event>, c: &mut T) -> Vec<Action> {
    if c.is_active() {
        let mut actions = vec![];

        let action = match event {
            Some(Event::Key(key_event)) => c.handle_key_events(*key_event),
            Some(Event::Mouse(mouse_event)) => c.handle_mouse_events(*mouse_event),
            Some(Event::Tick) => c.handle_tick_event(),
            Some(Event::Render) => c.handle_frame_event(),
            Some(Event::Paste(s)) => c.handle_paste_event(s),
            _ => None,
        };

        if let Some(action) = action {
            actions.push(action);
        }

        for child in c.get_children().values_mut() {
            let child_actions = handle_event_for(event, child.as_mut());
            actions.extend(child_actions);
        }

        actions
    } else {
        vec![]
    }
}
