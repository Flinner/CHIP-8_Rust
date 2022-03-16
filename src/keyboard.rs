use raylib::consts::KeyboardKey;

/// maps 0-F values of CHIP-8 to real keyboard keys
///```
/// 1 2 3 4
/// Q W E R
/// A S D F
/// Z X C V
///```
pub fn key_mapping(keycode: u8) -> Option<KeyboardKey> {
    use KeyboardKey::*;

    match keycode {
        0x0 => Some(KEY_ONE),
        0x1 => Some(KEY_TWO),
        0x2 => Some(KEY_THREE),
        0x3 => Some(KEY_FOUR),
        0x4 => Some(KEY_Q),
        0x5 => Some(KEY_W),
        0x6 => Some(KEY_E),
        0x7 => Some(KEY_R),
        0x8 => Some(KEY_A),
        0x9 => Some(KEY_S),
        0xA => Some(KEY_D),
        0xB => Some(KEY_F),
        0xC => Some(KEY_Z),
        0xD => Some(KEY_X),
        0xE => Some(KEY_C),
        0xF => Some(KEY_V),
        _ => None,
    }
}
