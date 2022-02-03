use raylib::prelude::*;

pub fn clear(d: &mut RaylibDrawHandle) {
    d.clear_background(Color::WHITE);
}

pub fn draw_pixel(d: &mut RaylibDrawHandle, x: u8, y: u8, n: u8) {
    d.draw_rectangle(0, 0, 10, 10, Color::BLACK);
}
