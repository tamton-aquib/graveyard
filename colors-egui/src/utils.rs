use clipboard_ext::prelude::*;

pub fn copy_to_clip(content: &str) {
    clipboard_ext::x11_bin::ClipboardContext::new()
        .unwrap()
        .set_contents(content.to_string())
        .unwrap();
}

pub fn to_hex(v: &[u8]) -> String {
    format!("#{:02x}{:02x}{:02x}", &v[0], &v[1], &v[2])
}
pub fn to_rgb(v: &[u8]) -> String {
    format!("rgb({},{},{})", &v[0], &v[1], &v[2])
}
