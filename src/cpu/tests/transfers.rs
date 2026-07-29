use crate::cpu::{CPU, status::*};

mod tax {
    use super::*;

    #[test]
    fn test_0xaa_tax_copy_a_to_x() {
        let mut cpu = CPU::new();

        cpu.load_and_run(&[0xA9, 0x05, 0xAA, 0x00]);

        assert_eq!(cpu.register_x, 0x05);
        assert_eq!(cpu.register_a, cpu.register_x);
        assert_eq!(cpu.status & ZERO, 0);
        assert_eq!(cpu.status & NEGATIVE, 0);
    }

    #[test]
    fn test_0xaa_tax_copy_a_to_x_zero() {
        let mut cpu = CPU::new();

        cpu.load_and_run(&[0xA9, 0x00, 0xAA, 0x00]);

        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.register_a, cpu.register_x);
        assert_eq!(cpu.status & ZERO, ZERO);
        assert_eq!(cpu.status & NEGATIVE, 0);
    }

    #[test]
    fn test_0xaa_tax_copy_a_to_x_negative() {
        let mut cpu = CPU::new();

        cpu.load_and_run(&[0xA9, 0x80, 0xAA, 0x00]);

        assert_eq!(cpu.register_x, 0x80);
        assert_eq!(cpu.register_a, cpu.register_x);
        assert_eq!(cpu.status & ZERO, 0);
        assert_eq!(cpu.status & NEGATIVE, NEGATIVE);
    }
}
