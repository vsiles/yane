# Implementation Roadmap

## Overview

This document provides a step-by-step roadmap for implementing your NES emulator. Follow these phases sequentially to build a solid foundation.

---

## Phase 0: Project Setup

### Rust Project Structure
```bash
cargo init yane
cd yane

# Add dependencies to Cargo.toml
```

**Cargo.toml**:
```toml
[package]
name = "yane"
version = "0.1.0"
edition = "2021"

[dependencies]
# For now, keep it minimal. Add as needed:
# - GUI/rendering library later (e.g., pixels, minifb, SDL2)
# - ROM parsing if needed

[dev-dependencies]
# For testing
```

### Initial Directory Structure
```
yane/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── cpu/
│   │   ├── mod.rs
│   │   ├── registers.rs
│   │   ├── opcodes.rs
│   │   └── addressing.rs
│   ├── bus.rs
│   └── cartridge.rs
├── tests/
│   ├── cpu_tests.rs
│   └── integration_tests.rs
├── nestest/
│   ├── nestest.nes
│   └── nestest.log
└── docs/
    └── (documentation files you already have)
```

---

## Phase 1: CPU Registers and Status Flags

**Goal**: Create the basic CPU structure with registers and status flag handling.

### Tasks

#### 1. Define CPU Registers
```rust
// src/cpu/registers.rs

pub struct Cpu {
    // 8-bit registers
    pub a: u8,      // Accumulator
    pub x: u8,      // X index register
    pub y: u8,      // Y index register
    pub sp: u8,     // Stack pointer

    // Status flags
    pub p: StatusFlags,

    // 16-bit register
    pub pc: u16,    // Program counter

    // Cycle counter
    pub cycles: u64,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD,
            p: StatusFlags::new(),
            pc: 0,
            cycles: 0,
        }
    }

    pub fn reset(&mut self, pc_start: u16) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.p = StatusFlags::from_byte(0x34); // I flag set
        self.pc = pc_start;
        self.cycles = 0;
    }
}
```

#### 2. Implement Status Flags
```rust
// src/cpu/registers.rs

#[derive(Debug, Clone, Copy)]
pub struct StatusFlags {
    pub carry: bool,        // C
    pub zero: bool,         // Z
    pub interrupt: bool,    // I
    pub decimal: bool,      // D (no effect on NES)
    pub overflow: bool,     // V
    pub negative: bool,     // N
}

impl StatusFlags {
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

    pub fn as_byte(&self) -> u8 {
        let mut byte = 0u8;
        if self.carry { byte |= 0x01; }
        if self.zero { byte |= 0x02; }
        if self.interrupt { byte |= 0x04; }
        if self.decimal { byte |= 0x08; }
        // Bit 5 always set when pushed (handle in push_status)
        if self.overflow { byte |= 0x40; }
        if self.negative { byte |= 0x80; }
        byte
    }

    pub fn from_byte(byte: u8) -> Self {
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

impl Cpu {
    pub fn update_zero_and_negative_flags(&mut self, value: u8) {
        self.p.zero = value == 0;
        self.p.negative = value & 0x80 != 0;
    }
}
```

#### 3. Write Tests
```rust
// tests/cpu_tests.rs

#[test]
fn test_cpu_power_on() {
    let cpu = Cpu::new();
    assert_eq!(cpu.sp, 0xFD);
    assert_eq!(cpu.p.interrupt, true);
}

#[test]
fn test_status_flags_conversion() {
    let mut flags = StatusFlags::new();
    flags.carry = true;
    flags.zero = true;
    assert_eq!(flags.as_byte() & 0x03, 0x03);

    let flags2 = StatusFlags::from_byte(0x03);
    assert_eq!(flags2.carry, true);
    assert_eq!(flags2.zero, true);
}
```

