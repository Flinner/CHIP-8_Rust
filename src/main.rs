use std::time::Duration;

use chip_8::cpu::CPU;
use chip_8::gui::{GUI_HEIGHT, GUI_WIDTH};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(GUI_WIDTH, GUI_HEIGHT)
        .title("Hello, World")
        .build();
    rl.set_target_fps(60);

    let mut cpu = CPU::new();
    cpu.load_rom("IBM-Logo.ch8");

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.draw_fps(4, 4);
        std::thread::sleep(Duration::from_nanos(10000000));
        cpu.decode_and_execture(&mut d);
    }
}
