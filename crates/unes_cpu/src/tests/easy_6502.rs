// tests based on code samples from https://skilldrick.github.io/easy6502/

#[cfg(test)]
mod tests {
    use crate::CPU;
    use crate::flags::*;

    #[test]
    fn test_first() {
        let code = [0xa9, 0x01, 0x8d, 0x00, 0x02, 0xa9, 0x05, 0x8d, 0x01, 0x02, 0xa9, 0x08, 0x8d, 0x02, 0x02];
        let mut cpu = CPU::new();
        cpu.load_executable::<15>(0x0600, &code);
        cpu.run();
        assert!(cpu.reg_a == 0x08);
        assert!(cpu.reg_x == 0x00);
        assert!(cpu.reg_y == 0x00);
        assert!(cpu.pc == 0x0610);
        assert!(cpu.status == 0b0011_0000);
        assert!(cpu.sp == 0xff);
    }
   
    #[test]
    fn test_flags() {
        let code = [0xa9, 0xc0, 0xaa, 0xe8, 0x69, 0xc4, 0x00];
        let mut cpu = CPU::new();
        cpu.load_executable::<7>(0x0600, &code);
        cpu.run();
        assert!(cpu.reg_a == 0x84);
        assert!(cpu.reg_x == 0xc1);
        assert!(cpu.reg_y == 0x00);
        assert!(cpu.pc == 0x0607);
        assert!(cpu.status == 0b1011_0001);
        assert!(cpu.sp == 0xff);
    }
    #[test]
    fn test_branching() {
        let code = [0xa2, 0x08, 0xca, 0x8e, 0x00, 0x02, 0xe0, 0x03, 0xd0, 0xf8, 0x8e, 0x01, 0x02, 0x00];
        let mut cpu = CPU::new();
        cpu.load_executable::<14>(0x0600, &code);
        cpu.run();
        assert!(cpu.reg_a == 0x00);
        assert!(cpu.reg_x == 0x03);
        assert!(cpu.reg_y == 0x00);
        assert!(cpu.pc == 0x060e);
        assert!(cpu.status == 0b0011_0011);
        assert!(cpu.sp == 0xff);
    }
}