use crate::excel_handler::ExcelHandler;
use std::collections::HashMap;
use std::time::Instant;
use regex::Regex;

pub enum AppMode {
    Normal,
    Edit,
    Command,
    Search,
}

pub struct Viewport {
    pub rows: usize,
    pub cols: usize,
    pub scroll_y: usize,
    pub scroll_x: usize,
}

impl Viewport {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            scroll_y: 0,
            scroll_x: 0,
        }
    }

    pub fn visible_range(&self, total_rows: usize, total_cols: usize) -> (usize, usize, usize, usize) {
        let start_y = self.scroll_y;
        let end_y = (start_y + self.rows).min(total_rows);
        let start_x = self.scroll_x;
        let end_x = (start_x + self.cols).min(total_cols);
        (start_y, end_y, start_x, end_x)
    }
}

pub struct App {
    pub mode: AppMode,
    pub file_path: String,
    pub handler: ExcelHandler,
    
    // Virtual Scroller
    pub viewport: Viewport,
    
    // Cursor position in absolute coordinates
    pub cursor_y: usize,
    pub cursor_x: usize,

    // sparse changes map
    pub cell_edits: HashMap<String, HashMap<(usize, usize), String>>,

    pub edit_input: String,
    pub command_input: String,
    pub last_key_was_g: bool,
    pub should_quit: bool,
    pub last_click_time: Option<Instant>,
    pub last_click_pos: Option<(u16, u16)>,
    pub search_input: String,
    pub filtered_rows: Option<Vec<usize>>,
    pub show_help: bool,
}

impl App {
    pub fn new(file_path: String) -> Self {
        Self {
            mode: AppMode::Normal,
            handler: ExcelHandler::new(&file_path),
            file_path,
            viewport: Viewport::new(40, 10), // Default terminal size, dynamically updated in ui.rs
            cursor_y: 0,
            cursor_x: 0,
            cell_edits: HashMap::new(),
            edit_input: String::new(),
            command_input: String::new(),
            last_key_was_g: false,
            should_quit: false,
            last_click_time: None,
            last_click_pos: None,
            search_input: String::new(),
            filtered_rows: None,
            show_help: false,
        }
    }

    pub fn handle_click(&mut self, mx: u16, my: u16) {
        if !matches!(self.mode, AppMode::Normal) {
            return;
        }

        let clicked_row = self.viewport.scroll_y + (my as usize);
        if clicked_row < self.total_virtual_rows() && (my as usize) < self.viewport.rows {
            self.cursor_y = clicked_row;
        }

        if mx >= 7 {
            let offset_x = (mx - 7) as usize;
            let clicked_col = self.viewport.scroll_x + (offset_x / 16);
            if clicked_col < self.handler.total_cols() && clicked_col < self.viewport.scroll_x + self.viewport.cols {
                self.cursor_x = clicked_col;
            }
        } else {
            self.cursor_x = 0;
        }
        self.adjust_viewport();

        // Check for double click
        let now = Instant::now();
        if let Some(time) = self.last_click_time {
            if let Some(pos) = self.last_click_pos {
                if pos == (mx, my) && now.duration_since(time).as_millis() < 500 {
                    // It's a double click!
                    self.enter_edit_mode();
                    self.last_click_time = None;
                    self.last_click_pos = None;
                    return;
                }
            }
        }
        self.last_click_time = Some(now);
        self.last_click_pos = Some((mx, my));
    }

    pub fn enter_edit_mode(&mut self) {
        self.mode = AppMode::Edit;
        let cell_val = self.get_cell_value(self.cursor_y, self.cursor_x);
        self.edit_input = cell_val;
    }

    pub fn exit_edit_mode(&mut self) {
        self.mode = AppMode::Normal;
        self.edit_input.clear();
    }

    pub fn save_edit(&mut self) {
        let raw_r = self.actual_row(self.cursor_y);
        self.cell_edits
            .entry(self.handler.sheet_name())
            .or_default()
            .insert((raw_r, self.cursor_x), self.edit_input.clone());
    }

    pub fn enter_command_mode(&mut self) {
        self.mode = AppMode::Command;
        self.command_input.clear();
    }

    pub fn execute_command(&mut self) {
        let cmd = self.command_input.trim();
        if cmd == "w" {
            // Save logic here
            self.handler.save(&self.file_path, &self.cell_edits);
        } else if cmd == "q" || cmd == "quit" {
            self.should_quit = true;
        } else if cmd == "wq" {
            self.handler.save(&self.file_path, &self.cell_edits);
            self.should_quit = true;
        } else if let Ok(row_num) = cmd.parse::<usize>() {
            // Jump to row
            if row_num > 0 {
                let target = row_num - 1; // 1-indexed to 0-indexed
                let total = self.total_virtual_rows();
                self.cursor_y = target.min(total.saturating_sub(1));
                self.adjust_viewport();
            }
        }
        self.mode = AppMode::Normal;
    }

    pub fn enter_search_mode(&mut self) {
        self.mode = AppMode::Search;
        self.search_input.clear();
    }

