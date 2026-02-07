//! Integration tests for CPU
//!
//! These tests exercise the CPU as an external user would (via the public API)

use yane::cpu::{Cpu, StatusFlags};

#[test]
fn test_cpu_power_on_state() {
    let cpu = Cpu::new();

    // Verify all power-up state
    assert_eq!(cpu.a, 0x00, "Accumulator should be 0x00 on power-up");
    assert_eq!(cpu.x, 0x00, "X register should be 0x00 on power-up");
    assert_eq!(cpu.y, 0x00, "Y register should be 0x00 on power-up");
    assert_eq!(cpu.sp, 0xFD, "Stack pointer should be 0xFD on power-up");
    assert_eq!(cpu.pc, 0x0000, "Program counter should be 0x0000 on power-up");
    assert_eq!(cpu.cycles, 0, "Cycles should be 0 on power-up");

    // Verify status flags
    assert!(cpu.p.interrupt, "I flag should be set on power-up");
    assert!(!cpu.p.carry, "C flag should be clear on power-up");
    assert!(!cpu.p.zero, "Z flag should be clear on power-up");
    assert!(!cpu.p.decimal, "D flag should be clear on power-up");
    assert!(!cpu.p.overflow, "V flag should be clear on power-up");
    assert!(!cpu.p.negative, "N flag should be clear on power-up");
}

#[test]
fn test_status_flags_bit_positions() {
    // Verify each flag maps to correct bit position
    let mut flags = StatusFlags::new();

    // Clear I flag for testing
    flags.interrupt = false;

    // Test Carry (bit 0, 0x01)
    flags.carry = true;
    assert_eq!(flags.as_byte(), 0x01, "Carry flag should be bit 0");
    flags.carry = false;

    // Test Zero (bit 1, 0x02)
    flags.zero = true;
    assert_eq!(flags.as_byte(), 0x02, "Zero flag should be bit 1");
    flags.zero = false;

    // Test Interrupt Disable (bit 2, 0x04)
    flags.interrupt = true;
    assert_eq!(flags.as_byte(), 0x04, "Interrupt flag should be bit 2");
    flags.interrupt = false;

    // Test Decimal (bit 3, 0x08)
    flags.decimal = true;
    assert_eq!(flags.as_byte(), 0x08, "Decimal flag should be bit 3");
    flags.decimal = false;

    // Test Overflow (bit 6, 0x40)
    flags.overflow = true;
    assert_eq!(flags.as_byte(), 0x40, "Overflow flag should be bit 6");
    flags.overflow = false;

    // Test Negative (bit 7, 0x80)
    flags.negative = true;
    assert_eq!(flags.as_byte(), 0x80, "Negative flag should be bit 7");
}

#[test]
fn test_status_flags_ignore_bit_4_and_5() {
    // Bits 4 and 5 should be ignored when creating from byte
    // This tests the NES hardware quirk where B flag (bit 4) and bit 5
    // only exist when P is pushed to the stack

    // Test with bits 4 and 5 set (0x30 = 00110000)
    let flags_with_phantom_bits: StatusFlags = 0x34.into();  // 00110100 (bits 5,4,2)
    let flags_without_phantom_bits: StatusFlags = 0x04.into();  // 00000100 (bit 2 only)

    // Both should result in the same internal state
    assert_eq!(flags_with_phantom_bits, flags_without_phantom_bits,
        "StatusFlags should ignore bits 4 and 5 when loading from byte");

    // Both should have only I flag set
    assert!(flags_with_phantom_bits.interrupt);
    assert!(!flags_with_phantom_bits.carry);
    assert!(!flags_with_phantom_bits.zero);
    assert!(!flags_with_phantom_bits.decimal);
    assert!(!flags_with_phantom_bits.overflow);
    assert!(!flags_with_phantom_bits.negative);

    // When converted back to byte, bits 4 and 5 should NOT be set
    let byte: u8 = flags_with_phantom_bits.into();
    assert_eq!(byte, 0x04,
        "Converting to u8 should not include phantom bits 4 and 5");
}

#[test]
fn test_cpu_reset_with_pc() {
    let mut cpu = Cpu::new();

    // Modify CPU state
    cpu.a = 0xFF;
    cpu.x = 0xAA;
    cpu.y = 0x55;
    cpu.cycles = 12345;

    // Reset to specific PC (like nestest automation mode)
    cpu.reset(0xC000);

    // Verify reset clears registers
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.x, 0x00);
    assert_eq!(cpu.y, 0x00);
    assert_eq!(cpu.sp, 0xFD);
    assert_eq!(cpu.cycles, 0);

    // Verify PC set to specified value
    assert_eq!(cpu.pc, 0xC000, "Reset should set PC to specified value");

    // Verify I flag set
    assert!(cpu.p.interrupt, "Reset should set I flag");
}

#[test]
fn test_update_flags_helper() {
    let mut cpu = Cpu::new();

    // Test zero flag with 0x00
    cpu.update_zero_and_negative_flags(0x00);
    assert!(cpu.p.zero, "Z should be set for value 0x00");
    assert!(!cpu.p.negative, "N should be clear for value 0x00");

    // Test negative flag with 0x80
    cpu.update_zero_and_negative_flags(0x80);
    assert!(!cpu.p.zero, "Z should be clear for value 0x80");
    assert!(cpu.p.negative, "N should be set for value 0x80");

    // Test both clear with 0x42
    cpu.update_zero_and_negative_flags(0x42);
    assert!(!cpu.p.zero, "Z should be clear for value 0x42");
    assert!(!cpu.p.negative, "N should be clear for value 0x42");

    // Test negative with 0xFF
    cpu.update_zero_and_negative_flags(0xFF);
    assert!(!cpu.p.zero, "Z should be clear for value 0xFF");
    assert!(cpu.p.negative, "N should be set for value 0xFF");
}

#[test]
fn test_status_flags_default() {
    let flags = StatusFlags::default();
    let flags2 = StatusFlags::new();

    assert_eq!(flags, flags2, "default() should match new()");
}

#[test]
fn test_cpu_default() {
    let cpu = Cpu::default();
    let cpu2 = Cpu::new();

    assert_eq!(cpu.a, cpu2.a);
    assert_eq!(cpu.x, cpu2.x);
    assert_eq!(cpu.y, cpu2.y);
    assert_eq!(cpu.sp, cpu2.sp);
    assert_eq!(cpu.p, cpu2.p);
    assert_eq!(cpu.pc, cpu2.pc);
    assert_eq!(cpu.cycles, cpu2.cycles);
}
