use std::thread::sleep;
use std::time::Duration;

use chip_8::cpu::CPU;
use chip_8::display::{Display, DISPLAY};

fn main() {
    let mut cpu = CPU::new();
    cpu.load_rom("IBM-Logo.ch8");

    unsafe { DISPLAY.buffer[0][2] = 1 };
    unsafe { DISPLAY.update_from_mem(0, 0, &[0, 1, 2, 3, 4]) };
    // loop {
    //     sleep(Duration::new(0, 100));
    //     cpu.decode_and_execture();
    //     // render
    // }
}
