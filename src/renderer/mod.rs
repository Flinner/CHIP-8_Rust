//mod raylib;

mod raylib;

use ::raylib::consts::KeyboardKey;

use crate::display::DISPLAY;

pub static mut RENDER_BACKEND: Option<Backend> = None;

pub enum Backend {
    Raylib {
        rl: ::raylib::RaylibHandle,
        thread: ::raylib::RaylibThread,
    },
}

impl Backend {
    pub fn render(&mut self) {
        let display = unsafe { &DISPLAY };

        match self {
            Self::Raylib { rl, thread } => raylib::render(rl, thread, display),
        };
    }

    pub fn should_close(&mut self) -> bool {
        match self {
            Self::Raylib { rl, thread: _ } => raylib::should_close(rl),
        }
    }
    pub fn is_key_pressed(&mut self, key: KeyboardKey) -> bool {
        match self {
            Self::Raylib { rl, thread: _ } => rl.is_key_up(key),
        }
    }
}

// iterate over bits
// TODO: use 1 bit size!
pub fn binary_iter(n: u8) -> impl Iterator<Item = u8> {
    (1..=8).map(move |i| (n << (8 - i)) >> 7)
}
