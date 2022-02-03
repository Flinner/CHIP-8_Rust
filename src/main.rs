use chip_8::cpu::CPU;
use raylib::prelude::*;

fn main() {
    // 10x10 pixels = pixel
    let (mut rl, thread) = raylib::init().size(640, 320).title("Hello, World").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
    // let mut cpu = CPU::new();
}
