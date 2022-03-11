extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use chip_8::cpu::CPU;
use chip_8::renderer::{Backend, RENDER_BACKEND};

fn main() {
    pretty_env_logger::init();

    let mut cpu = CPU::new();
    cpu.load_rom("IBM-Logo.ch8");
    // cpu.load_rom("roms/test_opcode.ch8");
    cpu.load_default_font();
    cpu.pc = 0x200;

    // // initialize renderer here!
    // let (rl, thread) = //.
    // 	raylib::init().size(640, 320).title("CHIP-8 Emulator").build();

    // unsafe { RENDER_BACKEND = Some(Backend::Raylib { rl, thread }) };
    unsafe { RENDER_BACKEND = Some(Backend::Terminal) };

    let renderer = unsafe { &mut RENDER_BACKEND.as_mut().unwrap() };

    let mut buf = String::new();
    while !renderer.should_close() {
        std::io::stdin().read_line(&mut buf).unwrap(); // debugging
        trace!("main game loop:");
        cpu.decode_and_execture();
        renderer.render();
    }
}
