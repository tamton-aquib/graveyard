use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table},
};

use crate::app::{App, AppMode};

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(2),
            Constraint::Length(2),
        ])
        .split(f.size());

    render_title_bar(f, app, chunks[0]);
    render_table(f, app, chunks[1]);
    render_status_bar(f, app, chunks[2]);

    if app.show_help {
        render_help(f);
    }
}

fn render_title_bar(f: &mut Frame, app: &mut App, area: Rect) {
    let name = std::path::Path::new(&app.file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&app.file_path);

    let total = app.total_virtual_rows();
    let info = if total >= 100_000 {
        format!("{:.1}K rows", total as f64 / 1000.0)
    } else {
        format!("{} rows", total)
    };

    let text = Line::from(vec![
        Span::raw("  "),
        Span::styled(name, Style::default()
            .fg(Color::Rgb(210, 215, 225))
            .add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::styled(info, Style::default().fg(Color::Rgb(100, 105, 120))),
    ]);

    f.render_widget(
        Paragraph::new(text).style(Style::default().bg(Color::Rgb(24, 26, 33))),
        area,
    );
}

fn render_table(f: &mut Frame, app: &mut App, area: Rect) {
    let table_block = Block::default()
        .borders(Borders::ALL)
        .border_set(symbols::border::ROUNDED)
        .border_style(Style::default().fg(Color::Rgb(65, 68, 82)))
        .title(format!(" {} ", app.handler.sheet_name()))
        .title_alignment(Alignment::Left)
        .title_style(
            Style::default()
                .fg(Color::Rgb(200, 180, 80))
                .add_modifier(Modifier::BOLD),
        );

    let inner = table_block.inner(area);
    f.render_widget(table_block, area);

    let available_rows = inner.height as usize;
    let available_cols = (inner.width / 12).max(1) as usize;

    app.viewport.rows = available_rows;
    app.viewport.cols = available_cols;
    app.adjust_viewport();

    let (start_y, end_y, start_x, end_x) = app
        .viewport
        .visible_range(app.total_virtual_rows(), app.handler.total_cols());

    let mut rows = Vec::new();
    let mut widths = vec![Constraint::Length(7)];
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

    // ── Style constants ──────────────────────────────────────────────

    let cursor_cell = Style::default()
        .bg(Color::Rgb(220, 170, 50))
        .fg(Color::Rgb(10, 10, 10))
        .add_modifier(Modifier::BOLD);

    let header = Style::default()
        .bg(Color::Rgb(35, 47, 66))
        .fg(Color::Rgb(180, 195, 215))
        .add_modifier(Modifier::BOLD);

    let cursor_row = Style::default().bg(Color::Rgb(48, 48, 52));
    let alt_row = Style::default().bg(Color::Rgb(27, 28, 34));
    let normal = Style::default();

    let row_num = Style::default().fg(Color::Rgb(75, 78, 90));
    let row_num_cursor = Style::default()
        .fg(Color::Rgb(200, 205, 215))
        .add_modifier(Modifier::BOLD);

    // ── Build rows ───────────────────────────────────────────────────

    for r in start_y..end_y {
        let mut cells = Vec::new();
        let is_cursor = r == app.cursor_y;

        // Row number cell with continuous cursor row background
        let rn_style = if is_cursor { row_num_cursor.patch(cursor_row) } else { row_num };
        cells.push(
            Cell::from((app.actual_row(r) + 1).to_string()).style(rn_style),
        );

        for (i, c) in (start_x..end_x).enumerate() {
            let val = app.get_cell_value(r, c);
            let trunc_len = render_widths[i].saturating_sub(1);
            let display = if val.chars().count() > trunc_len {
                if trunc_len > 3 {
                    let mut s = val.chars().take(trunc_len - 3).collect::<String>();
                    s.push_str("...");
                    s
                } else {
                    val.chars().take(trunc_len).collect::<String>()
                }
            } else {
                val.into_owned()
            };

            // Priority: cursor cell > header > cursor row > zebra
            let style = if r == app.cursor_y && c == app.cursor_x {
                cursor_cell
            } else if r == 0 {
                header
            } else if is_cursor {
                cursor_row
            } else if r % 2 == 0 {
                alt_row
            } else {
                normal
            };

            cells.push(Cell::from(display).style(style));
        }

        rows.push(Row::new(cells).height(1));
    }

    f.render_widget(Table::new(rows, widths).column_spacing(1), inner);
}

fn render_status_bar(f: &mut Frame, app: &mut App, area: Rect) {
    let (mode_label, mode_style) = match app.mode {
        AppMode::Normal => (
            " NORMAL ",
            Style::default()
                .bg(Color::Rgb(42, 90, 128))
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        AppMode::Edit => (
            " EDIT ",
            Style::default()
                .bg(Color::Rgb(42, 128, 64))
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        AppMode::Command => (
            " CMD ",
            Style::default()
                .bg(Color::Rgb(160, 100, 40))
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        AppMode::Search => (
            " SEARCH ",
            Style::default()
                .bg(Color::Rgb(100, 60, 130))
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    };

    let dim = Color::Rgb(80, 82, 95);
    let info = Color::Rgb(190, 195, 205);
    let faint = Color::Rgb(100, 103, 115);

    let total = app.total_virtual_rows();
    let total_str = if total >= 100_000 {
        format!("{:.1}K", total as f64 / 1000.0)
    } else {
        total.to_string()
    };

    let filter_str = if app.filtered_rows.is_some() {
        format!(" | {} shown", app.total_virtual_rows())
    } else {
        String::new()
    };

    let input_hint = match app.mode {
        AppMode::Edit => format!(" {}_", app.edit_input),
        AppMode::Command => format!(" {}", app.command_input),
        AppMode::Search => format!(" {}...", app.search_input),
        _ => String::new(),
    };

    let text = Line::from(vec![
        Span::styled(mode_label, mode_style),
        Span::styled("│", dim),
        Span::styled(
            format!(
                " Sheet: {}[{}/{}] ",
                app.handler.sheet_name(),
                app.handler.current_sheet_idx + 1,
                app.handler.sheet_names.len()
            ),
            info,
        ),
        Span::styled("│", dim),
        Span::styled(
            format!(" Cell: ({},{}) ", app.cursor_y + 1, app.cursor_x + 1),
            info,
        ),
        Span::styled("│", dim),
        Span::styled(format!(" {} rows{} ", total_str, filter_str), faint),
        Span::raw(" "),
        Span::styled(input_hint, info),
    ]);

    f.render_widget(
        Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .border_style(Style::default().fg(Color::Rgb(55, 58, 70))),
            )
            .style(Style::default()),
        area,
    );
}

fn render_help(f: &mut Frame) {
    let section = |label: &str| {
        Span::styled(
            format!(" {} ", label),
            Style::default()
                .fg(Color::Rgb(220, 170, 50))
                .add_modifier(Modifier::BOLD),
        )
    };

    let help_text = vec![
        Line::from(Span::styled(
            " Keyboard Shortcuts ",
            Style::default()
                .fg(Color::Rgb(200, 180, 80))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![section("Navigation")]),
        Line::from("  h/j/k/l  or Arrows      Move cursor"),
        Line::from("  g / G                   Top / Bottom"),
        Line::from("  Ctrl-D / Ctrl-U         Page Down / Up"),
        Line::from(""),
        Line::from(vec![section("Editing")]),
        Line::from("  i / Enter               Edit cell"),
        Line::from("  Escape                  Exit edit / clear search"),
        Line::from(""),
        Line::from(vec![section("File & Sheets")]),
        Line::from("  Tab / Shift-Tab         Cycle sheets"),
        Line::from("  :w / Ctrl-S             Save"),
        Line::from("  :q / :wq / q            Quit"),
        Line::from(""),
        Line::from(vec![section("Search")]),
        Line::from("  /                       Search column"),
        Line::from("  :<N>                    Jump to row N"),
    ];

    let block = Block::default()
        .title("  Help  ")
        .title_alignment(Alignment::Center)
        .title_style(
            Style::default()
                .fg(Color::Rgb(200, 180, 80))
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_set(symbols::border::ROUNDED)
        .border_style(Style::default().fg(Color::Rgb(65, 68, 82)))
        .style(
            Style::default()
                .bg(Color::Rgb(18, 19, 25))
                .fg(Color::White),
        );

    f.render_widget(
        Clear,
        centered_rect(50, 60, f.size()),
    );
    f.render_widget(
        Paragraph::new(help_text)
            .block(block)
            .alignment(Alignment::Left),
        centered_rect(50, 60, f.size()),
    );
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
