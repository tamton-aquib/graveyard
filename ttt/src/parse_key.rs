use crate::app::App;
use crossterm::event::KeyCode;

pub fn parse(app: &mut App, key: KeyCode) -> Option<()> {
    match key {
        KeyCode::Char('q') => return Some(()),
        KeyCode::Char(c) => {
            if (app.cursor as usize + 1) < app.line.len() {
                if c == app.line.chars().nth(app.cursor as usize).unwrap() {
                    if !app.game_over {
                        app.score += 1;
                    }
                }
                app.cursor += 1
            } else {
                app.game_over = true
            }
        }
        KeyCode::Backspace => {
            if app.cursor > 0 {
                app.cursor -= 1
            }
        }
        _ => {}
    }
    None
}
