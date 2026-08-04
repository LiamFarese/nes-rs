use crate::cpu::tests::with_cpu;

mod and {
    use crate::cpu::Mem;

    use super::*;

    #[test]
    fn test_0x29_and_immediate() {
        let cpu = with_cpu(&[0xA9, 0x10, 0x29, 0x01, 0x00], |_| {});
        assert_eq!(cpu.register_a, 0x00);
    }

    #[test]
    fn test_0x25_and_zeropage() {
        let cpu = with_cpu(&[0xA9, 0x10, 0x25, 0x01, 0x00], |cpu| {
            cpu.mem_write(0x01, 0x10);
        });
        assert_eq!(cpu.register_a, 0x10);
    }
}
