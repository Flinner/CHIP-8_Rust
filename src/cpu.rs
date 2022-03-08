use std::{fs::File, io::Read, time::Duration};

use crate::{
    display::Display,
    gui::{self, draw_from_mem},
};
use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct CPU {
    /// Memory 4K bytes
    pub mem: [u8; 4096],
    /// Program Counter
    pub pc: u16,
    pub index_register: usize,
    pub stack: [u16; 16],
    // delay_timer: u8,
    // sound_timer: u8,
    pub reg: [u8; 16],
    pub display: Display,
}

#[derive(Debug, Clone, Copy)]
pub struct Decoded {
    /// 1st nibble
    pub n1: u8,
    /// 2nd nibble (4-bit)
    /// DO **NOT** use its value directly!
    /// This points to a register.
    pub x: usize,
    /// 3rd nibble (4-bit)
    /// DO **NOT** use its value directly!
    /// This points to a register
    pub y: usize,
    /// 4th nibble (4-bit)
    /// Can use it as a value
    pub n: u8,
    /// 3,4 byte (8-bit)
    /// Can use it as a value
    pub nn: u8,
    /// 2,3,4 (12-bit)
    /// Can use it as a value
    pub nnn: u16,
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            mem: [0; 4096],
            pc: 0,
            index_register: 0,
            stack: Default::default(),
            reg: Default::default(),
            display: Default::default(),
        }
    }
    pub fn load_rom(&mut self, path: &str) {
        let mut file_content = Vec::with_capacity(4096);
        file_content.append(&mut vec![0; 512]);

        let mut file = File::open(&path).expect("Unable to open file");
        file.read_to_end(&mut file_content).expect("Unable to read");

        file_content.resize(4096, 0);

        self.mem = file_content.try_into().unwrap_or_else(|v: Vec<u8>| {
            panic!("Expected a Vec of length {} but it was {}", 512, v.len())
        })
    }
    fn fetch(&mut self) -> u16 {
        let pc = self.pc as usize;
        let big = self.mem[pc];
        let little = self.mem[pc + 1];

        self.pc += 2;

        ((big as u16) << 8) + little as u16
    }
    pub fn decode(&mut self) -> Decoded {
        let instruction: u16 = self.fetch();
        let n1: u8 = ((instruction & 0xF000) >> 12) as u8;
        let x: usize = ((instruction & 0x0F00) >> 8) as usize;
        let y: usize = ((instruction & 0x00F0) >> 4) as usize;
        let n: u8 = (instruction & 0x000F) /* **/ as u8;
        let nn: u8 = (instruction & 0x0FF) /* **/ as u8;
        let nnn: u16 = (instruction & 0x0FFF) as u16;

        // eprintln!("{:#?}", Decoded {n1, x, y, n, nn, nnn});

        Decoded {
            n1,
            x,
            y,
            n,
            nn,
            nnn,
        }
    }
    pub fn exectue(
        &mut self,
        Decoded {
            n1,
            x,
            y,
            n,
            nn,
            nnn,
        }: Decoded,
        d: &mut RaylibDrawHandle,
    ) {
        let reg = &mut self.reg;
        let mem = &mut self.mem;
        let i_reg = self.index_register;

        // memory range that should be displayed
        let disp_mem = i_reg..(i_reg + (n as usize));
        // println!("{:x}{:x} {:x}{:x}", n1, x, y, n);
        match (n1, x, y, n) {
            // clear screen
            (0x0, 0, 0xE, 0) => gui::clear(d),
            // Jump to NNN
            (0x1, _, _, _) => self.pc = nnn,
            // set Register x to NN
            (0x6, _, _, _) => self.reg[x] = nn,
            // add NN to Register x
            (0x7, _, _, _) => self.reg[x] += nn,
            // set index Register to NNN
            (0xA, _, _, _) => self.index_register = nnn as usize,
            // Display/Draw x y n
            (0xD, _, _, _) => {
                self.reg[0xF] =
                    gui::draw_from_mem(d, &mut reg[..], x, y, &mem[disp_mem], &mut self.display)
            }
            // _ => (),
            _ => d.draw_text("Instruction not found!", 20, 20, 30, Color::WHITE),
            // _ => todo!("Instruction Not Yet Implemented!"),
        };
    }
    pub fn decode_and_execture(&mut self, d: &mut RaylibDrawHandle) {
        let decoded = self.decode();
        self.exectue(decoded, d)
    }
}
