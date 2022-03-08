const DISPLAY_X_PIXELS: usize = 64;
const DISPLAY_Y_PIXELS: usize = 32;

pub static mut DISPLAY: Display = Display::default();

pub struct Display {
    pub buffer: [[u8; DISPLAY_X_PIXELS]; DISPLAY_Y_PIXELS], //TODO: make it a 1D array!
    pub scale: usize,
}

impl Display {
    pub const fn default() -> Self {
        Self {
            buffer: [[0; DISPLAY_X_PIXELS]; DISPLAY_Y_PIXELS],
            scale: 1,
        }
    }

    pub fn update_from_mem(&mut self, vx: u8, vy: u8, disp_mem: &[u8]) -> u8 {
        let mut vx = vx as usize;
        let mut vy = vy as usize;

        let mut collision = false;

        eprintln!("original \t byte \t new \t collision");
        for byte in disp_mem {
            let original_buffer_byte = self.buffer[vy][vx];
            self.buffer[vy][vx] ^= byte;
            let new_buffer_byte = self.buffer[vy][vx];
            collision |= (original_buffer_byte & new_buffer_byte) != original_buffer_byte;

            // eprintln!(
            //     "{original_buffer_byte:#05b}\t {byte:#05b}\t = {new_buffer_byte:#05b} \t {collision}"
            // );

            vx += 1;
            // go to next row if full
            if vx > DISPLAY_X_PIXELS {
                vx = 0;
                vy += 1;
            }
        }
        eprintln!("updating from memory");
        // this goes to VF
        match collision {
            true => 1,
            false => 0,
        }
    }
    pub fn clear(&mut self) {
        self.buffer = [[0; DISPLAY_X_PIXELS]; DISPLAY_Y_PIXELS];
        eprintln!("cleared screen!")
    }
}
