//mod raylib;

mod raylib;
mod terminal;

use ::raylib::consts::KeyboardKey;

use crate::display::DISPLAY;

pub static mut RENDER_BACKEND: Option<Backend> = None;

pub enum Backend {
    Raylib {
        rl: ::raylib::RaylibHandle,
        thread: ::raylib::RaylibThread,
    },
    Terminal,
}

impl Backend {
    pub fn render(&mut self) {
        let display = unsafe { &DISPLAY };

        match self {
            Self::Raylib { rl, thread } => raylib::render(rl, thread, display),
            Self::Terminal => terminal::render(display),
        };
    }

    pub fn should_close(&mut self) -> bool {
        match self {
            Self::Raylib { rl, thread: _ } => raylib::should_close(rl),
            Self::Terminal => false,
        }
    }
}

// iterate over bits
// TODO: use 1 bit size!
pub fn binary_iter(n: u8) -> impl Iterator<Item = u8> {
    (1..=8).map(move |i| (n << (8 - i)) >> 7)
}
