use crate::display::Display;

use super::binary_iter;

pub fn render(display: &Display) {
    let mut fmted_row = String::with_capacity(64);

    for row in display.buffer {
        for byte in row {
            binary_iter(byte).for_each(|bit| match bit {
                0 => fmted_row += "\x1b[1;30m█",
                1 => fmted_row += "\x1b[1;37m█",
                // 1 => fmted_row += "",
                _ => panic!("Bit can only be 0 or 1"),
            })
        }
        println!("{fmted_row}");
        fmted_row.clear();
    }
}
