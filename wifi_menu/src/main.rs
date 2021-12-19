// INCOMPLETE
use eframe::{
    egui::{self, CentralPanel, Vec2},
    epi::App,
    run_native, NativeOptions,
};
use std::process::Command;

const TEMP_PASS: &str = "";

struct Menu;
impl App for Menu {
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        println!("Setup ready!");
        println!("This program wont work! Its just a prototype!");
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        let wifis = get_interfaces();

        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    ui.heading("Connect to WIFI!");
                    ui.add_space(10 as f32);

                    ui.group(|ui| {
                        for wifi in &wifis {
                            if ui.button(&wifi.name).clicked() {
                                let status = wifi.connect();

                                if status {
                                    println!("Connected to wifi: {}", wifi.name);
                                } else {
                                    println!("Something went wrong!");
                                }
                            }
                        }
                    });
                },
            );

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                if ui.button("Quit").clicked() {
                    frame.quit();
                }
            })
        });
    }

    fn name(&self) -> &str {
        "Wifi_Menu"
    }
}

struct Wifi {
    name: String,
}
impl Wifi {
    // TODO: another thread?
    fn connect(&self) -> bool {
        println!("About to connect to: {}", self.name);
        Command::new("nmcli")
            .args(&["d", "wifi", "connect", &self.name, "password", TEMP_PASS])
            .status()
            .is_ok()
    }
}

fn get_interfaces() -> Vec<Wifi> {
    let output = Command::new("nmcli")
        .args(&["d", "wifi", "list"])
        .output()
        .unwrap()
        .stdout;

    let lines = String::from_utf8(output).unwrap();
    let vec_lines: Vec<&str> = lines.split("\n").collect();
    let valid_lines = &vec_lines[1..&vec_lines.len() - 1];
    let mut wifi_list = vec![];

    for i in valid_lines {
        let mut current_wifi: Wifi = Wifi {
            name: String::new(),
        };
        let parts: Vec<&str> = i.split_whitespace().collect();
        let length = parts.len();

        match length {
            10 => current_wifi.name = parts[2].to_string(),
            9 => current_wifi.name = parts[1].to_string(),
            _ => panic!("Couldnt get wifi name: {:?}", parts),
        }

        wifi_list.push(current_wifi);
    }
    wifi_list
}

fn main() {
    let app = Menu;
    let mut opts = NativeOptions::default();
    opts.initial_window_size = Some(Vec2::new(250 as f32, 350 as f32));

    run_native(Box::new(app), opts);
}
