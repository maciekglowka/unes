use unes_cpu::CPU;
use unes_cpu::flags::*;

fn main() {
        let mut cpu = CPU::new();
        cpu.load_executable::<2>(0x80f0, &[0xd0, 0x20]);
        cpu.set_flag(ZERO_FLAG, false);
        let cycles = cpu.step();
        println!("{:x}", cpu.pc);
}