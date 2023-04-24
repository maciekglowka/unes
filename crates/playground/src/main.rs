use unes_cpu;

fn main() {
        let mut cpu = unes_cpu::CPU::default();
        // load executable
        cpu.load_executable::<3>(0x8000, &[0xb1, 0x04, 0x00]);
        // load memory table entry at 0x0004
        cpu.load::<2>(0x0004, &[0x34, 0x12]);
        // load operand at 0x1236
        cpu.load::<1>(0x1236, &[0xac]);
        cpu.reg_y = 2;
        cpu.step();
        // assert!(cpu.reg_a == 0xac);
        println!("{:x}", cpu.reg_a);
}