**Completion Criteria**:
- [x] CPU struct compiles
- [x] Status flags convert to/from bytes correctly
- [x] Tests pass
- [x] Zero warnings (clippy --all-targets)
- [x] Trait implementations (From<u8>, From<StatusFlags> for u8)
- [x] Both unit tests (10) and integration tests (7) passing

**✅ PHASE 1 COMPLETE** (17/17 tests passing, 0 warnings)

**Implemented**:
- `StatusFlags` struct with 6 flags (no B flag - correct per NES spec)
- `impl From<u8> for StatusFlags` and `impl From<StatusFlags> for u8`
- `Cpu` struct with all registers (a, x, y, sp, pc, p, cycles)
- Power-up state: SP=0xFD, P with I flag set
- `reset(pc_start)` method for nestest automation mode
- `update_zero_and_negative_flags()` helper
- Comprehensive tests covering all functionality

---

## Phase 2: Memory Bus

**Goal**: Create a memory interface for CPU to read/write.

### Tasks

#### 1. Create Bus Structure
```rust
// src/bus.rs

pub struct Bus {
    ram: [u8; 2048],  // 2KB internal RAM
    rom: Vec<u8>,     // Cartridge ROM (simple for now)
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: [0; 2048],
            rom: Vec::new(),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // Internal RAM (with mirroring)
            0x0000..=0x1FFF => self.ram[(addr & 0x07FF) as usize],

            // PPU Registers (stub for now)
            0x2000..=0x3FFF => 0,

            // APU and I/O (stub for now)
            0x4000..=0x4017 => 0,

            // Cartridge space
            0x8000..=0xFFFF => {
                let rom_addr = (addr - 0x8000) as usize;
                if rom_addr < self.rom.len() {
                    self.rom[rom_addr]
                } else {
                    0
                }
            }

            _ => 0, // Open bus
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // Internal RAM (with mirroring)
            0x0000..=0x1FFF => {
                self.ram[(addr & 0x07FF) as usize] = value;
            }

            // PPU Registers (stub for now)
            0x2000..=0x3FFF => {}

            // APU and I/O (stub for now)
            0x4000..=0x4017 => {}

            // ROM is read-only
            _ => {}
        }
    }

    pub fn load_rom(&mut self, rom_data: Vec<u8>) {
        self.rom = rom_data;
    }
}
```

#### 2. Add CPU-Bus Interface
```rust
// src/cpu/mod.rs

impl Cpu {
    pub fn read_byte(&self, bus: &Bus, addr: u16) -> u8 {
        bus.read(addr)
    }

    pub fn write_byte(&mut self, bus: &mut Bus, addr: u16, value: u8) {
        bus.write(addr, value);
    }

    pub fn fetch_byte(&mut self, bus: &Bus) -> u8 {
        let byte = self.read_byte(bus, self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }
}
```

**Completion Criteria**:
- [ ] Bus reads and writes work
- [ ] RAM mirroring implemented
- [ ] ROM space accessible
- [ ] Tests pass

---

## Phase 3: Simple Instructions (Load/Store/Transfer)

**Goal**: Implement enough instructions to run a basic program.

### Tasks

#### 1. Implement Immediate Addressing
```rust
// src/cpu/addressing.rs

impl Cpu {
    pub fn immediate(&mut self, bus: &Bus) -> u8 {
        self.fetch_byte(bus)
    }
}
```

#### 2. Implement LDA (Load Accumulator)
```rust
// src/cpu/opcodes.rs

impl Cpu {
    pub fn lda(&mut self, value: u8) {
        self.a = value;
        self.update_zero_and_negative_flags(value);
    }

    pub fn execute_lda_immediate(&mut self, bus: &Bus) -> u8 {
        let value = self.immediate(bus);
        self.lda(value);
        2 // cycles
    }
}
```

#### 3. Implement Other Load/Store Instructions
- LDA, LDX, LDY (immediate mode only for now)
- STA, STX, STY (zero page for testing)
- TAX, TXA, TAY, TYA

