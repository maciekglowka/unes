use crate::cpu::{AddrMode, CPU, Instruction};
use crate::flags::*;
use crate::utils::is_page_crossed;

pub fn match_opcode(code: u8) -> (Instruction, AddrMode, u8) {
    // ins, mode, base cycles
    match code {
        // adc
        0x69 => (adc, AddrMode::Immediate, 2),
        // brk
        0x00 => (brk, AddrMode::Implied, 7),
        // bne
        0xD0 => (bne, AddrMode::Relative, 2),
        // cpx
        0xEC => (cpx, AddrMode::Absolute, 4),
        0xE0 => (cpx, AddrMode::Immediate, 2),
        0xE4 => (cpx, AddrMode::ZeroPage, 3),
        // dex
        0xCA => (dex, AddrMode::Implied, 2),
        // inx
        0xE8 => (inx, AddrMode::Implied, 2),
        // jmp
        0x6C => (jmp, AddrMode::Indirect, 5),
        // lda
        0xAD => (lda, AddrMode::Absolute, 4),
        0xBD => (lda, AddrMode::AbsoluteX, 4),
        0xB9 => (lda, AddrMode::AbsoluteY, 4),
        0xA9 => (lda, AddrMode::Immediate, 2),
        0xA1 => (lda, AddrMode::IndirectX, 6),
        0xB1 => (lda, AddrMode::IndirectY, 5),
        0xA5 => (lda, AddrMode::ZeroPage, 3),
        0xB5 => (lda, AddrMode::ZeroPageX, 4),
        // ldx
        0xAE => (ldx, AddrMode::Absolute, 4),
        0xBE => (ldx, AddrMode::AbsoluteY, 4),
        0xA2 => (ldx, AddrMode::Immediate, 2),
        0xA6 => (ldx, AddrMode::ZeroPage, 3),
        0xB6 => (ldx, AddrMode::ZeroPageY, 4),
        // sta
        0x8D => (sta, AddrMode::Absolute, 4),
        0x9D => (sta, AddrMode::AbsoluteX, 5),
        0x99 => (sta, AddrMode::AbsoluteY, 5),
        0x85 => (sta, AddrMode::ZeroPage, 3),
        0x95 => (sta, AddrMode::ZeroPageX, 4),
        //stx
        0x8E => (stx, AddrMode::Absolute, 4),
        0x86 => (stx, AddrMode::ZeroPage, 3),
        0x96 => (stx, AddrMode::ZeroPageY, 4),
        // tax
        0xAA => (tax, AddrMode::Implied, 2),
        _ => panic!("{:?} opcode is not supported!", code)
    }
}

fn adc(cpu: &mut CPU, addr: Option<u16>) -> u8 {
    let operand = cpu.memory.read(
        addr.expect("Invalid ADC operand!")
    );
    let (mut res, carry) = cpu.reg_a.overflowing_add(operand);
    // if prev carry add 1
    if cpu.check_flag(CARRY_FLAG) { res += 1 };
    // set new carry
    cpu.set_flag(CARRY_FLAG, carry);
    // set overflow
    cpu.set_flag(
        OVERFLOW_FLAG,
        (operand ^ res) & (cpu.reg_a ^ res) & 0x80 != 0
    );
    // assign reg value
    cpu.reg_a = res;

    cpu.update_zero_negative_flags(cpu.reg_a);
    if cpu.addr_page_crossed { 1 } else { 0 }
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
        let before = cpu.pc;
        cpu.pc = cpu.pc.wrapping_add(offset as u16);
        if is_page_crossed(before, cpu.pc) { return 2 } else { return 1 }
    }
    0
}
fn cpx(cpu: &mut CPU, addr: Option<u16>) -> u8 {
    let val = cpu.memory.read(
        addr.expect("Invalid CPX operand!")
    );
    let res = cpu.reg_x.wrapping_sub(val);
    cpu.set_flag(CARRY_FLAG, cpu.reg_x >= val);
    cpu.update_zero_negative_flags(res);
    0
}
fn dex(cpu: &mut CPU, _addr: Option<u16>) -> u8 {
    cpu.reg_x = cpu.reg_x.wrapping_sub(1);
    cpu.update_zero_negative_flags(cpu.reg_x);
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
fn sta(cpu: &mut CPU, addr: Option<u16>) -> u8 {
    cpu.memory.write(
        addr.expect("Invalid LDA operand!"),
        cpu.reg_a
    );
    0
}
fn stx(cpu: &mut CPU, addr: Option<u16>) -> u8 {
    cpu.memory.write(
        addr.expect("Invalid STX operand!"),
        cpu.reg_x
    );
    0
}
fn tax(cpu: &mut CPU, _addr: Option<u16>) -> u8 {
    cpu.reg_x = cpu.reg_a;
    cpu.update_zero_negative_flags(cpu.reg_x);
    0
}