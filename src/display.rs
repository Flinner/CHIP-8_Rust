use crate::renderer::binary_iter;

pub const DISPLAY_X_PIXELS: usize = 64;
pub const DISPLAY_Y_PIXELS: usize = 32;

pub static mut DISPLAY: Display = Display::default();

pub struct Display {
    pub buffer: [[u8; DISPLAY_X_PIXELS]; DISPLAY_Y_PIXELS], //TODO: make it a 1D array!
    pub scale: usize,
}

impl Display {
    pub const fn default() -> Self {
        Self {
            buffer: [[0; DISPLAY_X_PIXELS]; DISPLAY_Y_PIXELS],
            scale: 10,
        }
    }

    pub fn update_from_mem(&mut self, vx: u8, vy: u8, disp_mem: &[u8]) -> u8 {
        let vx = vx as usize % DISPLAY_X_PIXELS;
        let vy = vy as usize % DISPLAY_Y_PIXELS;
        trace!("Vx: {vx:X} Vy: {vy:X}");
        trace!("dis_mem: {:X?}", disp_mem);

        let mut collision = false;

        for (i, &byte) in disp_mem.iter().enumerate() {
            let y = vy + i;
            if y > DISPLAY_Y_PIXELS {
                continue;
            };
            for (j, bit) in binary_iter(byte).enumerate() {
                let x = vx + j;

                if x >= DISPLAY_X_PIXELS {
                    continue;
                };

                let orig_buf_bit = self.buffer[y][x];
                self.buffer[y][x] ^= bit;
                let new_buf_bit = self.buffer[y][x];

                collision |= (orig_buf_bit & new_buf_bit) != orig_buf_bit;
            }
        }
        match collision {
            true => 1,
            false => 0,
        }
    }
    pub fn clear(&mut self) {
        // TOOD: just clear through renderer
        self.buffer = [[0; DISPLAY_X_PIXELS]; DISPLAY_Y_PIXELS];
        eprintln!("cleared screen!")
    }
}
