use quick_xml::events::Event;
use quick_xml::Reader as XmlReader;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use zip::ZipArchive;

type SparseCells = HashMap<usize, HashMap<usize, String>>;

pub struct ExcelHandler {
    pub sheet_names: Vec<String>,
    pub current_sheet_idx: usize,
    pub file_path: String,
    sheet_cache: HashMap<String, SparseCells>,
    sheet_dims: HashMap<String, (usize, usize)>,
}

impl ExcelHandler {
    pub fn new(file_path: &str) -> Self {
        let file = match File::open(file_path) {
            Ok(f) => f,
            Err(_) => return Self::empty(file_path),
        };
        let mut archive = match ZipArchive::new(file) {
            Ok(a) => a,
            Err(_) => return Self::empty(file_path),
        };

        let sheet_names = parse_sheet_names(&mut archive).unwrap_or_default();
        let mut sheet_cache = HashMap::new();
        let mut sheet_dims = HashMap::new();

        if let Some(name) = sheet_names.first().cloned() {
            let strings = parse_shared_strings(&mut archive);
            if let Some((cells, dims)) = load_worksheet(&mut archive, 0, &strings) {
                sheet_cache.insert(name, cells);
                sheet_dims.insert(sheet_names[0].clone(), dims);
            }
        }

        Self {
            sheet_names,
            current_sheet_idx: 0,
            file_path: file_path.to_string(),
            sheet_cache,
            sheet_dims,
        }
    }

    fn empty(file_path: &str) -> Self {
        Self {
            sheet_names: Vec::new(),
            current_sheet_idx: 0,
            file_path: file_path.to_string(),
            sheet_cache: HashMap::new(),
            sheet_dims: HashMap::new(),
        }
    }

    pub fn load_sheet(&mut self, idx: usize) {
        if idx >= self.sheet_names.len() {
            return;
        }
        self.current_sheet_idx = idx;
        let name = self.sheet_names[idx].clone();
        if self.sheet_cache.contains_key(&name) {
            return;
        }

        let Ok(file) = File::open(&self.file_path) else { return };
        let Ok(mut archive) = ZipArchive::new(file) else { return };
        let strings = parse_shared_strings(&mut archive);
        if let Some((cells, dims)) = load_worksheet(&mut archive, idx, &strings) {
            self.sheet_cache.insert(name, cells);
            self.sheet_dims.insert(self.sheet_names[idx].clone(), dims);
        }
    }

    pub fn sheet_name(&self) -> String {
        self.sheet_names
            .get(self.current_sheet_idx)
            .cloned()
            .unwrap_or_default()
    }

    pub fn total_rows(&self) -> usize {
        let name = self.sheet_name();
        self.sheet_dims.get(&name).map(|d| d.0).unwrap_or(0)
    }

    pub fn total_cols(&self) -> usize {
        let name = self.sheet_name();
        self.sheet_dims.get(&name).map(|d| d.1).unwrap_or(0)
    }

    pub fn get_cell(&self, r: usize, c: usize) -> Cow<'_, str> {
        let name = self.sheet_name();
        if let Some(sheet) = self.sheet_cache.get(&name) {
            if let Some(row) = sheet.get(&r) {
                if let Some(val) = row.get(&c) {
                    return Cow::Borrowed(val.as_str());
                }
            }
        }
        Cow::Borrowed("")
    }

    pub fn save(
        &self,
        file_path: &str,
        all_edits: &HashMap<String, HashMap<(usize, usize), String>>,
    ) {
        let path = std::path::Path::new(file_path);
        let mut book = match umya_spreadsheet::reader::xlsx::read(path) {
            Ok(b) => b,
            Err(_) => return,
        };

        for (sheet_name, edits) in all_edits {
            let sheet = match book.get_sheet_by_name_mut(sheet_name) {
                Some(s) => s,
                None => continue,
            };

            for (&(r, c), val) in edits {
                // umya-spreadsheet interprets (u32, u32) as (column, row)
                sheet
                    .get_cell_mut((c as u32 + 1, r as u32 + 1))
                    .set_value_string(val);
            }
        }

        let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
    }
}

// ── Parsing helpers ──────────────────────────────────────────────────────

fn parse_sheet_names(archive: &mut ZipArchive<File>) -> Option<Vec<String>> {
    let entry = archive.by_name("xl/workbook.xml").ok()?;
    let mut reader = XmlReader::from_reader(BufReader::new(entry));
    reader.config_mut().trim_text(true);

    let mut names = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e))
                if e.name().as_ref() == b"sheet" =>
            {
                if let Some(name) = attr_val(e, b"name") {
                    names.push(name);
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
        buf.clear();
    }

    Some(names)
}

