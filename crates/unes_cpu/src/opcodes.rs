use crate::cpu::{AddrMode, CPU, Instruction};
use crate::flags::*;

pub fn match_opcode(code: u8) -> (Instruction, AddrMode) {
    match code {
        // brk
        0x00 => (brk, AddrMode::Implied),
        // bne
        0xD0 => (bne, AddrMode::Relative),
        // inx
        0xE8 => (inx, AddrMode::Implied),
        // lda
        0xAD => (lda, AddrMode::Absolute),
        0xBD => (lda, AddrMode::AbsoluteX),
        0xB9 => (lda, AddrMode::AbsoluteY),
        0xA9 => (lda, AddrMode::Immediate),
        0xA5 => (lda, AddrMode::ZeroPage),
        0xB5 => (lda, AddrMode::ZeroPageX),
        // ldx
        0xB6 => (ldx, AddrMode::ZeroPageY),
        // tax
        0xAA => (tax, AddrMode::Implied),
        _ => panic!("{:?} opcode is not supported!", code)
    }
}

fn brk(cpu: &mut CPU, _data: Option<u8>) {
    cpu.running = false;
}
fn bne(cpu: &mut CPU, data: Option<u8>) {
    if !cpu.check_flag(ZERO_FLAG) {
        let offset = data.expect("Invalid BNE operand!") as i8;
        cpu.pc = cpu.pc.wrapping_add(offset as u16);
    }
}
fn inx(cpu: &mut CPU, _data: Option<u8>) {
    cpu.reg_x = cpu.reg_x.wrapping_add(1);
    cpu.update_zero_negative_flags(cpu.reg_x);
}
fn jmp(cpu: &mut CPU, data: Option<u8>) {
    
}
fn lda(cpu: &mut CPU, data: Option<u8>) {
    cpu.reg_a = data.expect("Invalid LDA operand!");
    cpu.update_zero_negative_flags(cpu.reg_a);
}
fn ldx(cpu: &mut CPU, data: Option<u8>) {
    cpu.reg_x = data.expect("Invalid LDX operand!");
    cpu.update_zero_negative_flags(cpu.reg_x);
}
fn tax(cpu: &mut CPU, _data: Option<u8>) {
    cpu.reg_x = cpu.reg_a;
    cpu.update_zero_negative_flags(cpu.reg_x);
}