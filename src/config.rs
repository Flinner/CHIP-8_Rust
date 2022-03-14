pub static mut CONFIG: Config = Config::new();

pub struct Config {
    /// when using `8xy6` and `8xye`
    /// Shift `Vy` or `Vx`?
    /// starting with CHIP-48 and SUPER-CHIP, `Vx` is shifted
    pub shift_vx: bool,
    /// `true` : Bnnn Jump to address nnn + V[0]
    /// `false`: Bxnn Jump to address nn + V[x]
    /// Bnnn (true) isn't widley used
    pub jmp_offset_v0: bool,
}

impl Config {
    pub const fn new() -> Self {
        Config {
            shift_vx: true,
            jmp_offset_v0: false,
        }
    }

    pub fn get() -> &'static Self {
        unsafe { &CONFIG }
    }
}
