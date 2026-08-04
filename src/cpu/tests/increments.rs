use crate::cpu::CPU;

#[test]
fn test_inx_overflow() {
    let mut cpu = CPU::new();

    cpu.load(&[0xE8, 0xE8, 0x00]);
    cpu.reset();
    cpu.register_x = 0xFF;
    cpu.run();

    assert_eq!(cpu.register_x, 1)
}