fn parse_shared_strings(archive: &mut ZipArchive<File>) -> Vec<String> {
    let Ok(entry) = archive.by_name("xl/sharedStrings.xml") else {
        return vec![];
    };

    let mut reader = XmlReader::from_reader(BufReader::new(entry));
    reader.config_mut().trim_text(true);

    let mut strings = Vec::new();
    let mut buf = Vec::new();
    let mut in_si = false;
    let mut in_t = false;
    let mut text = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name().as_ref() {
                b"si" => {
                    in_si = true;
                    text.clear();
                }
                b"t" if in_si => in_t = true,
                _ => {}
            },
            Ok(Event::Text(ref e)) if in_t => {
                text.push_str(std::str::from_utf8(e.as_ref()).unwrap_or(""));
            }
            Ok(Event::End(ref e)) => match e.name().as_ref() {
                b"si" => {
                    if in_si {
                        strings.push(std::mem::take(&mut text));
                        in_si = false;
                    }
                }
                b"t" => in_t = false,
                _ => {}
            },
            Ok(Event::Eof) => break,
            _ => {}
        }
        buf.clear();
    }

    strings
}

#[allow(unused_assignments)]
fn load_worksheet(
    archive: &mut ZipArchive<File>,
    idx: usize,
    shared_strings: &[String],
) -> Option<(SparseCells, (usize, usize))> {
    let entry_name = format!("xl/worksheets/sheet{}.xml", idx + 1);
    let entry = archive.by_name(&entry_name).ok()?;

    let mut reader = XmlReader::from_reader(BufReader::new(entry));
    reader.config_mut().trim_text(true);

    let mut sheet: SparseCells = HashMap::new();
    let mut buf = Vec::new();
    let mut max_r: usize = 0;
    let mut max_c: usize = 0;
    let mut dim_end_col: Option<usize> = None;

    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut cell_type = String::new();
    let mut v_text = String::new();
    let mut is_text = String::new();
    let mut in_cell = false;
    let mut in_v = false;
    let mut in_is = false;
    let mut in_t_is = false;

    macro_rules! finish_cell {
        () => {{
            if in_cell {
                let value = if cell_type == "s" {
                    v_text
                        .parse::<usize>()
                        .ok()
                        .and_then(|i| shared_strings.get(i))
                        .cloned()
                        .unwrap_or_default()
                } else if cell_type == "inlineStr" {
                    is_text.clone()
                } else {
                    v_text.clone()
                };

                if !value.is_empty() {
                    sheet.entry(row).or_default().insert(col, value);
                }
                if row > max_r {
                    max_r = row;
                }
                in_cell = false;
                max_c = max_c.max(col);
                in_v = false;
                in_is = false;
                in_t_is = false;
            }
        }};
    }

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) if e.name().as_ref() == b"dimension" => {
                if let Some(r) = attr_val(e, b"ref") {
                    let end = r.split_once(':').map(|(_, e)| e).unwrap_or(&r);
                    if let Some(d) = end.find(|c: char| c.is_ascii_digit()) {
                        let cs = &end[..d];
                        if !cs.is_empty() {
                            dim_end_col = Some(
                                cs.bytes().fold(0, |a, b| a * 26 + (b - b'A' + 1) as usize) - 1
                            );
                        }
                    }
                }
            }
            Ok(Event::Start(ref e)) => match e.name().as_ref() {
                b"row" => {
                    if let Some(r) = attr_val(e, b"r") {
                        row = r.parse::<usize>().unwrap_or(1).saturating_sub(1);
                    }
                }
                b"c" => {
                    finish_cell!();
                    in_cell = true;
                    cell_type = attr_val(e, b"t").unwrap_or_default();
                    col = attr_val(e, b"r")
                        .as_deref()
                        .and_then(col_from_ref)
                        .unwrap_or(0);
                }
                b"v" if in_cell => {
                    in_v = true;
                    v_text.clear();
                }
                b"is" if in_cell => {
                    in_is = true;
                    is_text.clear();
                }
                b"t" if in_is => in_t_is = true,
                _ => {}
            },
            Ok(Event::Empty(ref e)) if e.name().as_ref() == b"c" => {
                finish_cell!();
            }
            Ok(Event::Text(ref e)) => {
                let t = std::str::from_utf8(e.as_ref()).unwrap_or("");
                if in_v {
                    v_text.push_str(t);
                } else if in_t_is {
                    is_text.push_str(t);
                }
            }
            Ok(Event::End(ref e)) => match e.name().as_ref() {
                b"c" => finish_cell!(),
                b"v" => in_v = false,
                b"is" => in_is = false,
                b"t" => in_t_is = false,
                _ => {}
            },
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    let total_cols = dim_end_col.map(|c| c + 1).unwrap_or(max_c + 1);
    Some((sheet, (max_r + 1, total_cols)))
}

fn attr_val(e: &quick_xml::events::BytesStart, name: &[u8]) -> Option<String> {
    for attr in e.attributes().flatten() {
        if attr.key.as_ref() == name {
            return String::from_utf8(attr.value.to_vec()).ok();
        }
    }
    None
}

fn col_from_ref(cell_ref: &str) -> Option<usize> {
    let start = cell_ref.find(|c: char| c.is_ascii_digit())?;
    let s = &cell_ref[..start];
    if s.is_empty() {
        return None;
    }
    Some(s.bytes().fold(0, |acc, b| acc * 26 + (b - b'A' + 1) as usize) - 1)
}
