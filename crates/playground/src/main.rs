use unes_cpu;

fn main() {
    // let code = [0xa9, 0x05, 0x00];
    // let mut cpu = unes_cpu::CPU::default();
    // cpu.load::<3>(0x8000, &code);

    // while cpu.running {
    //     cpu.step();
    //     println!("{:?}", cpu.reg_a);
    // }
        let mut cpu = unes_cpu::CPU::default();
        // load executable
        cpu.load_executable::<2>(0x8000, &[0xd0, 0xf8]);
        cpu.unset_flag(unes_cpu::flags::ZERO_FLAG);
        cpu.step();
        // expected negative offset by 0x06 from the instruction start
        println!("{:x}", cpu.pc);
}