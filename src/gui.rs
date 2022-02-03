use raylib::prelude::*;
/// users's gui width
pub const GUI_WIDTH: usize = 640;
/// users's gui height
pub const GUI_HEIGHT: usize = 320;

/// number of width pixels as defined by CHIP-8 spec
pub const DISPLAY_WIDTH: usize = 64;
/// number of height pixels as defined by CHIP-8 spec
pub const DISPLAY_HEIGHT: usize = 32;

/// a single pixel's width in GUI
pub const PIXEL_WIDTH: i32 = (GUI_WIDTH / DISPLAY_WIDTH) as i32;
/// a single pixel's height in GUI
pub const PIXEL_HEIGHT: i32 = (GUI_HEIGHT / DISPLAY_HEIGHT) as i32;

pub fn clear(d: &mut RaylibDrawHandle) {
    d.clear_background(Color::WHITE);
}

/// Pixels here should be between `PIXEL_{HEIGHT,WIDTH}` or it will wrap
pub fn draw_from_mem(d: &mut RaylibDrawHandle, x: u8, y: u8, rows: &[u8]) {
    // pixels can wrap
    let x = x as usize % DISPLAY_WIDTH;
    let y = y as usize % DISPLAY_HEIGHT;

    // reg[0xF]
    for row in rows {
        for (ix, bit) in binary_iter(*row).enumerate() {
            match bit {
                0 => draw_pixel(d, (x + ix) as u8, y as u8, Color::BLACK),
                1 => draw_pixel(d, (x + ix) as u8, y as u8, Color::WHITE),
                _ => todo!(),
            }
        }
    }
}

// iterate over bits
fn binary_iter(n: u8) -> impl Iterator<Item = u8> {
    (1..=8).map(move |i| (n << (8 - i)) >> 7)
}

/// Pixels greater than PIXEL_{HEIGHT,WIDTH} will not be drawn!
pub fn draw_pixel(d: &mut RaylibDrawHandle, x: u8, y: u8, color: Color) {
    let x = x as i32 * PIXEL_WIDTH;
    let y = y as i32 * PIXEL_HEIGHT;

    d.begin_blend_mode(BlendMode::BLEND_SUBTRACT_COLORS) // invert colors in subtraction mode
        .draw_rectangle(x, y, PIXEL_WIDTH, PIXEL_HEIGHT, color); // white is used to invert
}