#### 4. Create Opcode Dispatch
```rust
// src/cpu/mod.rs

impl Cpu {
    pub fn step(&mut self, bus: &mut Bus) -> u8 {
        let opcode = self.fetch_byte(bus);
        let cycles = self.execute(opcode, bus);
        self.cycles += cycles as u64;
        cycles
    }

    fn execute(&mut self, opcode: u8, bus: &mut Bus) -> u8 {
        match opcode {
            0xA9 => self.execute_lda_immediate(bus),
            0xAA => self.execute_tax(),
            0xA2 => self.execute_ldx_immediate(bus),
            // ... add more opcodes
            _ => {
                panic!("Unimplemented opcode: {:02X} at PC: {:04X}",
                    opcode, self.pc.wrapping_sub(1));
            }
        }
    }
}
```

#### 5. Write Tests
```rust
#[test]
fn test_lda_immediate() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    // Write program: LDA #$42
    bus.write(0x8000, 0xA9);
    bus.write(0x8001, 0x42);
    cpu.pc = 0x8000;

    let cycles = cpu.step(&mut bus);

    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.p.zero, false);
    assert_eq!(cpu.p.negative, false);
    assert_eq!(cycles, 2);
}
```

**Completion Criteria**:
- [ ] LDA/LDX/LDY work (immediate mode)
- [ ] TAX/TXA/TAY/TYA work
- [ ] Flags update correctly
- [ ] Cycle counts correct
- [ ] Tests pass

---

## Phase 4: All Addressing Modes

**Goal**: Implement all addressing modes so instructions can access any memory.

### Tasks

#### 1. Implement Each Addressing Mode
```rust
// src/cpu/addressing.rs

impl Cpu {
    // Already done
    pub fn immediate(&mut self, bus: &Bus) -> u8 { /* ... */ }

    // Zero page: $nn
    pub fn zero_page(&mut self, bus: &Bus) -> u16 {
        self.fetch_byte(bus) as u16
    }

    // Zero page,X: $nn,X
    pub fn zero_page_x(&mut self, bus: &Bus) -> u16 {
        let addr = self.fetch_byte(bus);
        addr.wrapping_add(self.x) as u16 // Wraps in zero page
    }

    // Absolute: $nnnn
    pub fn absolute(&mut self, bus: &Bus) -> u16 {
        let low = self.fetch_byte(bus);
        let high = self.fetch_byte(bus);
        u16::from_le_bytes([low, high])
    }

    // Absolute,X: $nnnn,X (with page crossing detection)
    pub fn absolute_x(&mut self, bus: &Bus) -> (u16, bool) {
        let base = self.absolute(bus);
        let addr = base.wrapping_add(self.x as u16);
        let page_crossed = (base & 0xFF00) != (addr & 0xFF00);
        (addr, page_crossed)
    }

    // ... implement all other modes
}
```

#### 2. Expand Instructions to All Addressing Modes
For each instruction (LDA, STA, etc.), implement all its addressing mode variants.

**Example: LDA has 8 addressing modes**:
- 0xA9: Immediate
- 0xA5: Zero Page
- 0xB5: Zero Page,X
- 0xAD: Absolute
- 0xBD: Absolute,X
- 0xB9: Absolute,Y
- 0xA1: Indexed Indirect
- 0xB1: Indirect Indexed

**Completion Criteria**:
- [ ] All 13 addressing modes implemented
- [ ] Page crossing detection works
- [ ] Cycle penalties correct
- [ ] Tests for each mode pass

---

## Phase 5: Arithmetic, Logic, and Shifts

**Goal**: Implement ALU operations.

### Tasks

#### 1. Implement ADC (Add with Carry)
```rust
pub fn adc(&mut self, value: u8) {
    let a = self.a as u16;
    let v = value as u16;
    let c = if self.p.carry { 1 } else { 0 };

    let result = a + v + c;

    self.p.carry = result > 0xFF;

    let result = result as u8;

    // Overflow: (A^result) & (value^result) & 0x80
    self.p.overflow = ((self.a ^ result) & (value ^ result) & 0x80) != 0;

    self.a = result;
    self.update_zero_and_negative_flags(result);
}
```

