use crate::cpu::Mem;
use crate::cpu::{CPU, status::*};

mod adc {
    use super::*;

    #[test]
    fn test_0x69_adc_immediate() {
        let mut cpu = CPU::new();

        cpu.load_and_run(&[0xA9, 0x01, 0x69, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x11);
        assert_eq!(cpu.status & NEGATIVE, 0);
        assert_eq!(cpu.status & CARRY, 0);
    }

    #[test]
    fn test_0x69_positive_overflow_sets_overflow_flag() {
        let mut cpu = CPU::new();
        let max_positive_u8 = 0x7F; // 127
        let min_positive_u8 = 0x01; // 1

        cpu.load_and_run(&[0xA9, max_positive_u8, 0x69, min_positive_u8, 0x00]);

        assert_eq!(cpu.register_a, 0x80);
        assert_eq!(cpu.status & NEGATIVE, NEGATIVE);
        assert_eq!(cpu.status & OVERFLOW, OVERFLOW);
        assert_eq!(cpu.status & CARRY, 0);
    }

    #[test]
    fn test_0x69_negative_overflow_sets_overflow_flag() {
        let mut cpu = CPU::new();
        let max_negative_u8 = 0x80; // -128
        let min_negative_u8 = 0xFF; // -1

        cpu.load_and_run(&[0xA9, max_negative_u8, 0x69, min_negative_u8, 0x00]);

        assert_eq!(cpu.register_a, 0x7F);
        assert_eq!(cpu.status & NEGATIVE, 0);
        assert_eq!(cpu.status & OVERFLOW, OVERFLOW);
        assert_eq!(cpu.status & CARRY, CARRY);
    }

    #[test]
    fn test_0x65_adc_zeropage() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(&[0xA9, 0x01, 0x65, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x56);
    }

    #[test]
    fn test_0x65_adc_zeropage_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x11, 0x55);

        cpu.load(&[0xA9, 0x01, 0x75, 0x10, 0x00]);
        cpu.reset();
        cpu.register_x = 0x01;
        cpu.run();

        assert_eq!(cpu.register_a, 0x56);
    }

    #[test]
    fn test_0x6d_adc_absolute() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x8010, 0x55);

        cpu.load_and_run(&[0xA9, 0x01, 0x6D, 0x10, 0x80, 0x00]);

        assert_eq!(cpu.register_a, 0x56);
    }

    #[test]
    fn test_0x7d_adc_absolute_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x8011, 0x55);

        cpu.load(&[0xA9, 0x01, 0x7D, 0x10, 0x80, 0x00]);
        cpu.reset();
        cpu.register_x = 0x01;
        cpu.run();

        assert_eq!(cpu.register_a, 0x56);
    }

    #[test]
    fn test_0x79_adc_absolute_y() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x8011, 0x55);

        cpu.load(&[0xA9, 0x01, 0x79, 0x10, 0x80, 0x00]);
        cpu.reset();
        cpu.register_y = 0x01;
        cpu.run();

        assert_eq!(cpu.register_a, 0x56);
    }

    #[test]
    fn test_0x61_adc_indirect_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x81, 0xF0);
        cpu.mem_write(0x82, 0x0F);
        cpu.mem_write(0x0FF0, 0x16);

        cpu.load(&[0xA9, 0x01, 0x61, 0x80, 0x00]);
        cpu.reset();
        cpu.register_x = 0x01;
        cpu.run();

        assert_eq!(cpu.register_a, 0x17);
    }

    #[test]
    fn test_0x71_adc_indirect_y() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x80, 0xF0);
        cpu.mem_write(0x81, 0x0F);
        cpu.mem_write(0x0FF1, 0x16);

        cpu.load(&[0xA9, 0x01, 0x71, 0x80, 0x00]);
        cpu.reset();
        cpu.register_y = 0x01;
        cpu.run();

        assert_eq!(cpu.register_a, 0x17);
    }
}

#[test]
fn test_5_ops_working_together() {
    let mut cpu = CPU::new();

    cpu.load_and_run(&[0xA9, 0xc0, 0xAA, 0xE8, 0x00]);

    assert_eq!(cpu.register_x, 0xc1)
}
