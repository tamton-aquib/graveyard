#[derive(Debug, Clone)]
pub struct App {
    pub line: String,
    pub cursor: u8,
    pub score: u8,
    pub elapsed: u8,
    pub game_over: bool,
}

impl App {
    pub fn new() -> App {
        App {
            line: "This is an example line for demonstation".to_string(),
            cursor: 0,
            score: 0,
            elapsed: 0,
            game_over: false,
        }
    }

    pub fn on_tick(&mut self) {
        if !self.game_over {
            self.elapsed += 1
        }
    }
}
