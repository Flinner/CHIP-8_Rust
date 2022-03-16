use crate::display::Display;

pub fn render(display: &Display) {
    let mut fmted_grid = String::with_capacity(64);

    for row in display.buffer {
        for bit in row {
            match bit {
                0 => fmted_grid += "\x1b[1;30m██",
                1 => fmted_grid += "\x1b[1;37m██",
                _ => panic!("Bit can only be 0 or 1"),
            }
        }
        fmted_grid += "\n";
    }
    println!("{fmted_grid}");
}
