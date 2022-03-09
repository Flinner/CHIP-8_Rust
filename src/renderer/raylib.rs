use raylib::color::Color;
use raylib::prelude::RaylibDraw;
use raylib::{RaylibHandle, RaylibThread};

use crate::display::Display;

use super::binary_iter;

pub fn render(rl: &mut RaylibHandle, thread: &mut RaylibThread, display: &Display) {
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::WHITE);

    let (mut x, mut y) = (0, 0);
    let scale = display.scale as i32;
    for row in display.buffer {
        for byte in row {
            binary_iter(byte).for_each(|bit| {
                d.draw_rectangle(x, y, scale, scale, bit_color(bit));
                x += scale;
            })
        }
        y += scale;
        x = 0;
    }
}

pub fn should_close(rl: &mut RaylibHandle) -> bool {
    rl.window_should_close()
}

fn bit_color(bit: u8) -> Color {
    match bit {
        0 => Color::BLACK,
        1 => Color::WHITE,
        _ => panic!(),
    }
}
