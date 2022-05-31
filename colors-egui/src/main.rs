mod utils;

use eframe::{
    egui::{self, Layout, RichText},
    emath::{vec2, Align},
    epaint::Color32,
};
use mouse_rs::Mouse;

struct MyApp {
    rgb: [u8; 3],
    hex: String,
    rgb_str: String,
}

fn get_pixels(x: u32, y: u32) -> Result<[u8; 3], Box<dyn std::error::Error>> {
    let screen = x11_screenshot::Screen::open().expect("Couldn't open!");
    let ss = screen.capture().expect("Failed to take screenshot!");

    let pixel = ss.get_pixel(x, y).0;
    Ok(pixel)
}

impl Default for MyApp {
    fn default() -> Self {
        let pos = Mouse::new().get_position().unwrap();
        let pixels = get_pixels(pos.x as u32, pos.y as u32).unwrap();
        Self {
            rgb_str: utils::to_rgb(&pixels),
            rgb: pixels,
            hex: utils::to_hex(&pixels),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let [r, g, b] = self.rgb;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                ui.label(RichText::new("Click to copy!"));
                ui.add_space(15_f32);
                ui.label(
                    RichText::new(" ".repeat(15)).background_color(Color32::from_rgb(r, g, b)),
                );
                ui.add_space(15_f32);
                ui.group(|ui| {
                    if ui.button(&self.hex).clicked() {
                        utils::copy_to_clip(&self.hex);
                        frame.quit();
                    }
                    ui.add_space(10_f32);
                    if ui.button(&self.rgb_str).clicked() {
                        utils::copy_to_clip(&self.rgb_str);
                        frame.quit();
                    }
                })
            })
        });
    }
}

fn main() {
    let opts = eframe::NativeOptions {
        initial_window_size: Some(vec2(120_f32, 150_f32)),
        ..Default::default()
    };
    eframe::run_native("colors", opts, Box::new(|_cc| Box::new(MyApp::default())));
}
