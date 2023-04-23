#[cfg(test)]
mod tests {
    use crate::CPU;
    use crate::flags::*;

    #[test]
    fn test_inx_0x20() {
        let mut cpu = CPU::default();
        cpu.reg_x = 0x20;
        cpu.load_executable::<2>(0x8000, &[0xe8, 0x00]);
        cpu.run();
        assert!(cpu.reg_x == 0x21);
        assert!(!cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG)); 
    }
    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::default();
        cpu.reg_x = 0xff;
        cpu.load_executable::<2>(0x8000, &[0xe8, 0x00]);
        cpu.run();
        assert!(cpu.reg_x == 0x00);
        assert!(cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG)); 
    }
    #[test]
    fn test_lda_immediate() {
        let mut cpu = CPU::default();
        cpu.load_executable::<3>(0x8000, &[0xa9, 0x05, 0x00]);
        cpu.run();
        assert!(cpu.reg_a == 0x05);
        assert!(!cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG));
    }
    #[test]
    fn test_lda_zero() {
        let mut cpu = CPU::default();
        cpu.load_executable::<3>(0x8000, &[0xa9, 0x00, 0x00]);
        cpu.run();
        assert!(cpu.reg_a == 0x00);
        assert!(cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG));
    }
    #[test]
    fn test_tax_zero() {
        let mut cpu = CPU::default();
        cpu.reg_a = 0;
        cpu.reg_x = 10;
        cpu.load_executable::<2>(0x8000, &[0xaa, 0x00]);
        cpu.run();
        assert!(cpu.reg_x == 0x00);
        assert!(cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG)); 
    }
    #[test]
    fn test_tax_0x20() {
        let mut cpu = CPU::default();
        cpu.reg_a = 0x20;
        cpu.reg_x = 0x10;
        cpu.load_executable::<2>(0x8000, &[0xaa, 0x00]);
        cpu.run();
        assert!(cpu.reg_x == 0x20);
        assert!(!cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG)); 
    }
}