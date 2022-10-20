use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

static DISPLAY: &str = r#"
This is some example text!
Some more digits here 43254!
"#;

fn render_text(app: &mut App) -> Vec<Span> {
    if let Ok(re) = regex::Regex::new(&app.clone().re_str) {
        if let Some(item) = re.find(DISPLAY) {
            app.before = DISPLAY[..item.start()].to_string();
            app.content = DISPLAY[item.start()..item.end()].to_string();
            app.after = DISPLAY[item.end()..].to_string();
        }
    }
    vec![
        Span::styled(&app.before, Style::default().fg(Color::White)),
        Span::styled(&app.content, Style::default().fg(Color::LightGreen)),
        Span::styled(&app.after, Style::default().fg(Color::White)),
    ]
    // vec![&app.before, &app.content, &app.after]
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let block = Block::default(); // .style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(10),
                Constraint::Percentage(20),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(size);

    let text = vec![Spans::from(Span::styled(
        format!("Query: {}", &app.re_str),
        Style::default().fg(Color::Cyan),
    ))];

    let create_block = |title| {
        Block::default().borders(Borders::ALL).title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
    };

    let paragraph = Paragraph::new(text.clone())
        // .block(create_block("Regex"))
        .block(Block::default())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[1]);

    let paragraph = Paragraph::new(Spans::from(render_text(app)))
        .block(create_block("Match String"))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[3]);
}
