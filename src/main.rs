use std::thread::sleep;
use std::time::Duration;

use chip_8::cpu::CPU;
use chip_8::renderer::{self, Backend, RENDER_BACKEND};
use raylib::prelude::*;

fn main() {
    let mut cpu = CPU::new();
    cpu.load_rom("IBM-Logo.ch8");
    // initialize renderer here!
    let (rl, thread) = //.
	raylib::init().size(640, 320).title("CHIP-8 Emulator").build();

    unsafe { RENDER_BACKEND = Some(Backend::Raylib { rl, thread }) };
    let renderer = unsafe { &mut RENDER_BACKEND.as_mut().unwrap() };

    // unsafe { renderer::RENDER_BACKEND = Some(Backend::Raylib { d }) };
    while !renderer.should_close() {
        sleep(Duration::new(0, 500000000)); // 0.5 seconds
        cpu.decode_and_execture();
        renderer.render();
    }
}
