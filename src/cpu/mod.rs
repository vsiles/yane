//! NES CPU (Ricoh 2A03) implementation
//!
//! Based on the MOS 6502 processor, with no decimal mode support.

/// CPU Status Flags Register (P)
///
/// Layout: NV-B DIZC (bits 7-0)
/// - Bit 7: Negative (N)
/// - Bit 6: Overflow (V)
/// - Bit 5: Always 1 when pushed to stack (not stored in struct)
/// - Bit 4: B flag (only exists when pushed to stack, not stored in struct)
/// - Bit 3: Decimal (D) - flag exists but has NO effect on NES
/// - Bit 2: Interrupt Disable (I)
/// - Bit 1: Zero (Z)
/// - Bit 0: Carry (C)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusFlags {
    pub carry: bool,        // C - bit 0
    pub zero: bool,         // Z - bit 1
    pub interrupt: bool,    // I - bit 2
    pub decimal: bool,      // D - bit 3 (no effect on NES!)
    pub overflow: bool,     // V - bit 6
    pub negative: bool,     // N - bit 7
}

impl StatusFlags {
    /// Create new StatusFlags with power-up state
    ///
    /// Power-up state: I flag set, all others clear
    pub fn new() -> Self {
        StatusFlags {
            carry: false,
            zero: false,
            interrupt: true,  // Set on power-up
            decimal: false,
            overflow: false,
            negative: false,
        }
    }

    /// Convert status flags to a byte
    ///
    /// Note: Bits 4 and 5 are NOT set here. They only appear when P is
    /// pushed to the stack (handled in push_status operations).
    pub fn as_byte(&self) -> u8 {
        let mut byte = 0u8;
        if self.carry { byte |= 0x01; }
        if self.zero { byte |= 0x02; }
        if self.interrupt { byte |= 0x04; }
        if self.decimal { byte |= 0x08; }
        // Bits 4 and 5 NOT set here (handled in push_status later)
        if self.overflow { byte |= 0x40; }
        if self.negative { byte |= 0x80; }
        byte
    }

}

impl From<u8> for StatusFlags {
    /// Create StatusFlags from a byte
    ///
    /// Note: Bits 4 and 5 are ignored - they have no meaning when loading
    /// P from a byte (only meaningful during stack operations).
    fn from(byte: u8) -> Self {
        StatusFlags {
            carry: byte & 0x01 != 0,
            zero: byte & 0x02 != 0,
            interrupt: byte & 0x04 != 0,
            decimal: byte & 0x08 != 0,
            overflow: byte & 0x40 != 0,
            negative: byte & 0x80 != 0,
        }
    }
}

impl From<StatusFlags> for u8 {
    /// Convert StatusFlags to a byte
    ///
    /// Note: Bits 4 and 5 are NOT set here. They only appear when P is
    /// pushed to the stack (handled in push_status operations).
    fn from(flags: StatusFlags) -> Self {
        let mut byte = 0u8;
        if flags.carry { byte |= 0x01; }
        if flags.zero { byte |= 0x02; }
        if flags.interrupt { byte |= 0x04; }
        if flags.decimal { byte |= 0x08; }
        // Bits 4 and 5 NOT set here (handled in push_status later)
        if flags.overflow { byte |= 0x40; }
        if flags.negative { byte |= 0x80; }
        byte
    }
}

impl Default for StatusFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// NES CPU (Ricoh 2A03)
///
/// 6502-based CPU with no decimal mode support
pub struct Cpu {
    // 8-bit registers (public for easy access during testing/debugging)
    pub a: u8,           // Accumulator
    pub x: u8,           // X index register
    pub y: u8,           // Y index register
    pub sp: u8,          // Stack pointer (points to $0100-$01FF)

    // Status flags
    pub p: StatusFlags,  // Processor status flags

    // 16-bit register
    pub pc: u16,         // Program counter

    // Cycle counter
    pub cycles: u64,     // Total cycles executed
}

