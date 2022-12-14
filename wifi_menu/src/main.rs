use eframe::{
    egui::{self, CentralPanel, Vec2},
    epi::App,
    run_native,
};
use std::process::Command;

mod wifi;
use wifi::Wifi;

struct Menu {
    pass: String,
    list: Vec<Wifi>,
}

impl Menu {
    fn new() -> Menu {
        Menu {
            pass: String::new(),
            list: Vec::new(),
        }
    }

    fn get_interfaces(&self) -> Vec<Wifi> {
        let output = Command::new("nmcli")
            .args(&["-f", "SSID", "d", "wifi"])
            .output()
            .unwrap()
            .stdout;

        let lines = String::from_utf8(output).unwrap();

        lines
            .lines()
            .skip(1)
            .map(|line| Wifi {
                name: line.split(":").nth(0).unwrap().trim().to_string(),
            })
            .collect()
    }

    fn refresh_list(&mut self) {
        self.list = self.get_interfaces();
    }
}

impl App for Menu {
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        println!("Setup ready!");
        self.list = self.get_interfaces();
    }

    fn clear_color(&self) -> egui::Rgba {
        egui::Rgba::from_rgb(255f32, 255f32, 255f32)
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    ui.heading("Connect to WIFI!");
                    ui.add_space(10_f32);

                    ui.group(|ui| {
                        for wifi in &self.list {
                            if ui.button(&wifi.name).clicked() {
                                let status = wifi.connect(&self.pass);

                                if status {
                                    println!("Connected to wifi: {}", wifi.name);
                                    self.pass = String::from("");
                                } else {
                                    println!("Something went wrong!");
                                }
                            }
                        }
                    });
                    ui.add_space(30_f32);

                    ui.text_edit_singleline(&mut self.pass).request_focus();
                },
            );

            // ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }

                    if ui.button("refresh").clicked() {
                        self.refresh_list();
                    }
                })
            })
            // })
        });
    }

    fn name(&self) -> &str {
        "wifi_menu"
    }
}

fn main() {
    let opts = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(250_f32, 350_f32)),
        transparent: true,
        ..Default::default()
    };

    run_native(Box::new(Menu::new()), opts);
}
