use crate::cpu::{AddrMode, CPU, Instruction};
use crate::flags::*;

pub fn match_opcode(code: u8) -> (Instruction, AddrMode, u8) {
    // ins, mode, base cycles
    match code {
        // brk
        0x00 => (brk, AddrMode::Implied, 7),
        // bne
        0xD0 => (bne, AddrMode::Relative, 2),
        // inx
        0xE8 => (inx, AddrMode::Implied, 2),
        // jmp
        0x6C => (jmp, AddrMode::Indirect, 5),
        // lda
        0xAD => (lda, AddrMode::Absolute, 4),
        0xBD => (lda, AddrMode::AbsoluteX, 4),
        0xB9 => (lda, AddrMode::AbsoluteY, 4),
        0xA9 => (lda, AddrMode::Immediate, 2),
        0xA5 => (lda, AddrMode::ZeroPage, 3),
        0xB5 => (lda, AddrMode::ZeroPageX, 4),
        // ldx
        0xB6 => (ldx, AddrMode::ZeroPageY, 4),
        // tax
        0xAA => (tax, AddrMode::Implied, 2),
        _ => panic!("{:?} opcode is not supported!", code)
    }
}

fn brk(cpu: &mut CPU, _addr: Option<u16>) -> u8 {
    cpu.running = false;
    0
}
fn bne(cpu: &mut CPU, addr: Option<u16>) -> u8 {
    if !cpu.check_flag(ZERO_FLAG) {
        let offset = cpu.memory.read(
            addr.expect("Invalid BNE operand!")
        ) as i8;
        cpu.pc = cpu.pc.wrapping_add(offset as u16);
    }
    0
}
fn inx(cpu: &mut CPU, _addr: Option<u16>) -> u8 {
    cpu.reg_x = cpu.reg_x.wrapping_add(1);
    cpu.update_zero_negative_flags(cpu.reg_x);
    0
}
fn jmp(cpu: &mut CPU, addr: Option<u16>) -> u8 {
    cpu.pc = cpu.memory.read_u16(
        addr.expect("Invalid JMP operand!")
    );
    0
}
fn lda(cpu: &mut CPU, addr: Option<u16>) -> u8 {
    cpu.reg_a = cpu.memory.read(
        addr.expect("Invalid LDA operand!")
    );
    cpu.update_zero_negative_flags(cpu.reg_a);
    if cpu.addr_page_crossed { 1 } else { 0 }
}
fn ldx(cpu: &mut CPU, addr: Option<u16>) -> u8 {
    cpu.reg_x = cpu.memory.read(
        addr.expect("Invalid LDX operand!")
    );
    cpu.update_zero_negative_flags(cpu.reg_x);
    if cpu.addr_page_crossed { 1 } else { 0 }
}
fn tax(cpu: &mut CPU, _addr: Option<u16>) -> u8 {
    cpu.reg_x = cpu.reg_a;
    cpu.update_zero_negative_flags(cpu.reg_x);
    0
}