impl Cpu {
    /// Create new CPU with power-up state
    ///
    /// Power-up state:
    /// - A, X, Y: 0x00
    /// - SP: 0xFD (which is 0x100 - 3)
    /// - P: I flag set (0x04 internally, displays as 0x34 with phantom bits)
    /// - PC: 0x0000 (will be loaded from reset vector)
    /// - Cycles: 0
    pub fn new() -> Self {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD,
            p: StatusFlags::new(),  // I flag set
            pc: 0,
            cycles: 0,
        }
    }

    /// Reset CPU to known state
    ///
    /// In real hardware, only SP, I flag, and PC are affected by RESET.
    /// For simplicity, we initialize everything like power-up.
    pub fn reset(&mut self, pc_start: u16) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.p = 0x34.into();  // I flag set
        self.pc = pc_start;
        self.cycles = 0;
    }

    /// Update Zero and Negative flags based on a value
    ///
    /// This is a common operation after most instructions that modify
    /// registers or memory.
    pub fn update_zero_and_negative_flags(&mut self, value: u8) {
        self.p.zero = value == 0;
        self.p.negative = value & 0x80 != 0;
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // StatusFlags tests
    #[test]
    fn test_status_flags_new() {
        let flags = StatusFlags::new();
        assert!(!flags.carry);
        assert!(!flags.zero);
        assert!(flags.interrupt);  // Set on power-up
        assert!(!flags.decimal);
        assert!(!flags.overflow);
        assert!(!flags.negative);
    }

    #[test]
    fn test_status_flags_as_byte() {
        let mut flags = StatusFlags::new();
        flags.interrupt = false;  // Clear I for testing

        // Test each flag individually
        flags.carry = true;
        assert_eq!(flags.as_byte() & 0x01, 0x01);
        flags.carry = false;

        flags.zero = true;
        assert_eq!(flags.as_byte() & 0x02, 0x02);
        flags.zero = false;

        flags.interrupt = true;
        assert_eq!(flags.as_byte() & 0x04, 0x04);
        flags.interrupt = false;

        flags.decimal = true;
        assert_eq!(flags.as_byte() & 0x08, 0x08);
        flags.decimal = false;

        flags.overflow = true;
        assert_eq!(flags.as_byte() & 0x40, 0x40);
        flags.overflow = false;

        flags.negative = true;
        assert_eq!(flags.as_byte() & 0x80, 0x80);
    }

    #[test]
    fn test_status_flags_from_u8() {
        // Test with all flags set
        let flags: StatusFlags = 0xFF.into();
        assert!(flags.carry);
        assert!(flags.zero);
        assert!(flags.interrupt);
        assert!(flags.decimal);
        assert!(flags.overflow);
        assert!(flags.negative);

        // Test with no flags set
        let flags = StatusFlags::from(0x00);
        assert!(!flags.carry);
        assert!(!flags.zero);
        assert!(!flags.interrupt);
        assert!(!flags.decimal);
        assert!(!flags.overflow);
        assert!(!flags.negative);

        // Test specific pattern (I flag only, like power-up)
        let flags: StatusFlags = 0x04.into();
        assert!(flags.interrupt);
        assert!(!flags.carry);
        assert!(!flags.zero);
    }

    #[test]
    fn test_status_flags_round_trip() {
        let mut flags = StatusFlags::new();
        flags.carry = true;
        flags.zero = true;
        flags.overflow = true;

        let byte: u8 = flags.into();
        let flags2: StatusFlags = byte.into();

        assert_eq!(flags, flags2);
    }

    #[test]
    fn test_status_flags_ignores_bits_4_and_5() {
        // Bits 4 and 5 should be ignored when loading from byte
        let flags1: StatusFlags = 0x34.into();  // Bits 5, 4, and 2 set
        let flags2: StatusFlags = 0x04.into();  // Only bit 2 set

        // Both should have only I flag set (bit 2)
        assert!(flags1.interrupt);
        assert!(flags2.interrupt);

        // Converting back to u8 should not include bits 4 and 5
        let byte1: u8 = flags1.into();
        let byte2: u8 = flags2.into();
        assert_eq!(byte1, 0x04);
        assert_eq!(byte2, 0x04);
    }

    // Cpu tests
    #[test]
    fn test_cpu_new() {
        let cpu = Cpu::new();
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.sp, 0xFD);
        assert!(cpu.p.interrupt);
        assert_eq!(cpu.pc, 0);
        assert_eq!(cpu.cycles, 0);
    }

    #[test]
    fn test_cpu_reset() {
        let mut cpu = Cpu::new();

        // Modify some state
        cpu.a = 0x42;
        cpu.pc = 0x1234;
        cpu.cycles = 100;

        // Reset to specific PC
        cpu.reset(0xC000);

        // Check reset state
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.sp, 0xFD);
        assert!(cpu.p.interrupt);
        assert_eq!(cpu.pc, 0xC000);
        assert_eq!(cpu.cycles, 0);
    }

    #[test]
    fn test_update_zero_flag() {
        let mut cpu = Cpu::new();

        // Zero value should set Z flag
        cpu.update_zero_and_negative_flags(0x00);
        assert!(cpu.p.zero);
        assert!(!cpu.p.negative);

        // Non-zero value should clear Z flag
        cpu.update_zero_and_negative_flags(0x42);
        assert!(!cpu.p.zero);
    }

    #[test]
    fn test_update_negative_flag() {
        let mut cpu = Cpu::new();

        // Value with bit 7 set should set N flag
        cpu.update_zero_and_negative_flags(0x80);
        assert!(cpu.p.negative);
        assert!(!cpu.p.zero);

        cpu.update_zero_and_negative_flags(0xFF);
        assert!(cpu.p.negative);

        // Value with bit 7 clear should clear N flag
        cpu.update_zero_and_negative_flags(0x7F);
        assert!(!cpu.p.negative);
    }

    #[test]
    fn test_zero_and_negative_flags_together() {
        let mut cpu = Cpu::new();

        // Test 0x00: Z=1, N=0
        cpu.update_zero_and_negative_flags(0x00);
        assert!(cpu.p.zero);
        assert!(!cpu.p.negative);

        // Test 0x80: Z=0, N=1
        cpu.update_zero_and_negative_flags(0x80);
        assert!(!cpu.p.zero);
        assert!(cpu.p.negative);

        // Test 0x01: Z=0, N=0
        cpu.update_zero_and_negative_flags(0x01);
        assert!(!cpu.p.zero);
        assert!(!cpu.p.negative);
    }
}
