use std::process::Command;

pub struct Wifi {
    pub name: String,
}
impl Wifi {
    // TODO: another thread?
    pub fn connect(&self, pass: &str) -> bool {
        println!("About to connect to: {}", self.name);
        Command::new("nmcli")
            .args(&["d", "wifi", "connect", &self.name, "password", pass])
            .status()
            .is_ok()
    }
}