#### 2. Implement SBC, AND, ORA, EOR
Similar structure, different operations.

#### 3. Implement Shifts and Rotates
- ASL, LSR, ROL, ROR
- Both accumulator and memory versions

#### 4. Implement INC, DEC, INX, DEX, INY, DEY

**Completion Criteria**:
- [ ] All arithmetic operations work
- [ ] Flag calculations correct (especially V and C)
- [ ] Shifts handle carry correctly
- [ ] Tests pass for edge cases (overflow, wraparound)

---

## Phase 6: Branches, Jumps, and Stack

**Goal**: Control flow and subroutine support.

### Tasks

#### 1. Implement Branch Instructions
```rust
pub fn branch(&mut self, condition: bool, bus: &Bus) -> u8 {
    let offset = self.fetch_byte(bus) as i8;

    if !condition {
        return 2; // Not taken
    }

    let old_pc = self.pc;
    self.pc = self.pc.wrapping_add(offset as u16);

    // Page crossing adds 1 cycle
    if (old_pc & 0xFF00) != (self.pc & 0xFF00) {
        4 // Taken, page cross
    } else {
        3 // Taken, no page cross
    }
}

pub fn execute_bne(&mut self, bus: &Bus) -> u8 {
    self.branch(!self.p.zero, bus)
}
```

#### 2. Implement Stack Operations
```rust
pub fn push_byte(&mut self, bus: &mut Bus, value: u8) {
    let addr = 0x0100 | (self.sp as u16);
    bus.write(addr, value);
    self.sp = self.sp.wrapping_sub(1);
}

pub fn pop_byte(&mut self, bus: &mut Bus) -> u8 {
    self.sp = self.sp.wrapping_add(1);
    let addr = 0x0100 | (self.sp as u16);
    bus.read(addr)
}

pub fn execute_pha(&mut self, bus: &mut Bus) -> u8 {
    self.push_byte(bus, self.a);
    3
}

pub fn execute_pla(&mut self, bus: &mut Bus) -> u8 {
    self.a = self.pop_byte(bus);
    self.update_zero_and_negative_flags(self.a);
    4
}
```

#### 3. Implement JMP, JSR, RTS
```rust
pub fn execute_jmp_absolute(&mut self, bus: &Bus) -> u8 {
    self.pc = self.absolute(bus);
    3
}

pub fn execute_jsr(&mut self, bus: &mut Bus) -> u8 {
    let target = self.absolute(bus);
    let return_addr = self.pc.wrapping_sub(1);

    self.push_byte(bus, (return_addr >> 8) as u8);
    self.push_byte(bus, (return_addr & 0xFF) as u8);

    self.pc = target;
    6
}

pub fn execute_rts(&mut self, bus: &mut Bus) -> u8 {
    let low = self.pop_byte(bus);
    let high = self.pop_byte(bus);
    self.pc = u16::from_le_bytes([low, high]).wrapping_add(1);
    6
}
```

#### 4. Implement JMP Indirect (with bug!)
```rust
pub fn execute_jmp_indirect(&mut self, bus: &Bus) -> u8 {
    let ptr = self.absolute(bus);

    let low = bus.read(ptr);

    // Hardware bug: wraps within page if low byte is $FF
    let high_addr = if ptr & 0xFF == 0xFF {
        ptr & 0xFF00
    } else {
        ptr + 1
    };

    let high = bus.read(high_addr);
    self.pc = u16::from_le_bytes([low, high]);
    5
}
```

**Completion Criteria**:
- [ ] All branch instructions work
- [ ] Branch timing correct (taken vs not taken, page cross)
- [ ] Stack operations work
- [ ] JSR/RTS work
- [ ] JMP indirect bug implemented
- [ ] Tests pass

