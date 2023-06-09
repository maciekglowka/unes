#[cfg(test)]
mod tests {
    use crate::CPU;
    use crate::flags::*;

    #[test]
    fn test_adc_basic() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0x20;
        cpu.load_executable::<2>(0x8000, &[0x69, 0x10]);
        cpu.run();
        assert!(cpu.reg_a == 0x30);
        assert!(!cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG));
        assert!(!cpu.check_flag(OVERFLOW_FLAG));
    }
    #[test]
    fn test_adc_zero_flag() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0x00;
        cpu.load_executable::<2>(0x8000, &[0x69, 0x00]);
        cpu.run();
        assert!(cpu.reg_a == 0x00);
        assert!(cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG));
        assert!(!cpu.check_flag(OVERFLOW_FLAG));
    }
    #[test]
    fn test_adc_carry_flag() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0x11;
        cpu.load_executable::<4>(0x8000, &[0x69, 0xf0, 0x69, 0x05]);

        // 1st
        cpu.step();
        // overflowed result
        assert!(cpu.reg_a == 0x01);
        // carry set
        assert!(cpu.check_flag(CARRY_FLAG)); 
        assert!(!cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG));
        assert!(!cpu.check_flag(OVERFLOW_FLAG));

        // 2nd
        cpu.step();
        // result incl. +1 from CARRY
        assert!(cpu.reg_a == 0x07);
        // carry cleared
        assert!(!cpu.check_flag(CARRY_FLAG)); 
        assert!(!cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG));
        assert!(!cpu.check_flag(OVERFLOW_FLAG));
    }
    #[test]
    fn test_adc_carry_flag_high() {
        // test that 0xff + carry won't cause overflow
        let mut cpu = CPU::new();
        cpu.reg_a = 0xff;
        cpu.load_executable::<5>(0x8000, &[0x69, 0xff, 0x69, 0xff, 0x00]);
        cpu.run();
        assert!(cpu.reg_a == 0xfe);
        assert!(cpu.check_flag(CARRY_FLAG));
        assert!(cpu.check_flag(NEGATIVE_FLAG));
        assert!(!cpu.check_flag(OVERFLOW_FLAG));
    }
    #[test]
    fn test_adc_overflow_flag_positive() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0x50;
        cpu.load_executable::<3>(0x8000, &[0x69, 0x50, 0x00]);
        cpu.step();
        assert!(cpu.reg_a == 0xa0);
        // overflow and negative should be set
        assert!(cpu.check_flag(OVERFLOW_FLAG));
        assert!(cpu.check_flag(NEGATIVE_FLAG));
        // carry is clear
        assert!(!cpu.check_flag(CARRY_FLAG));
    }
    #[test]
    fn test_adc_overflow_flag_negative() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0xd0;
        cpu.load_executable::<3>(0x8000, &[0x69, 0x90, 0x00]);
        cpu.step();
        assert!(cpu.reg_a == 0x60);
        // overflow and carry should be set
        assert!(cpu.check_flag(OVERFLOW_FLAG));
        assert!(cpu.check_flag(CARRY_FLAG));
        // negative is clear
        assert!(!cpu.check_flag(NEGATIVE_FLAG));
    }
    #[test]
    fn test_bne_success() {
        let mut cpu = CPU::new();
        cpu.load_executable::<2>(0x8000, &[0xd0, 0x05]);
        cpu.set_flag(ZERO_FLAG, false);
        let cycles = cpu.step();
        assert!(cpu.pc == 0x8007);
        assert!(cycles == 3);
    }
    #[test]
    fn test_bne_fail() {
        let mut cpu = CPU::new();
        cpu.load_executable::<2>(0x8000, &[0xd0, 0x05]);
        cpu.set_flag(ZERO_FLAG, true);
        let cycles = cpu.step();
        assert!(cpu.pc == 0x8002);
        assert!(cycles == 2);
    }
    #[test]
    fn test_bne_success_new_page() {
        let mut cpu = CPU::new();
        cpu.load_executable::<2>(0x80f0, &[0xd0, 0x20]);
        cpu.set_flag(ZERO_FLAG, false);
        let cycles = cpu.step();
        assert!(cpu.pc == 0x8112);
        assert!(cycles == 4);
    }
    #[test]
    fn test_cpx_eq() {
        let mut cpu = CPU::new();
        cpu.reg_x = 0x21;
        cpu.load::<1>(0x1005, &[0x21]);
        cpu.load_executable::<4>(0x8000, &[0xEC, 0x05, 0x10, 0x00]);
        cpu.run();
        assert!(cpu.check_flag(CARRY_FLAG));
        assert!(cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG));
    }
    #[test]
    fn test_cpx_x_gt() {
        let mut cpu = CPU::new();
        cpu.reg_x = 0x25;
        cpu.load::<1>(0x1005, &[0x21]);
        cpu.load_executable::<4>(0x8000, &[0xEC, 0x05, 0x10, 0x00]);
        cpu.run();
        assert!(cpu.check_flag(CARRY_FLAG));
        assert!(!cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG));
    }
    #[test]
    fn test_cpx_x_lt() {
        let mut cpu = CPU::new();
        cpu.reg_x = 0x25;
        cpu.load::<1>(0x1005, &[0x31]);
        cpu.load_executable::<4>(0x8000, &[0xEC, 0x05, 0x10, 0x00]);
        cpu.run();
        assert!(!cpu.check_flag(CARRY_FLAG));
        assert!(!cpu.check_flag(ZERO_FLAG));
        assert!(cpu.check_flag(NEGATIVE_FLAG));
    }
    #[test]
    fn test_dex() {
        let mut cpu = CPU::new();
        cpu.reg_x = 0x20;
        cpu.load_executable::<2>(0x8000, &[0xCA, 0x00]);
        cpu.run();
        assert!(cpu.reg_x == 0x1f);
        assert!(!cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG)); 
    }
    #[test]
    fn test_dex_overflow() {
        let mut cpu = CPU::new();
        cpu.reg_x = 0x00;
        cpu.load_executable::<2>(0x8000, &[0xCA, 0x00]);
        cpu.run();
        assert!(cpu.reg_x == 0xff);
        assert!(!cpu.check_flag(ZERO_FLAG));
        assert!(cpu.check_flag(NEGATIVE_FLAG)); 
    }
    #[test]
    fn test_inx_0x20() {
        let mut cpu = CPU::new();
        cpu.reg_x = 0x20;
        cpu.load_executable::<2>(0x8000, &[0xe8, 0x00]);
        cpu.run();
        assert!(cpu.reg_x == 0x21);
        assert!(!cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG)); 
    }
    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.reg_x = 0xff;
        cpu.load_executable::<2>(0x8000, &[0xe8, 0x00]);
        cpu.run();
        assert!(cpu.reg_x == 0x00);
        assert!(cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG)); 
    }
    #[test]
    fn test_lda_immediate() {
        let mut cpu = CPU::new();
        cpu.load_executable::<3>(0x8000, &[0xa9, 0x05, 0x00]);
        cpu.run();
        assert!(cpu.reg_a == 0x05);
        assert!(!cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG));
    }
    #[test]
    fn test_lda_zero_flag() {
        // immediate addr, zero flag set
        let mut cpu = CPU::new();
        cpu.load_executable::<3>(0x8000, &[0xa9, 0x00, 0x00]);
        let cycles = cpu.step();
        assert!(cpu.reg_a == 0x00);
        assert!(cycles == 2);
        assert!(cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG));
    }
    #[test]
    fn test_lda_page_cross() {
        let mut cpu = CPU::new();
        cpu.load_executable::<4>(0x8000, &[0xbd, 0xfe, 0x90, 0x00]);
        cpu.load::<1>(0x911e, &[0xaf]);
        cpu.reg_x = 0x20;
        let cycles = cpu.step();
        assert!(cycles == 5);
        assert!(cpu.reg_a == 0xaf);
    }
    #[test]
    fn test_tax_zero() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0;
        cpu.reg_x = 10;
        cpu.load_executable::<2>(0x8000, &[0xaa, 0x00]);
        cpu.run();
        assert!(cpu.reg_x == 0x00);
        assert!(cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG)); 
    }
    #[test]
    fn test_sta_absolute() {
        let mut cpu = CPU::new();
        cpu.load_executable::<3>(0x8000, &[0x8d, 0x05, 0xf0]);
        cpu.reg_a = 0xfd;
        let cycles = cpu.step();
        assert!(cpu.memory.read(0xf005) == 0xfd);
        assert!(cycles == 4);
    }
    #[test]
    fn test_stx_zero_page() {
        let mut cpu = CPU::new();
        cpu.load_executable::<3>(0x8000, &[0x86, 0x05, 0x00]);
        cpu.reg_x = 0xfd;
        let cycles = cpu.step();
        assert!(cpu.memory.read(0x0005) == 0xfd);
        assert!(cycles == 3);
    }
    #[test]
    fn test_tax_0x20() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0x20;
        cpu.reg_x = 0x10;
        cpu.load_executable::<2>(0x8000, &[0xaa, 0x00]);
        cpu.run();
        assert!(cpu.reg_x == 0x20);
        assert!(!cpu.check_flag(ZERO_FLAG));
        assert!(!cpu.check_flag(NEGATIVE_FLAG)); 
    }
}