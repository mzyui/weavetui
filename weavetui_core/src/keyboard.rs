use {
    super::event::{Action, ActionKind},
    crate::kb,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    std::{collections::HashMap, str::FromStr},
};

#[derive(Clone, Debug)]
/// A struct that holds key bindings
///
/// The key bindings are stored in a hashmap where the key is a vector of
/// [`crossterm::event::KeyEvent`] and the value is a
/// [`Action`](crate::tui::Action). This is constructed automatically by [`Kb`](crate::tui::Kb)
/// using a [`str`] to [`Action`](crate::tui::Action) mapping, using special syntax to represent
/// keys and key sequences (see
/// [`parse_key_sequence`](crate::tui::utils::keyboard::parse_key_sequence) and
/// [`Kb`](crate::tui::Kb) for more information).
pub struct KeyBindings(pub HashMap<Vec<KeyEvent>, Action>);

impl KeyBindings {
    pub fn new<const N: usize>(raw: [(&str, impl Into<ActionKind>); N]) -> Self {
        let mut keybindings = HashMap::new();
        for (key_str, cmd) in raw.into_iter() {
            let cmd: ActionKind = cmd.into();

            match cmd {
                ActionKind::Full(action) => {
                    keybindings.insert(parse_key_sequence(key_str).unwrap(), action);
                }

                ActionKind::Stringified(cmd) => {
                    let action = Action::from_str(&cmd);
                    match action {
                        Ok(action) => {
                            keybindings.insert(parse_key_sequence(key_str).unwrap(), action)
                        }
                        Err(_) => keybindings
                            .insert(parse_key_sequence(key_str).unwrap(), Action::AppAction(cmd)),
                    };
                }
            }
        }

        // Check for Action::Quit
        if !keybindings
            .iter()
            .any(|(_, action)| *action == Action::Quit)
        {
            panic!("Action::Quit is not bound to any key. Consider binding it for graceful exit (e.g., <ctrl-c>).\n");
        }

        KeyBindings(keybindings)
    }

    pub fn get(&self, key_events: &[KeyEvent]) -> Option<&Action> {
        self.0.get(key_events)
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self::new(kb![
            "<ctrl-c>" => Action::Quit
        ])
    }
}

/// `@internal`
///
/// Parses a string into a [`KeyEvent`]
fn parse_key_event(raw: &str) -> Result<KeyEvent, std::io::Error> {
    let raw_lower = raw.to_ascii_lowercase();
    let (remaining, modifiers) = extract_modifiers(&raw_lower);
    parse_key_code_with_modifiers(remaining, modifiers)
}

/// `@internal`
///
/// Extracts the modifiers from a string formatted as `modifier-key`
fn extract_modifiers(raw: &str) -> (&str, KeyModifiers) {
    let mut modifiers = KeyModifiers::empty();
    let mut current = raw;

    loop {
        match current {
            rest if rest.starts_with("ctrl-") => {
                modifiers.insert(KeyModifiers::CONTROL);
                current = &rest[5..];
            }
            rest if rest.starts_with("alt-") => {
                modifiers.insert(KeyModifiers::ALT);
                current = &rest[4..];
            }
            rest if rest.starts_with("shift-") => {
                modifiers.insert(KeyModifiers::SHIFT);
                current = &rest[6..];
            }
            _ => break, // break out of the loop if no known prefix is detected
        };
    }

    (current, modifiers)
}

