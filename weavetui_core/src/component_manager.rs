//! Component management utilities.

use ratatui::{layout::Rect, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    event::{Action, Event},
    keyboard::KeyBindings,
    theme::ThemeManager,
    Component,
};

/// Draw a component and its children recursively
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

/// Update a component and its children with an action
pub fn update<T: Component + ?Sized>(c: &mut T, action: &Action) {
    if c.is_active() {
        c.update(action);

        for child in c.get_children().values_mut() {
            update(child.as_mut(), action);
        }
    }
}

/// Handle a string message for a component and its children
pub fn handle_message<T: Component + ?Sized>(c: &mut T, message: &str) {
    if c.is_active() {
        c.on_event(message);

        for child in c.get_children().values_mut() {
            handle_message(child.as_mut(), message);
        }
    }
}

/// Initialize a component and its children
pub fn init<T: Component + ?Sized>(c: &mut T, area: Rect) {
    c.init(area);

    for child in c.get_children().values_mut() {
        init(child.as_mut(), area);
    }
}

/// Set action handler for a component and its children
pub fn receive_action_handler<T: Component + ?Sized>(c: &mut T, tx: UnboundedSender<Action>) {
    c.register_action_handler(tx.clone());

    for child in c.get_children().values_mut() {
        receive_action_handler(child.as_mut(), tx.clone());
    }
}

/// Handle events for a component and collect resulting actions
pub fn handle_event_for<T: Component + ?Sized>(c: &mut T, event: &Option<Event>) -> Vec<Action> {
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
            let child_actions = handle_event_for(child.as_mut(), event);
            actions.extend(child_actions);
        }

        actions
    } else {
        vec![]
    }
}

/// Collect keybindings from a component and its children
pub fn custom_keybindings<T: Component + ?Sized>(c: &mut T, kb: &mut KeyBindings) {
    let other_kb = c.keybindings();
    kb.extend(other_kb);

    for child in c.get_children().values_mut() {
        custom_keybindings(child.as_mut(), kb);
    }
}

/// Set theme for a component and its children
pub fn handle_theme<T: Component + ?Sized>(c: &mut T, th: &ThemeManager) {
    c.set_theme_manager(th.clone());

    for child in c.get_children().values_mut() {
        handle_theme(child.as_mut(), th);
    }
}
