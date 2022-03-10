mod opcodes;

use std::{fs::File, io::Read, time::Duration};

use crate::display::{self, DISPLAY};

const FONTS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

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
    // pub display: Display,
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
        }
    }
    pub fn load_default_font(&mut self) {
        for (i, &byte) in FONTS.iter().enumerate() {
            let starting_mem = 0x50;
            self.mem[i + starting_mem] = byte;
        }
    }
    pub fn load_rom(&mut self, path: &str) {
        info!("Loading ROM: {path}");
        let mut file_content: Vec<u8> = vec![];

        let mut file = File::open(&path).expect("Unable to open file");
        file.read_to_end(&mut file_content).expect("Unable to read");

        for (i, &byte) in file_content.iter().enumerate() {
            let starting_mem = 512;
            self.mem[i + starting_mem] = byte;
        }
        info!("ROM loaded!");
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
    ) {
        trace!("decoded: {n1:X}{x:X}{y:X}{n:X}; PC: {:X}", self.pc - 2);
        match (n1, x, y, n) {
            (0, 0, 0, 0) => warn!("Uninitialized memory!"),
            (0x0, 0, 0xE, 0) => opcodes::clear_screen(),
            // Jump to NNN
            (0x1, _, _, _) => opcodes::jump_nnn(self, nnn),
            // set Register x to NN
            (0x6, _, _, _) => opcodes::set_reg_x_nn(self, x, nn),
            // add NN to Register x
            (0x7, _, _, _) => opcodes::add_reg_x_nn(self, x, nn),
            // set index Register to NNN
            (0xA, _, _, _) => opcodes::set_index_reg_nnn(self, nnn as usize),
            // Display/Draw x y n
            (0xD, x, y, _n) => opcodes::draw(self, x, y, n),
            // _ => (),
            a => todo!("Instruction Not yet Implemented!: {a:X?}"),
        };
        trace!("reg: {:X?}", self.reg);
        trace!("index_reg: {:X?}", self.index_register);
        trace!("pc: {:X?}", self.pc);
        //trace!("stack_pointer: {:?}", self.??);
    }
    pub fn decode_and_execture(&mut self) {
        let decoded = self.decode();
        self.exectue(decoded)
    }
}
