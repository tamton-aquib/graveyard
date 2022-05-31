use fltk::{app, button::Button, frame::Frame, group::Pack, prelude::*, window::Window};
use mouse_rs::Mouse;
mod utils;

#[derive(Debug)]
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

fn get_pixels(x: u32, y: u32) -> Result<[u8; 3], Box<dyn std::error::Error>> {
    let screen = x11_screenshot::Screen::open().expect("Couldn't open!");
    let ss = screen.capture().expect("Failed to take screenshot!");

    let pixel = ss.get_pixel(x, y).0;
    Ok(pixel)
}

fn main() {
    let app = app::App::default();
    let mut win = Window::default()
        .with_size(150, 200)
        .with_label("ColorCopy");
    let mut pack = Pack::default().with_size(120, 190).center_of(&win);

    // TODO: bad code structuring
    let c: Colors = get_color();
    pack.set_spacing(10);

    Frame::default()
        .with_size(0, 40)
        .with_label("Clip to copy!");

    let mut frame = Frame::default().with_size(0, 30).with_label("██████");
    let test_color = fltk::enums::Color::from_hex_str(&c.hex).unwrap();
    frame.set_color(test_color);
    frame.set_selection_color(test_color);
    frame.set_label_color(test_color);

    let mut but_hex = Button::default().with_size(0, 20).with_label(&c.hex);
    let mut but_rgb = Button::default().with_size(0, 20).with_label(&c.rgb);

    but_hex.set_callback(move |_| {
        utils::copy_to_clip(&c.hex);
        app.quit();
    });
    but_rgb.set_callback(move |_| {
        utils::copy_to_clip(&c.rgb);
        app.quit();
    });

    let mut quit = Button::default().with_size(0, 20).with_label("Cancel");
    quit.set_callback(move |_| app.quit());
    pack.end();

    win.end();
    win.show();
    app.run().unwrap();
}
