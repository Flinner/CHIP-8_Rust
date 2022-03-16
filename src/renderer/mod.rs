//mod raylib;

mod raylib;
mod terminal;

use crate::display::DISPLAY;

static mut RENDER_BACKEND: Option<Backend> = None;

pub enum BackendChoice {
    Raylib,
    Terminal,
}

pub enum Backend {
    Raylib {
        rl: ::raylib::RaylibHandle,
        thread: ::raylib::RaylibThread,
    },
    Terminal,
}

pub fn init(choice: BackendChoice) -> &'static mut Backend {
    match choice {
        BackendChoice::Raylib => {
            let (rl, thread) = //.
	::raylib::init().size(640, 320).title("CHIP-8 Emulator").build();
            unsafe { RENDER_BACKEND = Some(Backend::Raylib { rl, thread }) };
        }
        BackendChoice::Terminal => {
            unsafe { RENDER_BACKEND = Some(Backend::Terminal) };
        }
    }
    self::get()
}

pub fn get() -> &'static mut Backend {
    unsafe { RENDER_BACKEND.as_mut().unwrap() }
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
            Self::Terminal => false, // TODO!
        }
    }

    /// Returns if the key currently being pressed
    /// Value between `0` and `F`
    /// non blocking
    pub fn is_key_down(&mut self, key: u8) -> bool {
        match self {
            Backend::Raylib { rl, thread: _ } => raylib::is_key_down(rl, key),
            Backend::Terminal => todo!(),
        }
    }
}

// iterate over bits
pub fn binary_iter(n: u8) -> impl Iterator<Item = u8> {
    (1..=8).rev().map(move |i| (n << (8 - i)) >> 7)
}
