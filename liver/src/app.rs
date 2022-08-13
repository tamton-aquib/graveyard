#[derive(Clone)]
pub struct App {
    pub re_str: String,
    pub log: String,
    pub before: String,
    pub content: String,
    pub after: String,
}

impl App {
    pub fn new() -> App {
        App {
            re_str: String::new(),
            log: String::new(),
            before: String::new(),
            content: String::new(),
            after: String::new(),
        }
    }
}
