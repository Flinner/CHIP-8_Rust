use crate::{config::Config, display::DISPLAY, random, renderer};

use super::CPU;

/// 00E0 - CLS
pub(super) fn clear_screen() {
    unsafe { DISPLAY.clear() }
    trace!("Cleared Screen!");
}

/// 00EE - RET
pub(super) fn return_subroutine(cpu: &mut CPU) {
    cpu.pc = cpu.stack[cpu.stack_p];
    cpu.stack_p -= 1;

    //jump
    trace!("after RET");
    trace!("stack: {:X?}", cpu.stack);
    trace!("stack_p:{:X} ;PC = {:X}", cpu.stack_p, cpu.pc)
}

/// 0nnn - SYS addr
/// 1nnn - JP addr
pub(super) fn jump_nnn(cpu: &mut CPU, nnn: u16) {
    cpu.pc = nnn;
    trace!("PC = {nnn:X}")
}

/// 2nnn - CALL addr
pub(super) fn call_subroutine(cpu: &mut CPU, nnn: u16) {
    trace!("before JMP");
    trace!("stack: {:X?}", cpu.stack);
    trace!("stack_p:{:X} ;PC = {nnn:X}", cpu.stack_p);
    let current_pc = cpu.pc;
    cpu.stack_p += 1;

    // save pc
    cpu.stack[cpu.stack_p] = current_pc;

    //jump
    cpu.pc = nnn;
    trace!("after JMP");
    trace!("stack: {:X?}", cpu.stack);
    trace!("stack_p:{:X} ;PC = {nnn:X}", cpu.stack_p)
}

/// 3xkk - SE Vx, byte
pub(super) fn skip_if_vx_eq_nn(cpu: &mut CPU, x: usize, nn: u8) {
    if cpu.reg[x] == nn {
        cpu.increment_pc()
    }
}

/// 4xkk - SNE Vx, byte
pub(super) fn skip_if_vx_neq_nn(cpu: &mut CPU, x: usize, nn: u8) {
    if cpu.reg[x] != nn {
        cpu.increment_pc()
    }
}
/// 5xy0 - SE Vx, Vy
pub(super) fn skip_if_vx_eq_vy(cpu: &mut CPU, x: usize, y: usize) {
    if cpu.reg[x] == cpu.reg[y] {
        cpu.increment_pc()
    }
}
/// 6xkk - LD Vx, byte
pub(super) fn set_reg_x_nn(cpu: &mut CPU, x: usize, nn: u8) {
    cpu.reg[x] = nn;
    trace!("reg[{x:X}] = {nn:X}")
}

/// 7xkk - ADD Vx, byte
pub(super) fn add_reg_x_nn(cpu: &mut CPU, x: usize, nn: u8) {
    cpu.reg[x] = cpu.reg[x].overflowing_add(nn).0;
    trace!("reg[{x:X}] += {nn:X} = {:X}", cpu.reg[x])
}

/// 8xy0 - LD Vx, Vy
pub(super) fn set_vx_to_vy(cpu: &mut CPU, x: usize, y: usize) {
    cpu.reg[x] = cpu.reg[y];
    trace!("reg[{x:X}] = reg[{y:X}] = {:X}", cpu.reg[x]);
}

/// 8xy1 - OR Vx, Vy
pub(super) fn or_vx_vy(cpu: &mut CPU, x: usize, y: usize) {
    cpu.reg[x] |= cpu.reg[y];
    trace!("reg[{x:X}] |= reg[{y:X}] = {:X}", cpu.reg[x]);
}

/// 8xy2 - AND Vx, Vy
pub(super) fn and_vx_vy(cpu: &mut CPU, x: usize, y: usize) {
    cpu.reg[x] &= cpu.reg[y];
    trace!("reg[{x:X}] |= reg[{y:X}] = {:X}", cpu.reg[x]);
}

/// 8xy3 - XOR Vx, Vy
pub(super) fn xor_vx_vy(cpu: &mut CPU, x: usize, y: usize) {
    cpu.reg[x] ^= cpu.reg[y];
    trace!("reg[{x:X}] |= reg[{y:X}] = {:X}", cpu.reg[x]);
}

/// 8xy4 - ADD Vx, Vy
pub(super) fn add_vx_to_vy_carry_flag(cpu: &mut CPU, x: usize, nn: u8) {
    let (sum, overflow) = cpu.reg[x].overflowing_add(nn);
    let overflow = if overflow { 1 } else { 0 };

    cpu.reg[x] = sum;
    cpu.reg[0xF] = overflow;
    trace!("reg[{x:X}] += {nn:X} = {:X}", cpu.reg[x]);
    trace!("overflow: reg[0xF]={}", overflow);
}

/// 8xy5 - SUB Vx, Vy
pub(super) fn sub_vx_vy(cpu: &mut CPU, x: usize, y: usize) {
    let (result, overflow) = cpu.reg[x].overflowing_sub(cpu.reg[y]);
    let overflow = if overflow { 1 } else { 0 };

    cpu.reg[x] = result;
    cpu.reg[0xF] = overflow;
    trace!("reg[{x:X}] -= reg[{y:X}] = {:X}", cpu.reg[x]);
    trace!("overflow: reg[0xF]={}", overflow);
}