/// `@internal`
///
/// Parses a string into a [`KeyEvent`] with modifiers
fn parse_key_code_with_modifiers(
    raw: &str,
    mut modifiers: KeyModifiers,
) -> Result<KeyEvent, std::io::Error> {
    let c = match raw {
        "esc" => KeyCode::Esc,
        "enter" => KeyCode::Enter,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "home" => KeyCode::Home,
        "end" => KeyCode::End,
        "pageup" => KeyCode::PageUp,
        "pagedown" => KeyCode::PageDown,
        "backtab" => {
            modifiers.insert(KeyModifiers::SHIFT);
            KeyCode::BackTab
        }
        "backspace" => KeyCode::Backspace,
        "delete" => KeyCode::Delete,
        "insert" => KeyCode::Insert,
        "f1" => KeyCode::F(1),
        "f2" => KeyCode::F(2),
        "f3" => KeyCode::F(3),
        "f4" => KeyCode::F(4),
        "f5" => KeyCode::F(5),
        "f6" => KeyCode::F(6),
        "f7" => KeyCode::F(7),
        "f8" => KeyCode::F(8),
        "f9" => KeyCode::F(9),
        "f10" => KeyCode::F(10),
        "f11" => KeyCode::F(11),
        "f12" => KeyCode::F(12),
        "space" => KeyCode::Char(' '),
        "hyphen" => KeyCode::Char('-'),
        "minus" => KeyCode::Char('-'),
        "tab" => KeyCode::Tab,
        c if c.len() == 1 => {
            let mut c = c.chars().next().unwrap();
            if modifiers.contains(KeyModifiers::SHIFT) {
                c = c.to_ascii_uppercase();
            }
            KeyCode::Char(c)
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid key",
            ));
        }
    };
    Ok(KeyEvent::new(c, modifiers))
}

/// Converts a [`KeyEvent`] to a string representation
pub fn key_event_to_string(key_event: &KeyEvent) -> String {
    let char;
    let key_code = match key_event.code {
        KeyCode::Backspace => "backspace",
        KeyCode::Enter => "enter",
        KeyCode::Left => "left",
        KeyCode::Right => "right",
        KeyCode::Up => "up",
        KeyCode::Down => "down",
        KeyCode::Home => "home",
        KeyCode::End => "end",
        KeyCode::PageUp => "pageup",
        KeyCode::PageDown => "pagedown",
        KeyCode::Tab => "tab",
        KeyCode::BackTab => "backtab",
        KeyCode::Delete => "delete",
        KeyCode::Insert => "insert",
        KeyCode::F(c) => {
            char = format!("f({c})");
            &char
        }
        KeyCode::Char(' ') => "space",
        KeyCode::Char(c) => {
            char = c.to_string();
            &char
        }
        KeyCode::Esc => "esc",
        KeyCode::Null => "",
        KeyCode::CapsLock => "",
        KeyCode::Menu => "",
        KeyCode::ScrollLock => "",
        KeyCode::Media(_) => "",
        KeyCode::NumLock => "",
        KeyCode::PrintScreen => "",
        KeyCode::Pause => "",
        KeyCode::KeypadBegin => "",
        KeyCode::Modifier(_) => "",
    };

    let mut modifiers = Vec::with_capacity(3);

    if key_event.modifiers.intersects(KeyModifiers::CONTROL) {
        modifiers.push("ctrl");
    }

    if key_event.modifiers.intersects(KeyModifiers::SHIFT) {
        modifiers.push("shift");
    }

    if key_event.modifiers.intersects(KeyModifiers::ALT) {
        modifiers.push("alt");
    }

    // if the modifiers is "shift" and the key code is a letter, we just return the letter
    // otherwise we return the modifiers joined by a dash and the key code
    if modifiers.len() == 1
        && key_code.chars().count() == 1
        && key_code.chars().all(char::is_alphabetic)
    {
        return key_code.to_string();
    }

    let mut key = modifiers.join("-");

    if !key.is_empty() {
        key.push('-');
    }

    key.push_str(key_code);

    key
}

/// Parses a string into a vector of [`KeyEvent`]
pub fn parse_key_sequence(raw: &str) -> Result<Vec<KeyEvent>, std::io::Error> {
    if raw.chars().filter(|c| *c == '>').count() != raw.chars().filter(|c| *c == '<').count() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid key sequence",
        ));
    }
    let raw = if !raw.contains("><") {
        let raw = raw.strip_prefix('<').unwrap_or(raw);
        raw.strip_prefix('>').unwrap_or(raw)
    } else {
        raw
    };
    let sequences = raw
        .split("><")
        .map(|seq| {
            if let Some(s) = seq.strip_prefix('<') {
                s
            } else if let Some(s) = seq.strip_suffix('>') {
                s
            } else {
                seq
            }
        })
        .collect::<Vec<_>>();

    sequences.into_iter().map(parse_key_event).collect()
}
