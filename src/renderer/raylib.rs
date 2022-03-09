use raylib::color::Color;
use raylib::prelude::RaylibDraw;
use raylib::{RaylibHandle, RaylibThread};

use crate::display::Display;

pub fn render(rl: &mut RaylibHandle, thread: &mut RaylibThread, display: &Display) {
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::WHITE);
    d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
}

pub fn should_close(rl: &mut RaylibHandle) -> bool {
    rl.window_should_close()
}