/// 8xy6 - SHR Vx {, Vy}
pub(super) fn shift_vy_right(cpu: &mut CPU, x: usize, y: usize) {
    // configurable to support multiple game (?)
    let to_shift = if Config::get().shift_vx { x } else { y };

    cpu.reg[x] = cpu.reg[to_shift] >> 1;
    cpu.reg[0xF] = cpu.reg[to_shift] & 1;

    trace!(
        "reg[{x:X}] = cpu.reg[{to_shift:X}] >> 1  = {:X}",
        cpu.reg[x]
    );
}

/// 8xy7 - SUBN Vx, Vy
pub(super) fn sub_vy_vx(cpu: &mut CPU, x: usize, y: usize) {
    let (result, overflow) = cpu.reg[y].overflowing_sub(cpu.reg[x]);
    let overflow = if overflow { 1 } else { 0 };

    cpu.reg[x] = result;
    cpu.reg[0xF] = overflow;
    trace!("reg[{x:X}] -= reg[{y:X}] = {:X}", cpu.reg[x]);
    trace!("overflow: reg[0xF]={}", overflow);
}

/// 8xyE - SHL Vx {, Vy}
pub(super) fn shift_vy_left(cpu: &mut CPU, x: usize, y: usize) {
    // configurable to support multiple game (?)
    let to_shift = if Config::get().shift_vx { x } else { y };

    cpu.reg[x] = cpu.reg[to_shift] << 1;
    cpu.reg[0xF] = cpu.reg[to_shift] & 1;

    trace!(
        "reg[{x:X}] = cpu.reg[{to_shift:X}] >> 1  = {:X}",
        cpu.reg[x]
    );
}
/// 9xy0 - SNE Vx, Vy
pub(super) fn skip_if_vx_neq_vy(cpu: &mut CPU, x: usize, y: usize) {
    if cpu.reg[x] != cpu.reg[y] {
        cpu.increment_pc()
    }
}
/// Annn - LD I, addr
pub(super) fn set_index_reg_nnn(cpu: &mut CPU, nnn: usize) {
    cpu.index_register = nnn;
    trace!("index_reg = {nnn:X}")
}

/// Bnnn - JP V0, addr if `Config.jmp_offset_v0: true`
/// Bxnn - JP Vx, addr if `Config.jmp_offset_v0: false`
pub(super) fn jump_with_offset(cpu: &mut CPU, nnn: usize, x: usize, nn: u8) {
    if Config::get().jmp_offset_v0 {
        cpu.index_register = nnn + cpu.reg[0] as usize;

        trace!(
            "index_reg = {nnn:X} + cput.reg[0] = {:X}",
            cpu.index_register
        )
    } else {
        cpu.index_register = nn as usize + cpu.reg[x] as usize;

        trace!(
            "index_reg = {nn:X} + cput.reg[{x:X}] = {:X}",
            cpu.index_register
        )
    }
}

/// Cxkk - RND Vx, byte
pub(super) fn random_to_vx(cpu: &mut CPU, x: usize, nn: u8) {
    let random: u8 = random::get_random_max(255) as u8;
    cpu.reg[x] = random & nn;
}

/// Dxyn - DRW Vx, Vy, nibble
pub(super) fn draw(cpu: &mut CPU, x: usize, y: usize, n: u8) {
    let i_reg = cpu.index_register;

    // memory range that should be displayed
    let sprite_range = i_reg..(i_reg + (n as usize));
    cpu.reg[0xF] =
        unsafe { DISPLAY.update_from_mem(cpu.reg[x], cpu.reg[y], &cpu.mem[sprite_range]) };

    trace!("reg[F] = {}", cpu.reg[0xF])
}
/// Ex9E - SKP Vx
pub(super) fn skip_if_vx_eq_key_pressed(cpu: &mut CPU, x: usize) {
    let renderer = renderer::get();

    if renderer.is_key_down(cpu.reg[x]) {
        cpu.increment_pc()
    }
}
/// ExA1 - SKNP Vx
pub(super) fn skip_if_vx_eq_key_not_pressed(cpu: &mut CPU, x: usize) {
    let renderer = renderer::get();

    if !renderer.is_key_down(cpu.reg[x]) {
        cpu.increment_pc()
    }
}
/// Fx07 - LD Vx, DT
pub(super) fn set_vx_to_delay_timer(cpu: &mut CPU, x: usize) {
    cpu.reg[x] = cpu.delay_timer;
    trace!("reg[{x:X}] = {:X}", cpu.reg[x])
}
/// Fx0A - LD Vx, K
/// Fx15 - LD DT, Vx
pub(super) fn set_delay_timer_to_vx(cpu: &mut CPU, x: usize) {
    cpu.delay_timer = cpu.reg[x];
    trace!("delay_timer = reg[{x:X}] = {:X} ", cpu.delay_timer)
}
/// Fx18 - LD ST, Vx
pub(super) fn set_sound_timer_to_vx(cpu: &mut CPU, x: usize) {
    cpu.sound_timer = cpu.reg[x];
    trace!("sound_timer = reg[{x:X}] = {:X} ", cpu.sound_timer)
}
/// Fx1E - ADD I, Vx
/// Fx29 - LD F, Vx
/// Fx33 - LD B, Vx
/// Fx55 - LD [I], Vx
/// Fx65 - LD Vx, [I]

// to keep comments lol
struct Tmp {}
