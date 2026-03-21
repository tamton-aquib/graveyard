mod app;
mod ui;
mod excel_handler;

use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <excel_file.xlsx>", args[0]);
        return Ok(());
    }
    let file_path = args[1].clone();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new(file_path);
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        if app.should_quit {
            return Ok(());
        }

        if crossterm::event::poll(std::time::Duration::from_millis(16))? {
            let event = crossterm::event::read()?;
            match event {
                crossterm::event::Event::Key(key) => {
                    if app.show_help {
                        match key.code {
                            crossterm::event::KeyCode::Char('?') | crossterm::event::KeyCode::Esc | crossterm::event::KeyCode::Char('q') => {
                                app.toggle_help();
                            }
                            _ => {}
                        }
                        continue;
                    }

                    match app.mode {
                        app::AppMode::Normal => {
                            let has_control = key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL);
                            if has_control {
                                match key.code {
                                    crossterm::event::KeyCode::Char('d') => app.page_down(),
                                    crossterm::event::KeyCode::Char('u') => app.page_up(),
                                    crossterm::event::KeyCode::Char('s') => {
                                        app.handler.save(&app.file_path, &app.cell_edits);
                                    }
                                    _ => {}
                                }
                            } else {
                                match key.code {
                                    crossterm::event::KeyCode::Esc => app.clear_search(),
                                    crossterm::event::KeyCode::Char('q') => return Ok(()),
                                    crossterm::event::KeyCode::Char('h') | crossterm::event::KeyCode::Left => app.move_left(),
                                    crossterm::event::KeyCode::Char('j') | crossterm::event::KeyCode::Down => app.move_down(),
                                    crossterm::event::KeyCode::Char('k') | crossterm::event::KeyCode::Up => app.move_up(),
                                    crossterm::event::KeyCode::Char('l') | crossterm::event::KeyCode::Right => app.move_right(),
                                    crossterm::event::KeyCode::Tab | crossterm::event::KeyCode::Char(']') => app.next_sheet(),
                                    crossterm::event::KeyCode::BackTab | crossterm::event::KeyCode::Char('[') => app.prev_sheet(),
                                    crossterm::event::KeyCode::Char('i') | crossterm::event::KeyCode::Enter => {
                                        app.enter_edit_mode();
                                    }
                                    crossterm::event::KeyCode::Char(':') => {
                                        app.enter_command_mode();
                                    }
                                    crossterm::event::KeyCode::Char('/') => {
                                        app.enter_search_mode();
                                    }
                                    crossterm::event::KeyCode::Char('?') => {
                                        app.toggle_help();
                                    }
                                    crossterm::event::KeyCode::Char('G') => app.move_to_bottom(),
                                    crossterm::event::KeyCode::Char('g') => {
                                        if app.last_key_was_g {
                                            app.move_to_top();
                                            app.last_key_was_g = false;
                                        } else {
                                            app.last_key_was_g = true;
                                        }
                                    }
                                    _ => {
                                        app.last_key_was_g = false;
                                    }
                                }
                            }
                        }
                        app::AppMode::Edit => {
                            match key.code {
                                crossterm::event::KeyCode::Esc => app.exit_edit_mode(),
                                crossterm::event::KeyCode::Enter => {
                                    app.save_edit();
                                    app.exit_edit_mode();
                                }
                                crossterm::event::KeyCode::Char(c) => {
                                    app.edit_input.push(c);
                                }
                                crossterm::event::KeyCode::Backspace => {
                                    app.edit_input.pop();
                                }
                                _ => {}
                            }
                        }
                        app::AppMode::Command => {
                            match key.code {
                                crossterm::event::KeyCode::Esc => app.mode = app::AppMode::Normal,
                                crossterm::event::KeyCode::Enter => {
                                    app.execute_command();
                                }
                                crossterm::event::KeyCode::Char(c) => {
                                    app.command_input.push(c);
                                }
                                crossterm::event::KeyCode::Backspace => {
                                    app.command_input.pop();
                                }
                                _ => {}
                            }
                        }
                        app::AppMode::Search => {
                            match key.code {
                                crossterm::event::KeyCode::Esc => app.clear_search(),
                                crossterm::event::KeyCode::Enter => {
                                    app.execute_search();
                                }
                                crossterm::event::KeyCode::Char(c) => {
                                    app.search_input.push(c);
                                }
                                crossterm::event::KeyCode::Backspace => {
                                    app.search_input.pop();
                                }
                                _ => {}
                            }
                        }
                    }
                }
                crossterm::event::Event::Mouse(mouse_event) => {
                    match mouse_event.kind {
                        crossterm::event::MouseEventKind::ScrollDown => {
                            app.move_down();
                            app.move_down();
                            app.move_down();
                        }
                        crossterm::event::MouseEventKind::ScrollUp => {
                            app.move_up();
                            app.move_up();
                            app.move_up();
                        }
                        crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
                            app.handle_click(mouse_event.column, mouse_event.row);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
