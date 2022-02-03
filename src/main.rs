use chip_8::cpu::CPU;
use chip_8::gui;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 320).title("Hello, World").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        gui::draw_from_mem(&mut d, 62, 9, &[0x1]);
    }
    // let mut cpu = CPU::new();
}
