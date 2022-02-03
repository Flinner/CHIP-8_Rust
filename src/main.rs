use chip_8::cpu::CPU;
use chip_8::gui;
use raylib::prelude::*;

fn main() {
    // 10x10 pixels = pixel
    let (mut rl, thread) = raylib::init().size(640, 320).title("Hello, World").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        gui::draw_pixel(&mut d, 0, 9, 0);
        gui::draw_pixel(&mut d, 10, 10, 0);
        gui::draw_pixel(&mut d, 11, 11, 0);
        // gui::draw_pixel(&mut d, 3, 3, &[1]);
        // d.draw_text("Hello, world!", 12, 12, 20, Color::WHITE);
    }
    // let mut cpu = CPU::new();
}
