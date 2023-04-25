#[cfg(test)]
mod tests {
    use crate::CPU;

    #[test]
    fn test_lda_tax_inx() {
        let mut cpu = CPU::new();
        cpu.load_executable::<5>(0x8000, &[0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        cpu.run();
        assert!(cpu.reg_x == 0xc1);
    }
}