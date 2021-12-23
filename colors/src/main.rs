use fltk::{app, button::Button, frame::Frame, group::Pack, prelude::*, window::Window};
use image;
use mouse_rs::Mouse;
mod utils;

struct Colors {
    hex: String,
    rgb: String,
}
impl Colors {
    fn new(hex: String, rgb: String) -> Colors {
        Colors { hex, rgb }
    }
}

fn get_color() -> Colors {
    let pos = Mouse::new().get_position().unwrap();
    let pixels = get_pixels(pos.x as u32, pos.y as u32).unwrap();

    Colors::new(utils::to_hex(&pixels), utils::to_rgb(&pixels))
}

fn main() {
    let app = app::App::default();
    let mut win = Window::default()
        .with_size(150, 200)
        .with_label("ColorCopy");
    let mut pack = Pack::default().with_size(120, 190).center_of(&win);

    let c: Colors = get_color();
    pack.set_spacing(10);

    Frame::default()
        .with_size(0, 40)
        .with_label("Clip to copy!");

    let mut noice_frame = Frame::default().with_size(0, 30).with_label("██████");
    let test_color = fltk::enums::Color::from_hex_str(&c.hex).unwrap();
    noice_frame.set_color(test_color);
    noice_frame.set_selection_color(test_color);
    noice_frame.set_label_color(test_color);
    // Frame::default().with_size(0, 40).set_label("Noice");

    let mut but_hex = Button::default().with_size(0, 20).with_label(&c.hex);
    let mut but_rgb = Button::default().with_size(0, 20).with_label(&c.rgb);

    but_hex.set_callback(move |_| {
        //TODO:  try prnting struct values after the app quits
        utils::copy_to_clip(&c.hex);
        // app.quit();
    });
    but_rgb.set_callback(move |_| {
        utils::copy_to_clip(&c.rgb);
        // app.quit();
    });

    let mut quit = Button::default().with_size(0, 20).with_label("Cancel");
    quit.set_callback(move |_| app.quit());
    pack.end();

    win.end();
    win.show();
    app.run().unwrap();
}

fn get_pixels(x: u32, y: u32) -> Result<Vec<u8>, image::error::ImageError> {
    let ss = utils::screenshot();

    let img = image::open(&ss)?.to_rgb8();
    utils::delete_ss(ss);

    let pixel = img.get_pixel(x, y).0.to_vec();
    Ok(pixel)
}
