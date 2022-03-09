use raylib::color::Color;
use raylib::prelude::RaylibDraw;
use raylib::{RaylibHandle, RaylibThread};

use crate::display::DISPLAY;

pub static mut RENDER_BACKEND: Option<Backend> = None;

pub enum Backend {
    Raylib {
        rl: RaylibHandle,
        thread: RaylibThread,
    },
}

impl Backend {
    pub fn render(&mut self) {
        let (rl, thread) = match self {
            Self::Raylib { rl, thread } => (rl, thread),
        };
        let mut d = rl.begin_drawing(&thread);
        let buffer = unsafe { DISPLAY.buffer };
        let scale = unsafe { DISPLAY.scale };

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

        // for x in 0..(DISPLAY_X_PIXELS / 8) {
        //     let mut string = String::new();
        //     for y in 0..DISPLAY_Y_PIXELS {
        //         let byte = buffer[y][x];
        //         binary_iter(byte).for_each(|bit| {
        //             if bit == 0 {
        //                 // string += "\x1B[1;34m▀"
        //                 string += "0"
        //             } else {
        //                 // string += "\x1B[1;37m▀"
        //                 string += "▀"
        //             }
        //         });
        //     }
        // println!("{string}");
        // println!("\n");
        // }
    }

    // iterate over bits
    // fn binary_iter(n: u8) -> impl Iterator<Item = u8> {
    //     (1..=8).map(move |i| (n << (8 - i)) >> 7)
    // }
    pub fn should_close(&mut self) -> bool {
        let (rl, _) = match self {
            Self::Raylib { rl, thread } => (rl, thread),
        };
        rl.window_should_close()
    }
}
