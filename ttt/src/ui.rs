use crate::app::App;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

fn get_content(app: &mut App) -> Vec<Span> {
    let before = &app.line[..(app.cursor as usize)];
    let after = &app.line[(app.cursor as usize + 1)..];
    let current = &app.line.chars().nth(app.cursor as usize).unwrap();
    let nice = current.to_string();
    vec![
        Span::styled(before, Style::default().fg(Color::Blue)),
        Span::styled(nice, Style::default().add_modifier(Modifier::BOLD)),
        Span::styled(after, Style::default().fg(Color::Green)),
    ]
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let block = Block::default();
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(size);

    let heading = Paragraph::new(Spans::from(Span::styled(
        "T_T",
        Style::default().fg(Color::Red),
    )))
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });
    f.render_widget(heading, chunks[0]);

    let content = Paragraph::new(Spans::from(get_content(app)))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(content, chunks[1]);

    let score = Paragraph::new(format!("Score: {}\nTime: {}", app.score, app.elapsed))
        .block(Block::default())
        .alignment(Alignment::Center);
    f.render_widget(score, chunks[2]);
}