---

## Phase 7: Interrupts

**Goal**: Handle NMI, IRQ, BRK, and RTI.

### Tasks

#### 1. Implement BRK
```rust
pub fn execute_brk(&mut self, bus: &mut Bus) -> u8 {
    self.pc = self.pc.wrapping_add(1); // Skip signature byte

    // Push PC
    self.push_byte(bus, (self.pc >> 8) as u8);
    self.push_byte(bus, (self.pc & 0xFF) as u8);

    // Push P with B=1, bit 5=1
    let mut status = self.p.as_byte();
    status |= 0x30; // Set B and bit 5
    self.push_byte(bus, status);

    // Set I flag
    self.p.interrupt = true;

    // Load IRQ vector
    let low = bus.read(0xFFFE);
    let high = bus.read(0xFFFF);
    self.pc = u16::from_le_bytes([low, high]);

    7
}
```

#### 2. Implement RTI
```rust
pub fn execute_rti(&mut self, bus: &mut Bus) -> u8 {
    let status = self.pop_byte(bus);
    self.p = StatusFlags::from_byte(status);

    let low = self.pop_byte(bus);
    let high = self.pop_byte(bus);
    self.pc = u16::from_le_bytes([low, high]);

    6
}
```

#### 3. Implement NMI and IRQ Handlers
```rust
pub fn trigger_nmi(&mut self, bus: &mut Bus) {
    // Push PC
    self.push_byte(bus, (self.pc >> 8) as u8);
    self.push_byte(bus, (self.pc & 0xFF) as u8);

    // Push P with B=0, bit 5=1
    let mut status = self.p.as_byte();
    status |= 0x20;  // Bit 5
    status &= !0x10; // B=0
    self.push_byte(bus, status);

    // Set I flag
    self.p.interrupt = true;

    // Load NMI vector
    let low = bus.read(0xFFFA);
    let high = bus.read(0xFFFB);
    self.pc = u16::from_le_bytes([low, high]);

    self.cycles += 7;
}

pub fn trigger_irq(&mut self, bus: &mut Bus) {
    if self.p.interrupt {
        return; // IRQ masked
    }

    // Same as NMI but use IRQ vector at $FFFE
    // ... similar implementation
}
```

**Completion Criteria**:
- [ ] BRK works (B flag set correctly)
- [ ] RTI restores state
- [ ] NMI and IRQ can be triggered
- [ ] IRQ respects I flag
- [ ] B flag distinguishes hardware vs software interrupts
- [ ] Tests pass

---

## Phase 8: Unofficial Opcodes

**Goal**: Implement stable unofficial opcodes for nestest.

### Tasks

#### 1. Implement Combined Operations
```rust
// LAX: Load A and X
pub fn lax(&mut self, value: u8) {
    self.a = value;
    self.x = value;
    self.update_zero_and_negative_flags(value);
}

// SAX: Store A AND X
pub fn sax(&mut self, bus: &mut Bus, addr: u16) {
    let value = self.a & self.x;
    bus.write(addr, value);
}

// DCP: Decrement and compare
pub fn dcp(&mut self, bus: &mut Bus, addr: u16) {
    let value = bus.read(addr);
    bus.write(addr, value); // Dummy write
    let result = value.wrapping_sub(1);
    bus.write(addr, result);

    // Compare with A
    self.compare(self.a, result);
}

// ... implement ISC, SLO, RLA, SRE, RRA
```

#### 2. Implement Unofficial NOPs
```rust
pub fn unofficial_nop(&mut self, bus: &Bus, bytes: u8, cycles: u8) -> u8 {
    // Read bytes (if any) to consume operands
    for _ in 0..bytes - 1 {
        self.fetch_byte(bus);
    }
    cycles
}
```

#### 3. Add to Opcode Dispatch
Map all unofficial opcode bytes to their implementations.

