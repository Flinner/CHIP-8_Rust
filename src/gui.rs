use raylib::prelude::*;

use crate::display::Display;
/// users's gui width
pub const GUI_WIDTH: i32 = 640;
/// users's gui height
pub const GUI_HEIGHT: i32 = 320;

/// number of width pixels as defined by CHIP-8 spec
pub const DISPLAY_WIDTH: i32 = 64;
/// number of height pixels as defined by CHIP-8 spec
pub const DISPLAY_HEIGHT: i32 = 32;

/// a single pixel's width in GUI
pub const PIXEL_WIDTH: i32 = (GUI_WIDTH / DISPLAY_WIDTH) as i32;
/// a single pixel's height in GUI
pub const PIXEL_HEIGHT: i32 = (GUI_HEIGHT / DISPLAY_HEIGHT) as i32;

pub fn clear(d: &mut RaylibDrawHandle) {
    d.clear_background(Color::BLACK);
}

/// Pixels here should be between `PIXEL_{HEIGHT,WIDTH}` or it will wrap at **starting** only.
/// don't wrap on edge
/// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
/// Returns `0` if no collision happened, otherwise `1`
pub fn draw_from_mem(
    d: &mut RaylibDrawHandle,
    reg: &mut [u8],
    x: usize,
    y: usize,
    rows: &[u8], // this is to be displayed
    display: &mut Display,
) -> u8 {
    // did any collision happen?
    let mut collision = false;

    // pixels can wrap, modulo 63 and 31
    let mut x: u8 = reg[x & (DISPLAY_WIDTH - 1) as usize];
    let mut y: u8 = reg[y & (DISPLAY_HEIGHT - 1) as usize];

    for &row in rows {
        display.buffer[(y * x + x) as usize] ^= row;
        if x as i32 >= DISPLAY_WIDTH {
            x = 0;
            y += 1;
        } else {
            x += 1;
        }
        //         for (ix, bit) in binary_iter(row).enumerate() {
        //             println!("({},{})", x + ix as i32, y + row as i32);
        //             let collided = match bit {
        //                 0 => draw_pixel(d, x + ix as i32, y, Color::BLACK),
        //                 1 => draw_pixel(d, x + ix as i32, y, Color::WHITE),
        //                 _ => panic!("won't happen"),
        //             };
        //             // set to true if atleast one collision happens
        //             collision = collision || collided;
        //         }
    }
    display.render(d);

    // to VF (reg[0xF])
    match collision {
        true => 1,
        false => 0,
    }
}

// // iterate over bits
// fn binary_iter(n: u8) -> impl Iterator<Item = u8> {
//     (1..=8).map(move |i| (n << (8 - i)) >> 7)
// }

// /// Pixels greater than PIXEL_{HEIGHT,WIDTH} will not be drawn!
// pub fn draw_pixel(d: &mut RaylibDrawHandle, x: i32, y: i32, color: Color) -> bool {
//     let x = x * PIXEL_WIDTH;
//     let y = y * PIXEL_HEIGHT;

//     d.begin_blend_mode(BlendMode::BLEND_SUBTRACT_COLORS) // invert colors in subtraction mode
//         .draw_rectangle(x, y, PIXEL_WIDTH, PIXEL_HEIGHT, color); // white is used to invert
//                                                                  // d.get_screen_data(a).get_image_data().len();
//                                                                  // d.get_screen_data(&thread).get_pixel_data_size();
//     x % 2 != 0
// }
