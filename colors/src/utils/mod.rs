// use terminal_clipboard;
use clipboard_ext::prelude::*;

// use chrono::{DateTime, Utc};
use std::fs;
use std::process::Command;

// TODO: copy to clipboard
pub fn copy_to_clip(content: &str) {
    // println!("Copied: {}", terminal_clipboard::get_string().unwrap());
    // terminal_clipboard::set_string(content).expect("Couldnt Copy to Clipboard!");
    clipboard_ext::x11_bin::ClipboardContext::new()
        .unwrap()
        .set_contents(content.to_string())
        .unwrap();
}
pub fn to_hex(v: &Vec<u8>) -> String {
    format!("#{:02x}{:02x}{:02x}", &v[0], &v[1], &v[2])
}
pub fn to_rgb(v: &Vec<u8>) -> String {
    format!("rgb({},{},{})", &v[0], &v[1], &v[2])
}

pub fn delete_screenshot(file: String) {
    fs::remove_file(file).expect("Couldnt delete temp screenshot!");
}

pub fn screenshot() -> String {
    // let now: DateTime<Utc> = Utc::now();
    // let filename = now.format("screenshot%d_%m_%Y__%H_%M_%S.png").to_string();
    let filename = String::from("./screenshot.png");

    Command::new("scrot")
        .args(&["-z", &filename])
        .output()
        .expect("Couldnt get screenshot!");

    filename.to_string()
}
