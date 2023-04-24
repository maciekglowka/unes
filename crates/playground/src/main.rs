use unes_cpu;

fn main() {
        let mut cpu = unes_cpu::CPU::default();
        // load executable
        cpu.load_executable::<4>(0x8000, &[0x6c, 0x20, 0x01, 0x00]);
        // load addr at 0x0120
        cpu.load::<2>(0x0120, &[0xfc, 0xba]);
        cpu.step();
        println!("{:x}", cpu.pc);
}