**Completion Criteria**:
- [ ] LAX, SAX, DCP, ISC, SLO, RLA, SRE, RRA implemented
- [ ] All unofficial NOP variants work
- [ ] Cycle counts correct
- [ ] nestest progresses further

---

## Phase 9: nestest Integration

**Goal**: Pass nestest automation mode.

### Tasks

#### 1. Implement Logging
```rust
pub fn log_state(&self, bus: &Bus) {
    let pc = self.pc;
    let opcode = bus.read(pc);
    let byte1 = bus.read(pc.wrapping_add(1));
    let byte2 = bus.read(pc.wrapping_add(2));

    let disasm = self.disassemble(pc, bus);

    println!(
        "{:04X}  {:02X} {:02X} {:02X}  {:30} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{:3}",
        pc, opcode, byte1, byte2, disasm,
        self.a, self.x, self.y, self.p.as_byte(), self.sp,
        self.cycles
    );
}
```

#### 2. Implement Disassembler
Create a simple disassembler for log output.

#### 3. Run nestest
```rust
// src/main.rs

fn main() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    // Load nestest ROM
    let rom_data = std::fs::read("nestest/nestest.nes").unwrap();
    // Skip iNES header (16 bytes), load PRG ROM
    bus.load_rom(rom_data[16..].to_vec());

    // Start at $C000 for automation mode
    cpu.pc = 0xC000;

    loop {
        cpu.log_state(&bus);
        cpu.step(&mut bus);

        // Check for completion
        if cpu.pc == 0xC66E {
            let result = bus.read(0x0002);
            if result == 0x00 {
                println!("\nnestest PASSED!");
            } else {
                println!("\nnestest FAILED with code: {:02X}", result);
            }
            break;
        }
    }
}
```

#### 4. Compare Logs
```bash
./yane > my_output.log 2>&1
diff <(head -n 5000 nestest.log) my_output.log
```

#### 5. Debug Failures
When logs diverge:
1. Find first difference
2. Check instruction implementation
3. Verify addressing mode
4. Check cycle count
5. Verify flag updates

**Completion Criteria**:
- [ ] nestest runs to completion
- [ ] Logs match reference (all 5000 lines)
- [ ] Result code at $0002 is $00
- [ ] All tests pass

---

## Phase 10: Cartridge Loading (iNES Format)

**Goal**: Properly parse NES ROM files.

### Tasks

#### 1. Implement iNES Header Parser
```rust
// src/cartridge.rs

pub struct Cartridge {
    prg_rom: Vec<u8>,  // Program ROM
    chr_rom: Vec<u8>,  // Character ROM (for PPU)
    mapper: u8,
    mirroring: Mirroring,
}

#[derive(Debug)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
}

impl Cartridge {
    pub fn load(path: &str) -> Result<Self, String> {
        let data = std::fs::read(path)
            .map_err(|e| format!("Failed to read ROM: {}", e))?;

        // Verify iNES header
        if &data[0..4] != b"NES\x1A" {
            return Err("Invalid iNES header".to_string());
        }

        let prg_rom_size = data[4] as usize * 16384; // 16KB units
        let chr_rom_size = data[5] as usize * 8192;  // 8KB units

        let flags6 = data[6];
        let flags7 = data[7];

        let mapper = (flags7 & 0xF0) | (flags6 >> 4);

        let mirroring = if flags6 & 0x08 != 0 {
            Mirroring::FourScreen
        } else if flags6 & 0x01 != 0 {
            Mirroring::Vertical
        } else {
            Mirroring::Horizontal
        };

        // Skip trainer if present
        let mut offset = 16;
        if flags6 & 0x04 != 0 {
            offset += 512; // Trainer
        }

        let prg_rom = data[offset..offset + prg_rom_size].to_vec();
        let chr_rom = data[offset + prg_rom_size..offset + prg_rom_size + chr_rom_size].to_vec();

        Ok(Cartridge {
            prg_rom,
            chr_rom,
            mapper,
            mirroring,
        })
    }
}
```

