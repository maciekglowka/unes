// tests based on code samples from https://skilldrick.github.io/easy6502/

#[cfg(test)]
mod tests {
    use crate::CPU;
    use crate::flags::*;

    #[test]
    fn test_first_sample() {
        let code = [0xa9, 0x01, 0x8d, 0x00, 0x02, 0xa9, 0x05, 0x8d, 0x01, 0x02, 0xa9, 0x08, 0x8d, 0x02, 0x02];
        let mut cpu = CPU::default();
        cpu.load_executable::<15>(0x0600, &code);
        cpu.run();
        assert!(cpu.reg_a == 0x08);
        assert!(cpu.reg_x == 0x00);
        assert!(cpu.reg_y == 0x00);
        assert!(cpu.pc == 0x0610);
    }
}