use calamine::{Reader, open_workbook_auto, Data, Range};
use rust_xlsxwriter::Workbook;
use std::collections::HashMap;

pub struct ExcelHandler {
    pub sheet_cache: HashMap<String, Range<Data>>,
    pub sheet_names: Vec<String>,
    pub current_sheet_idx: usize,
    pub file_path: String,
}

impl ExcelHandler {
    pub fn new(file_path: &str) -> Self {
        let mut sheet_names = Vec::new();

        if let Ok(mut workbook) = open_workbook_auto(file_path) {
            sheet_names = workbook.sheet_names().to_vec();
        }

        let mut handler = Self {
            sheet_cache: HashMap::new(),
            sheet_names,
            current_sheet_idx: 0,
            file_path: file_path.to_string(),
        };
        handler.load_sheet(0);
        handler
    }

    pub fn load_sheet(&mut self, idx: usize) {
        if idx >= self.sheet_names.len() { return; }
        
        self.current_sheet_idx = idx;
        let sheet_name = self.sheet_names[idx].clone();

        if !self.sheet_cache.contains_key(&sheet_name) {
            if let Ok(mut workbook) = open_workbook_auto(&self.file_path) {
                if let Ok(range) = workbook.worksheet_range(&sheet_name) {
                    self.sheet_cache.insert(sheet_name.clone(), range);
                }
            }
        }
    }

    pub fn sheet_name(&self) -> String {
        self.sheet_names.get(self.current_sheet_idx).cloned().unwrap_or_default()
    }

    pub fn total_rows(&self) -> usize {
        let name = self.sheet_name();
        self.sheet_cache.get(&name).map(|r| r.get_size().0).unwrap_or(0)
    }

    pub fn total_cols(&self) -> usize {
        let name = self.sheet_name();
        self.sheet_cache.get(&name).map(|r| r.get_size().1).unwrap_or(0)
    }

    pub fn get_cell(&self, r: usize, c: usize) -> String {
        let name = self.sheet_name();
        if let Some(range) = self.sheet_cache.get(&name) {
            if let Some(cell) = range.get_value((r as u32, c as u32)) {
                return match cell {
                    Data::String(s) => s.to_string(),
                    Data::Float(f) => f.to_string(),
                    Data::Int(i) => i.to_string(),
                    Data::Bool(b) => b.to_string(),
                    Data::DateTime(d) => d.to_string(),
                    Data::Error(e) => format!("Error: {}", e),
                    Data::Empty => String::new(),
                    Data::DateTimeIso(d) => d.to_string(),
                    Data::DurationIso(d) => d.to_string(),
                };
            }
        }
        String::new()
    }

    pub fn save(&self, file_path: &str, all_edits: &HashMap<String, HashMap<(usize, usize), String>>) {
        let mut workbook_out = Workbook::new();
        
        if let Ok(mut calamine_wb) = open_workbook_auto(file_path) {
            for sheet_name in &self.sheet_names {
                let safe_sheet_name = if sheet_name.len() > 31 {
                    &sheet_name[..31]
                } else {
                    &sheet_name
                };
                let worksheet = workbook_out.add_worksheet().set_name(safe_sheet_name).unwrap();

                let empty_map = HashMap::new();
                let sheet_edits = all_edits.get(sheet_name).unwrap_or(&empty_map);

                let mut total_r = 0;
                let mut total_c = 0;

                if let Ok(range) = calamine_wb.worksheet_range(sheet_name) {
                    total_r = range.get_size().0;
                    total_c = range.get_size().1;
                    
                    for (r, row) in range.rows().enumerate() {
                        for (c, cell) in row.iter().enumerate() {
                            if let Some(edited_val) = sheet_edits.get(&(r, c)) {
                                let _ = worksheet.write_string(r as u32, c as u16, edited_val);
                            } else {
                                match cell {
                                    Data::String(s) => { let _ = worksheet.write_string(r as u32, c as u16, s); },
                                    Data::Float(f) => { let _ = worksheet.write_number(r as u32, c as u16, *f); },
                                    Data::Int(i) => { let _ = worksheet.write_number(r as u32, c as u16, *i as f64); },
                                    Data::Bool(b) => { let _ = worksheet.write_boolean(r as u32, c as u16, *b); },
                                    Data::Empty | _ => {},
                                }
                            }
                        }
                    }
                }

                for (&(r, c), edited_val) in sheet_edits.iter() {
                    if r >= total_r || c >= total_c {
                        let _ = worksheet.write_string(r as u32, c as u16, edited_val);
                    }
                }
            }
        }

        let _ = workbook_out.save(file_path);
    }
}
