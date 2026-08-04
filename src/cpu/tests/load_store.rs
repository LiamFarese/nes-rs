use crate::cpu::Mem;
use crate::cpu::{CPU, status::*};

mod lda {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();

        cpu.load_and_run(&[0xA9, 0x05, 0x00]);

        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.status & ZERO, 0);
        assert_eq!(cpu.status & NEGATIVE, 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();

        cpu.load_and_run(&[0xA9, 0x00, 0x00]);

        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.status & ZERO, ZERO);
        assert_eq!(cpu.status & NEGATIVE, 0);
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        // First negative value in signed 8-bit range (-128)
        let min_signed_byte = 0x80;

        cpu.load_and_run(&[0xA9, min_signed_byte, 0x00]);

        assert_eq!(cpu.register_a, min_signed_byte);
        assert_eq!(cpu.status & ZERO, 0);
        assert_eq!(cpu.status & NEGATIVE, NEGATIVE);
    }
    #[test]
    fn test_0xa5_lda_zeropage_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(&[0xa5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_0xb5_lda_zero_page_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x11, 0x55);

        cpu.load(&[0xB5, 0x10, 0x00]);
        cpu.reset();
        cpu.register_x = 0x01;
        cpu.run();

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_0xad_lda_absolute() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x8010, 0x55);

        cpu.load_and_run(&[0xAD, 0x10, 0x80, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_0xbd_lda_absolute_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x8011, 0x55);

        cpu.load(&[0xBD, 0x10, 0x80, 0x00]);
        cpu.reset();
        cpu.register_x = 0x01;
        cpu.run();

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_0xb9_lda_absolute_y() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x8011, 0x55);

        cpu.load(&[0xB9, 0x10, 0x80, 0x00]);
        cpu.reset();
        cpu.register_y = 0x01;
        cpu.run();

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_0xa1_lda_indirect_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x02, 0x01);
        cpu.mem_write(0x03, 0x0F);
        cpu.mem_write(0x0F01, 0x55);

        cpu.load(&[0xA1, 0x01, 0x00]);
        cpu.reset();
        cpu.register_x = 0x01;
        cpu.run();

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_0xb1_lda_indirect_y() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x01, 0x00);
        cpu.mem_write(0x02, 0xFF);
        cpu.mem_write(0xFFFF, 0x55);

        cpu.load(&[0xB1, 0x01, 0x00]);
        cpu.reset();
        cpu.register_y = 0xFF;
        cpu.run();

        assert_eq!(cpu.register_a, 0x55);
    }
}
