use crate::display::DISPLAY;

use super::CPU;

pub(super) fn clear_screen() {
    unsafe { DISPLAY.clear() }
    trace!("Cleared Screen!");
}

pub(super) fn jump_nnn(cpu: &mut CPU, nnn: u16) {
    cpu.pc = nnn;
    trace!("PC = {nnn:X}")
}

pub(super) fn set_reg_x_nn(cpu: &mut CPU, x: usize, nn: u8) {
    cpu.reg[x] = nn;
    trace!("reg[{x:X}] = {nn:X}")
}

pub(super) fn add_reg_x_nn(cpu: &mut CPU, x: usize, nn: u8) {
    cpu.reg[x] += nn;
    trace!("reg[{x:X}] += {nn:X} = {:X}", cpu.reg[x] + nn)
}

pub(super) fn set_index_reg_nnn(cpu: &mut CPU, nnn: usize) {
    cpu.index_register = nnn;
    trace!("index_reg = {nnn:X}")
}
pub(super) fn draw(cpu: &mut CPU, x: usize, y: usize, n: u8) {
    let i_reg = cpu.index_register;

    // memory range that should be displayed
    let disp_mem = i_reg..(i_reg + (n as usize));
    cpu.reg[0xF] = unsafe { DISPLAY.update_from_mem(cpu.reg[x], cpu.reg[y], &cpu.mem[disp_mem]) };

    trace!("reg[F] = {}", cpu.reg[0xF])
}
