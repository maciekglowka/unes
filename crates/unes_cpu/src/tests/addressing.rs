#[cfg(test)]
mod tests {
    use crate::CPU;
    use crate::flags::*;

    #[test]
    fn test_lda_immediate() {
        let mut cpu = CPU::default();
        cpu.load_executable::<3>(0x8000, &[0xa9, 0x05, 0x00]);
        cpu.run();
        assert!(cpu.reg_a == 0x05);
    }
    #[test]
    fn test_lda_zero_page() {
        let mut cpu = CPU::default();
        // load executable
        cpu.load_executable::<3>(0x8000, &[0xa5, 0x05, 0x00]);
        // load operand at 0x05
        cpu.load::<1>(0x05, &[0xaa]);
        cpu.run();
        assert!(cpu.reg_a == 0xaa);
    }
    #[test]
    fn test_lda_zero_page_x() {
        let mut cpu = CPU::default();
        // load executable
        cpu.load_executable::<3>(0x8000, &[0xb5, 0x03, 0x00]);
        // load operand at 0x05
        cpu.load::<1>(0x05, &[0xaa]);
        cpu.reg_x = 2;
        cpu.run();
        assert!(cpu.reg_a == 0xaa);
    }
    #[test]
    fn test_ldx_zero_page_y() {
        let mut cpu = CPU::default();
        // load executable
        cpu.load_executable::<3>(0x8000, &[0xb6, 0x03, 0x00]);
        // load operand at 0x07
        cpu.load::<1>(0x07, &[0xf0]);
        cpu.reg_y = 4;
        cpu.run();
        assert!(cpu.reg_x == 0xf0);
    }
    #[test]
    fn test_bne_relative_positive() {
        let mut cpu = CPU::default();
        // load executable
        cpu.load_executable::<2>(0x8000, &[0xd0, 0x03]);
        cpu.clear_flag(ZERO_FLAG);
        cpu.step();
        // expected pc = 0x8000 + 0x02 + 0x03
        assert!(cpu.pc == 0x8005);
    }
    #[test]
    fn test_bne_relative_negative() {
        let mut cpu = CPU::default();
        // load executable
        cpu.load_executable::<2>(0x8000, &[0xd0, 0xf8]);
        cpu.clear_flag(ZERO_FLAG);
        cpu.step();
        // expected negative offset by 0x06 from the instruction start
        assert!(cpu.pc == 0x7ffa);
    }
    #[test]
    fn test_lda_absolute() {
        let mut cpu = CPU::default();
        // load executable
        cpu.load_executable::<4>(0x8000, &[0xad, 0x05, 0x10, 0x00]);
        // load operand at 0x1005
        cpu.load::<1>(0x1005, &[0xae]);
        cpu.run();
        assert!(cpu.reg_a == 0xae);
    }
    #[test]
    fn test_lda_absolute_x() {
        let mut cpu = CPU::default();
        // load executable
        cpu.load_executable::<4>(0x8000, &[0xbd, 0x03, 0x10, 0x00]);
        // load operand at 0x1005
        cpu.load::<1>(0x1005, &[0xaf]);
        cpu.reg_x = 2;
        cpu.run();
        assert!(cpu.reg_a == 0xaf);
    }
    #[test]
    fn test_lda_absolute_y() {
        let mut cpu = CPU::default();
        // load executable
        cpu.load_executable::<4>(0x8000, &[0xb9, 0x03, 0x10, 0x00]);
        // load operand at 0x1006
        cpu.load::<1>(0x1006, &[0xaf]);
        cpu.reg_y = 3;
        cpu.run();
        assert!(cpu.reg_a == 0xaf);
    }
    #[test]
    fn test_jmp_indirect() {
        let mut cpu = CPU::default();
        // load executable
        cpu.load_executable::<4>(0x8000, &[0x6c, 0x05, 0x10, 0x00]);
        // load addr at 0x1005
        cpu.load::<2>(0x1005, &[0xaf, 0x10]);
        cpu.step();
        assert!(cpu.pc == 0x10af);
    }
}