    pub fn execute_search(&mut self) {
        self.mode = AppMode::Normal;
        let query = self.search_input.trim();
        
        if query.is_empty() {
            return;
        }

        let regex_opt = Regex::new(&format!("(?i){}", query)).ok();
        let literal_query = query.to_lowercase();

        let mut matches = Vec::new();
        let total = self.handler.total_rows();
        if total > 0 {
            matches.push(0); // keep header
        }

        let c = self.cursor_x;

        let rows_to_search: Box<dyn Iterator<Item = usize>> = if let Some(filtered) = &self.filtered_rows {
            Box::new(filtered.iter().copied().skip(1).collect::<Vec<usize>>().into_iter()) // Skip header 0
        } else {
            Box::new(1..total)
        };

        for r in rows_to_search {
            let val = self.get_raw_cell_value(r, c);
            let is_match = if let Some(re) = &regex_opt {
                re.is_match(&val)
            } else {
                val.to_lowercase().contains(&literal_query)
            };
            if is_match {
                matches.push(r);
            }
        }
        self.filtered_rows = Some(matches);
        self.cursor_y = 0;
        self.adjust_viewport();
    }

    pub fn clear_search(&mut self) {
        self.mode = AppMode::Normal;
        self.search_input.clear();
        self.filtered_rows = None;
        self.adjust_viewport();
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn total_virtual_rows(&self) -> usize {
        if let Some(filtered) = &self.filtered_rows {
            filtered.len()
        } else {
            self.handler.total_rows()
        }
    }

    pub fn actual_row(&self, visual_r: usize) -> usize {
        if let Some(filtered) = &self.filtered_rows {
            *filtered.get(visual_r).unwrap_or(&visual_r)
        } else {
            visual_r
        }
    }

    pub fn get_raw_cell_value(&self, raw_r: usize, c: usize) -> String {
        if let Some(sheet_edits) = self.cell_edits.get(&self.handler.sheet_name()) {
            if let Some(val) = sheet_edits.get(&(raw_r, c)) {
                return val.clone();
            }
        }
        self.handler.get_cell(raw_r, c)
    }

    pub fn get_cell_value(&self, visual_r: usize, c: usize) -> String {
        let raw_r = self.actual_row(visual_r);
        self.get_raw_cell_value(raw_r, c)
    }

    pub fn next_sheet(&mut self) {
        let mut idx = self.handler.current_sheet_idx + 1;
        if idx >= self.handler.sheet_names.len() {
            idx = 0;
        }
        self.handler.load_sheet(idx);
        self.filtered_rows = None;
        self.cursor_y = 0;
        self.cursor_x = 0;
        self.adjust_viewport();
    }

    pub fn prev_sheet(&mut self) {
        let len = self.handler.sheet_names.len();
        if len == 0 { return; }
        
        let mut idx = self.handler.current_sheet_idx;
        if idx == 0 {
            idx = len - 1;
        } else {
            idx -= 1;
        }
        self.handler.load_sheet(idx);
        self.filtered_rows = None;
        self.cursor_y = 0;
        self.cursor_x = 0;
        self.adjust_viewport();
    }

    pub fn move_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
        }
        self.adjust_viewport();
    }

    pub fn move_down(&mut self) {
        if self.cursor_y < self.total_virtual_rows().saturating_sub(1) {
            self.cursor_y += 1;
        }
        self.adjust_viewport();
    }

    pub fn move_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
        self.adjust_viewport();
    }

    pub fn move_right(&mut self) {
        if self.cursor_x < self.handler.total_cols() - 1 {
            self.cursor_x += 1;
        }
        self.adjust_viewport();
    }

    pub fn move_to_top(&mut self) {
        self.cursor_y = 0;
        self.adjust_viewport();
    }

    pub fn page_up(&mut self) {
        let rows = self.viewport.rows;
        if self.cursor_y >= rows {
            self.cursor_y -= rows;
        } else {
            self.cursor_y = 0;
        }
        self.adjust_viewport();
    }

    pub fn page_down(&mut self) {
        let rows = self.viewport.rows;
        let total_rows = self.total_virtual_rows();
        self.cursor_y += rows;
        if self.cursor_y >= total_rows && total_rows > 0 {
            self.cursor_y = total_rows - 1;
        }
        self.adjust_viewport();
    }

    pub fn move_to_bottom(&mut self) {
        let total_rows = self.total_virtual_rows();
        if total_rows > 0 {
            self.cursor_y = total_rows - 1;
        }
        self.adjust_viewport();
    }

    pub fn adjust_viewport(&mut self) {
        // Adjust Y
        if self.cursor_y < self.viewport.scroll_y {
            self.viewport.scroll_y = self.cursor_y;
        } else if self.cursor_y >= self.viewport.scroll_y + self.viewport.rows {
            self.viewport.scroll_y = self.cursor_y - self.viewport.rows + 1;
        }

        // Adjust X
        if self.cursor_x < self.viewport.scroll_x {
            self.viewport.scroll_x = self.cursor_x;
        } else if self.cursor_x >= self.viewport.scroll_x + self.viewport.cols {
            self.viewport.scroll_x = self.cursor_x - self.viewport.cols + 1;
        }
    }
}
