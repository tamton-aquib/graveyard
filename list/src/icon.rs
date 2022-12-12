use colored::Colorize;
use std::path::Path;

pub fn get_fileicon(filename: &Path) -> String {
    if filename.is_dir() {
        return "".truecolor(86, 156, 214).to_string();
    }

    match filename.extension() {
        Some(e) => match e.to_str().unwrap() {
            "lua" => "".blue().to_string(),
            "md" => "".blue().to_string(),
            "py" => "".blue().to_string(),
            "cpp" => "".bright_blue().to_string(),
            "c" => "".bright_blue().to_string(),
            "dart" => "".blue().to_string(),
            "toml" | "conf" | "yml" | "ini" | "sh" => "".white().to_string(),
            "rs" => "".truecolor(211, 158, 129).to_string(),

            "html" => "".truecolor(216, 76, 40).to_string(),
            "css" => "".truecolor(109, 145, 242).to_string(),
            "java" => "".truecolor(218, 106, 3).to_string(),
            "json" => "".truecolor(245, 200, 63).to_string(),
            "js" => "".truecolor(232, 213, 82).to_string(),
            "ts" => "ﯤ".truecolor(47, 114, 188).to_string(),
            "jsx" | "tsx" => "".bright_blue().to_string(),
            "svelte" => "".truecolor(247, 60, 0).to_string(),

            "norg" => "".truecolor(72, 120, 190).to_string(),
            "lock" => "".white().to_string(),
            "leex" | "ex" | "exs" => "".truecolor(160, 116, 196).to_string(), // _ => "".to_string(),
            _ => "".to_string(),
        },
        None => match filename.file_name().unwrap().to_str().unwrap() {
            "Makefile" => "".yellow().to_string(),
            _ => "".to_string(),
        },
    }
}
