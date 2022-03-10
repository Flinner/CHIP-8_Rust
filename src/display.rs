pub const DISPLAY_X_PIXELS: usize = 64;
pub const DISPLAY_Y_PIXELS: usize = 32;

pub static mut DISPLAY: Display = Display::default();

pub struct Display {
    pub buffer: [[u8; DISPLAY_X_PIXELS / 8]; DISPLAY_Y_PIXELS], //TODO: make it a 1D array!
    pub scale: usize,
}

impl Display {
    pub const fn default() -> Self {
        Self {
            buffer: [[0; DISPLAY_X_PIXELS / 8]; DISPLAY_Y_PIXELS],
            scale: 10,
        }
    }

    pub fn update_from_mem(&mut self, vx: u8, vy: u8, disp_mem: &[u8]) -> u8 {
        let mut vx = vx as usize;
        let mut vy = vy as usize;
        trace!("Vx: {vx:X} Vy: {vy:X}");
        trace!("dis_mem: {:X?}", disp_mem);

        let mut collision = false;

        for byte in disp_mem {
            let original_buffer_byte = self.buffer[vy][vx / 8];
            self.buffer[vy][vx / 8] ^= byte;
            let new_buffer_byte = self.buffer[vy][vx / 8];
            trace!(
                "updated buffer ({},{vy}), {} => {}",
                vx / 8,
                new_buffer_byte,
                original_buffer_byte
            );

            collision |= (original_buffer_byte & new_buffer_byte) != original_buffer_byte;

            trace!(
                "{original_buffer_byte:#09b} ^ {byte:#09b} = {new_buffer_byte:#09b} | {collision}"
            );

            vx += 1;
            trace!("Vx: {vx:X} Vy: {vy:X}");
            // go to next row if full
            if (vx / 8) > DISPLAY_X_PIXELS {
                vx = 0;
                vy += 1;
            }
        }
        trace!("updated Display from memory");
        // this goes to VF
        match collision {
            true => 1,
            false => 0,
        }
    }
    pub fn clear(&mut self) {
        // TOOD: just clear through renderer
        self.buffer = [[0; DISPLAY_X_PIXELS / 8]; DISPLAY_Y_PIXELS];
        eprintln!("cleared screen!")
    }
}
