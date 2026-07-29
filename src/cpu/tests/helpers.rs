use crate::cpu::CPU;
use crate::cpu::status::*;

#[test]
fn test_fetch_byte() {
    let mut cpu = CPU::new();
    let program = &[0xA9, 0x05, 0x00];

    cpu.load(program);
    cpu.reset();

    assert_eq!(cpu.fetch8(), 0xa9);
    assert_eq!(cpu.fetch8(), 0x05);
    assert_eq!(cpu.fetch8(), 0x00);
    assert_eq!(cpu.program_counter, 0x8003); // ROM starts at 0x8000
}

#[test]
fn set_update_flags_zero_value() {
    let mut cpu = CPU::new();

    cpu.update_zero_and_negative_flags(0x00);

    assert_eq!(cpu.status & ZERO, ZERO);
    assert_eq!(cpu.status & NEGATIVE, 0);
}

#[test]
fn set_update_flags_negative_value() {
    let mut cpu = CPU::new();

    cpu.update_zero_and_negative_flags(0x80);

    assert_eq!(cpu.status & ZERO, 0);
    assert_eq!(cpu.status & NEGATIVE, NEGATIVE);
}

#[test]
fn set_update_flags_clears_previous_flags() {
    let mut cpu = CPU::new();

    // Clears zero
    cpu.update_zero_and_negative_flags(0x00);
    cpu.update_zero_and_negative_flags(0x05);

    assert_eq!(cpu.status & ZERO, 0);

    // Clears negative
    cpu.update_zero_and_negative_flags(0x80);
    cpu.update_zero_and_negative_flags(0x7F);

    assert_eq!(cpu.status & NEGATIVE, 0);
}
