use crate::display::DISPLAY;

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
/// 5xy0 - SE Vx, Vy
/// 6xkk - LD Vx, byte
pub(super) fn set_reg_x_nn(cpu: &mut CPU, x: usize, nn: u8) {
    cpu.reg[x] = nn;
    trace!("reg[{x:X}] = {nn:X}")
}

/// 7xkk - ADD Vx, byte
pub(super) fn add_reg_x_nn(cpu: &mut CPU, x: usize, nn: u8) {
    cpu.reg[x] += nn;
    trace!("reg[{x:X}] += {nn:X} = {:X}", cpu.reg[x] + nn)
}
/// 8xy0 - LD Vx, Vy
/// 8xy1 - OR Vx, Vy
/// 8xy2 - AND Vx, Vy
/// 8xy3 - XOR Vx, Vy
/// 8xy4 - ADD Vx, Vy
/// 8xy5 - SUB Vx, Vy
/// 8xy6 - SHR Vx {, Vy}
/// 8xy7 - SUBN Vx, Vy
/// 8xyE - SHL Vx {, Vy}
/// 9xy0 - SNE Vx, Vy
/// Annn - LD I, addr
pub(super) fn set_index_reg_nnn(cpu: &mut CPU, nnn: usize) {
    cpu.index_register = nnn;
    trace!("index_reg = {nnn:X}")
}
/// Bnnn - JP V0, addr
/// Cxkk - RND Vx, byte
/// Dxyn - DRW Vx, Vy, nibble
pub(super) fn draw(cpu: &mut CPU, x: usize, y: usize, n: u8) {
    let i_reg = cpu.index_register;

    // memory range that should be displayed
    let disp_mem = i_reg..(i_reg + (n as usize));
    cpu.reg[0xF] = unsafe { DISPLAY.update_from_mem(cpu.reg[x], cpu.reg[y], &cpu.mem[disp_mem]) };

    trace!("reg[F] = {}", cpu.reg[0xF])
}
/// Ex9E - SKP Vx
/// ExA1 - SKNP Vx
/// Fx07 - LD Vx, DT
/// Fx0A - LD Vx, K
/// Fx15 - LD DT, Vx
/// Fx18 - LD ST, Vx
/// Fx1E - ADD I, Vx
/// Fx29 - LD F, Vx
/// Fx33 - LD B, Vx
/// Fx55 - LD [I], Vx
/// Fx65 - LD Vx, [I]

struct Tmp {}
