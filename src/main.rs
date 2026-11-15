use citro2d::{Citro2d, Color, TextBuf};
use ctru::prelude::*;

pub(crate) mod utils;

#[allow(
    clippy::significant_drop_tightening,
    reason = "Stop clippy from freaking out about c2d not being dropped sooner."
)]
fn main() {
    let gfx = Gfx::new().unwrap(); // Graphics stuff
    let apt = Apt::new().unwrap(); // Applet stuff
    let mut hid = Hid::new().unwrap(); // User input stuff

    let c2d = Citro2d::new(&gfx).unwrap(); // Citro2d handle

    let text_buf_top_screen = TextBuf::new(256).unwrap();
    while apt.main_loop() {
        hid.scan_input();

        // let keys_pressed = hid.keys_down();
        // let keys_held = hid.keys_held();
        let keys_released = hid.keys_up();

        // Release the SELECT key to quit the program.
        if keys_released.contains(KeyPad::SELECT) {
            break;
        }

        text_buf_top_screen.clear();
        let text = text_buf_top_screen.parse("Press Select To Quit!").unwrap();

        c2d.frame(|frame| {
            frame.scene(c2d.top_screen(), Color::rgb(255, 0, 0), |scene| {
                scene.draw_text(&text, 0., 0., 0.5, Color::rgb(255, 255, 255));
            });
        });
    }
}
