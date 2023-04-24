use crate::flags::*;
use crate::opcodes::match_opcode;
use crate::utils::is_page_crossed;

pub struct Memory {
    state: [u8; 0xFFFF]
}
impl Memory {
    pub fn read(&self, addr: u16) -> u8 {
        self.state[addr as usize]
    }
    pub fn read_u16(&self, addr: u16) -> u16 {
        u16::from_le_bytes([
            self.read(addr),
            self.read(addr+1)
        ])
    }
    pub fn write(&mut self, addr: u16, value: u8) {
        self.state[addr as usize] = value;
    }
    pub fn write_u16(&mut self, addr: u16, value: u16) {
        let bytes = value.to_le_bytes();
        self.write(addr, bytes[0]);
        self.write(addr + 1, bytes[1]);
    }
    fn load<const S: usize>(&mut self, addr: u16, code: &[u8; S]) {
        self.state[addr as usize..addr as usize + S].copy_from_slice(code);
    }
}
impl Default for Memory {
    fn default() -> Self {
        Memory { state: [0; 0xFFFF] }
    }
}

pub enum AddrMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Implied,
    Immediate,
    Indirect,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY
}
impl AddrMode {
    pub fn get_size(&self) -> u16 {
        match self {
            Self::Absolute => 2,
            Self::AbsoluteX => 2,
            Self::AbsoluteY => 2,
            Self::Implied => 0,
            Self::Immediate => 1,
            Self::Indirect => 2,
            Self::Relative => 1,
            Self::ZeroPage => 1,
            Self::ZeroPageX => 1,
            Self::ZeroPageY => 1,
        }
    }
}

// a number of extra cycles should be returned
pub type Instruction = fn(&mut CPU, Option<u16>) -> u8;

#[derive(Default)]
pub struct CPU {
    // emulator only flag set by BRK ins
    pub running: bool,
    // emulator only flag set when a page is crossed during addressing
    // that might result in an extra cpu cycle
    pub addr_page_crossed: bool,

    // 6502 regs and flags
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    pub pc: u16,
    pub status: u8,
    pub memory: Memory
}
impl CPU {
    pub fn load<const S: usize>(&mut self, addr: u16, code: &[u8; S]) {
        self.memory.load::<S>(addr, code);
    }
    pub fn load_executable<const S: usize>(&mut self, addr: u16, code: &[u8; S]) {
        self.load::<S>(addr, code);
        self.pc = addr;
        self.running = true;
    }
    pub fn step(&mut self) -> u8 {
        // return cycles taken
        let code = self.memory.read(self.pc);
        let (ins, mode, cycles) = match_opcode(code);
        let extra_cycles = self.op_execute(ins, mode);
        cycles + extra_cycles
    }
    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            self.step();
        }
    }
    fn get_op_addr(&mut self, mode: &AddrMode) -> u16 {
        self.addr_page_crossed = false;
        match mode {
            AddrMode::Absolute => self.memory.read_u16(self.pc),
            AddrMode::AbsoluteX => {
                let base = self.memory.read_u16(self.pc);
                let addr = base.wrapping_add(self.reg_x as u16);
                if is_page_crossed(base, addr) { self.addr_page_crossed = true }
                addr
            },
            AddrMode::AbsoluteY => {
                let base = self.memory.read_u16(self.pc);
                let addr = base.wrapping_add(self.reg_y as u16);
                if is_page_crossed(base, addr) { self.addr_page_crossed = true }
                addr
            },
            AddrMode::Implied => panic!("Invalid addr mode!"),
            AddrMode::Immediate => self.pc,
            AddrMode::Indirect => self.memory.read_u16(self.pc),
            AddrMode::Relative => self.pc,
            AddrMode::ZeroPage => self.memory.read(self.pc) as u16,
            AddrMode::ZeroPageX => self.memory.read(self.pc).wrapping_add(self.reg_x) as u16,
            AddrMode::ZeroPageY => self.memory.read(self.pc).wrapping_add(self.reg_y) as u16,
        }
    }
    fn op_execute(&mut self, ins: Instruction, mode: AddrMode) -> u8 {
        // returns a number of extra cycles
        self.pc += 1;
        let addr = match mode {
            AddrMode::Implied => None,
            _ => Some(self.get_op_addr(&mode))
        };
        self.pc += mode.get_size();
        ins(self, addr)
    }
    pub fn check_flag(&self, flag: u8) -> bool {
        self.status & flag != 0
    }
    pub fn set_flag(&mut self, flag: u8) {
        self.status |= flag;
    }
    pub fn clear_flag(&mut self, flag: u8) {
        self.status &= !flag;
    }
    pub fn update_zero_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.set_flag(ZERO_FLAG);
        } else {
            self.clear_flag(ZERO_FLAG);
        }
        if result & 0b1000_0000 != 0 {
            self.set_flag(NEGATIVE_FLAG);
        } else {
            self.clear_flag(NEGATIVE_FLAG);
        }
    }
}
