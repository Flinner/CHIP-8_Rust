extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use chip_8::cpu::CPU;
use chip_8::renderer::{self, BackendChoice};

fn main() {
    pretty_env_logger::init();

    let mut cpu = CPU::new();
    cpu.load_rom("roms/test_opcode.ch8");
    cpu.load_default_font();
    cpu.pc = 0x200;

    let renderer = renderer::init(BackendChoice::Terminal);

    while !renderer.should_close() {
        std::io::stdin().read_line(&mut String::new()).unwrap(); // debugging
        trace!("main game loop:");
        cpu.decode_and_execture();
        renderer.render();
    }
}