#### 2. Update Bus to Use Cartridge
```rust
// src/bus.rs

pub struct Bus {
    ram: [u8; 2048],
    cartridge: Option<Cartridge>,
}

impl Bus {
    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = Some(cartridge);
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.ram[(addr & 0x07FF) as usize],
            0x8000..=0xFFFF => {
                if let Some(cart) = &self.cartridge {
                    // Handle mapper 0 (NROM)
                    let rom_addr = (addr - 0x8000) as usize;
                    if cart.prg_rom.len() == 16384 {
                        // 16KB ROM, mirrored
                        cart.prg_rom[rom_addr & 0x3FFF]
                    } else {
                        // 32KB ROM
                        cart.prg_rom[rom_addr]
                    }
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}
```

**Completion Criteria**:
- [ ] iNES header parsed correctly
- [ ] PRG ROM loaded
- [ ] Mapper 0 (NROM) works
- [ ] nestest loads via cartridge loader

---

## Phase 11: Minimal PPU (Visual Output)

**Goal**: Get visual output to see nestest results on screen.

This is a large phase. For minimal support:

1. Implement basic PPU registers ($2000-$2007)
2. Implement frame timing (262 scanlines, 341 cycles each)
3. Trigger NMI on VBlank
4. Render background (minimal)
5. Use a graphics library (SDL2, minifb, etc.) to display

**Note**: This is beyond the initial CPU-focused scope but necessary to run most games.

---

## Summary: Development Order

1. ✅ **Phase 0**: Project setup
2. ✅ **Phase 1**: CPU registers and flags
3. ✅ **Phase 2**: Memory bus
4. ✅ **Phase 3**: Simple instructions (LDA, STA, transfers)
5. ✅ **Phase 4**: All addressing modes
6. ✅ **Phase 5**: Arithmetic, logic, shifts
7. ✅ **Phase 6**: Branches, jumps, stack
8. ✅ **Phase 7**: Interrupts (BRK, NMI, IRQ, RTI)
9. ✅ **Phase 8**: Unofficial opcodes
10. ✅ **Phase 9**: Pass nestest automation mode
11. **Phase 10**: Proper cartridge loading
12. **Phase 11**: Minimal PPU for visual output

---

## Daily Development Routine

### Morning: Implementation
- Choose next opcode/feature from roadmap
- Implement in small increments
- Write unit test immediately after

### Afternoon: Testing
- Run nestest
- Compare logs
- Debug any divergence
- Fix bugs found

### Evening: Documentation
- Update implementation notes
- Document any quirks discovered
- Commit working code

---

## Success Metrics

### Week 1
- [ ] Phases 1-3 complete
- [ ] Basic instructions work
- [ ] Simple programs run

### Week 2
- [ ] Phases 4-6 complete
- [ ] All addressing modes work
- [ ] Control flow works

### Week 3
- [ ] Phase 7-8 complete
- [ ] Interrupts work
- [ ] Unofficial opcodes implemented

### Week 4
- [ ] Phase 9 complete
- [ ] nestest passes in automation mode
- [ ] CPU implementation complete!

### Beyond
- Cartridge loading
- PPU implementation
- APU implementation
- Mapper support
- Full game compatibility

---

## Resources

- **nesdev Wiki**: https://www.nesdev.org/
- **6502 Reference**: http://www.6502.org/
- **Test ROMs**: https://github.com/christopherpow/nes-test-roms
- **Visual 6502**: http://www.visual6502.org/
- **NESDev Forums**: https://forums.nesdev.org/

---

## Next Steps

1. Set up Rust project structure
2. Start with Phase 1: CPU registers
3. Work through phases sequentially
4. Test continuously with nestest
5. Don't rush—accuracy over speed!

Good luck with your NES emulator! 🎮
