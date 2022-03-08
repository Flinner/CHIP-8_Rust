use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::gui::{DISPLAY_HEIGHT, DISPLAY_WIDTH, PIXEL_HEIGHT, PIXEL_WIDTH};

const DISPLAY_HEIGHT_U8: usize = (DISPLAY_HEIGHT / 8) as usize;
const DISPLAY_WIDTH_U8: usize = (DISPLAY_WIDTH / 8) as usize;

#[derive(Debug, Default)]
pub struct Display {
    /// 8 pixels per `u8` entry.
    pub buffer: [u8; (DISPLAY_HEIGHT_U8 * DISPLAY_WIDTH_U8)],
    /// Background, usually be white
    pub bg: Color,
    /// Foreground (drawn on), usually black
    pub fg: Color,
    /// Width/Height of a singel pixel
    pub scale: usize,
}

impl Display {
    /// Renders bufffer content to Raylib GUI
    pub fn render(&self, d: &mut RaylibDrawHandle) {
        for x in 0..DISPLAY_WIDTH_U8 {
            for y in 0..DISPLAY_HEIGHT_U8 {
                d.draw_rectangle(x as i32, y as i32, PIXEL_WIDTH, PIXEL_HEIGHT, self.fg);
            }
        }
    }
}

// iterate over bits
fn binary_iter(n: u8) -> impl Iterator<Item = u8> {
    (1..=8).map(move |i| (n << (8 - i)) >> 7)
}
