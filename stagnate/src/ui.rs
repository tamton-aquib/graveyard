use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table},
};

use crate::app::{App, AppMode};

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(2), Constraint::Length(3)].as_ref())
        .split(f.size());

    render_table(f, app, chunks[0]);
    render_status_bar(f, app, chunks[1]);

    if app.show_help {
        render_help(f);
    }
}

fn render_table(f: &mut Frame, app: &mut App, area: Rect) {
    let available_rows = area.height as usize;
    let available_cols = (area.width / 12) as usize; // Approximation to bootstrap viewport bounds

    app.viewport.rows = available_rows;
    if available_cols > 0 {
        app.viewport.cols = available_cols;
    }

    app.adjust_viewport();

    let (start_y, end_y, start_x, end_x) = app
        .viewport
        .visible_range(app.total_virtual_rows(), app.handler.total_cols());

    let mut rows = Vec::new();
    let mut widths = vec![Constraint::Length(6)];

    let mut render_widths = Vec::new();
    for c in start_x..end_x {
        let mut max_len = 8;
        for r in start_y..end_y {
            let val_len = app.get_cell_value(r, c).chars().count();
            max_len = max_len.max(val_len + 1);
        }
        max_len = max_len.min(50);
        render_widths.push(max_len);
        widths.push(Constraint::Length(max_len as u16));
    }

    for r in start_y..end_y {
        let mut cells = Vec::new();
        cells.push(
            Cell::from((app.actual_row(r) + 1).to_string())
                .style(Style::default().fg(Color::DarkGray)),
        );

        for (i, c) in (start_x..end_x).enumerate() {
            let mut val = app.get_cell_value(r, c);
            let trunc_len = render_widths[i].saturating_sub(1);
            if val.chars().count() > trunc_len {
                if trunc_len > 3 {
                    val = val.chars().take(trunc_len - 3).collect();
                    val.push_str("...");
                } else {
                    val = val.chars().take(trunc_len).collect();
                }
            }

            let mut style = Style::default();
            if r == app.cursor_y && c == app.cursor_x {
                style = style
                    .bg(Color::Cyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD);
            } else if r == 0 {
                style = style
                    .add_modifier(Modifier::REVERSED)
                    .add_modifier(Modifier::BOLD);
            }

            cells.push(Cell::from(val).style(style));
        }

        rows.push(Row::new(cells).height(1));
    }

    let table = Table::new(rows, widths).column_spacing(1);

    f.render_widget(table, area);
}

fn render_status_bar(f: &mut Frame, app: &mut App, area: Rect) {
    let mode_str = match app.mode {
        AppMode::Normal => format!(
            "NORMAL | Sheet: {} [{}/{}] | Cell: ({}, {})",
            app.handler.sheet_name(),
            app.handler.current_sheet_idx + 1,
            app.handler.sheet_names.len(),
            app.cursor_y + 1,
            app.cursor_x + 1
        ),
        AppMode::Edit => format!("EDIT: {}_", app.edit_input),
        AppMode::Command => format!("COMMAND: :{}", app.command_input),
        AppMode::Search => format!("FILTER: /{}...", app.search_input),
    };

    let p = Paragraph::new(Line::from(vec![Span::styled(
        mode_str,
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )]))
    .block(Block::default().borders(Borders::ALL).title(
        ratatui::widgets::block::Title::from(" 🦀 ").alignment(ratatui::layout::Alignment::Right),
    ));

    f.render_widget(p, area);
}

fn render_help(f: &mut Frame) {
    let help_text = vec![
        Line::from(Span::styled(
            " Keyboard Shortcuts ",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("h, j, k, l / Arrows : Navigate cells"),
        Line::from("Tab / Shift-Tab     : Cycle through sheets"),
        Line::from("i / Enter           : Edit current cell"),
        Line::from("Ctrl-D / Ctrl-U     : Page Down / Page Up"),
        Line::from("g / G               : Top / Bottom of sheet"),
        Line::from("/                   : Filter current column (Stackable)"),
        Line::from("Esc                 : Clear filters / Exit modes"),
        Line::from(":<row_number>       : Jump to row"),
        Line::from(":w / Ctrl-S         : Save edits to disk"),
        Line::from(":q / :wq            : Quit"),
        Line::from("?                   : Toggle this help window"),
    ];

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(Color::White));

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);

    let area = centered_rect(60, 60, f.size());
    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
