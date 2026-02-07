# Testing Guide for NES Emulator

## Overview

Testing is crucial for NES emulator development. The nestest ROM is the gold standard for CPU validation and should pass 100% before moving to PPU implementation.

## nestest - The Primary Test Suite

### What is nestest?

nestest is a comprehensive CPU test ROM that:
- Tests all official and many unofficial opcodes
- Validates cycle-accurate timing
- Checks flag behavior
- Tests addressing modes
- Verifies interrupt handling

### Getting nestest

The nestest ROM and reference log are available from:
- https://github.com/christopherpow/nes-test-roms

**In this project**: The nestest files are located in `nestest/`:
- **nestest/nestest.nes**: The test ROM (16KB PRG, 8KB CHR, Mapper 0)
- **nestest/nestest.log**: Reference execution log from Nintendulator (8991 lines)

### Running nestest

nestest has two modes:

#### Mode 1: Automation Mode (Recommended for Development)
Start execution at **$C000** (not the normal RESET vector):
```rust
fn run_nestest_automation() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    // Load nestest.nes ROM
    bus.load_rom("nestest/nestest.nes");

    // Start at $C000 for automation mode
    cpu.pc = 0xC000;

    // Run and log each instruction
    loop {
        log_state(&cpu);
        cpu.step(&mut bus);

        // Check result at $0002-$0003
        if cpu.pc == 0xC66E {
            let result = bus.read(0x0002);
            if result == 0x00 {
                println!("nestest PASSED!");
            } else {
                println!("nestest FAILED with code: {:02X}", result);
            }
            break;
        }
    }
}
```

#### Mode 2: Full Mode (With PPU)
Start at RESET vector. Requires minimal PPU implementation to pass.

**For initial development, use Mode 1 (automation mode).**

### nestest Log Format

The reference log format is:
```
C000  4C F5 C5  JMP $C5F5                       A:00 X:00 Y:00 P:24 SP:FD PPU:  0, 21 CYC:7
C5F5  A2 00     LDX #$00                        A:00 X:00 Y:00 P:24 SP:FD PPU:  0, 30 CYC:10
C5F7  86 00     STX $00 = 00                    A:00 X:00 Y:00 P:26 SP:FD PPU:  0, 36 CYC:12
C6BD  04 A9    *NOP $A9 = 00                    A:AA X:97 Y:4E P:EF SP:F9 PPU:128, 98 CYC:14582
...
```

Format breakdown:
- **C000**: Current PC
- **4C F5 C5**: Instruction bytes (opcode + operands, space-padded to 3 bytes)
- **JMP $C5F5**: Disassembled instruction (unofficial opcodes prefixed with `*`)
- **A:00 X:00 Y:00 P:24 SP:FD**: Register state BEFORE execution
- **PPU:  0, 21**: PPU state (scanline, pixel) - can be ignored for CPU-only testing
- **CYC:7**: Total CPU cycles executed

**Note**: The log has 8991 lines total. Automation mode (starting at $C000) covers the main CPU tests.

### Implementing Log Output

```rust
impl Cpu {
    fn log_state(&self, bus: &Bus) {
        let pc = self.pc;
        let opcode = bus.read(pc);

        // Fetch instruction bytes (1-3 bytes)
        let byte1 = opcode;
        let byte2 = bus.read(pc.wrapping_add(1));
        let byte3 = bus.read(pc.wrapping_add(2));

        // Disassemble instruction (implement disassembler)
        let instruction = self.disassemble(pc, bus);

        // Print in nestest format
        // Note: PPU state omitted for CPU-only implementation
        println!(
            "{:04X}  {:02X} {:02X} {:02X}  {:30} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PPU:XXX,XXX CYC:{}",
            pc,
            byte1, byte2, byte3,
            instruction,
            self.a, self.x, self.y,
            self.p.as_byte(),
            self.sp,
            self.cycles
        );
        // For initial CPU-only development, you can omit PPU state
        // or replace with dummy values as shown above
    }
}
```

**Important**: Log state BEFORE executing the instruction.

### Comparing Logs

```bash
# Run your emulator and save output
./yane > my_output.log

# Compare with reference log
diff nestest/nestest.log my_output.log

# Or use a side-by-side comparison
diff -y nestest/nestest.log my_output.log | less

# If you're only checking automation mode (recommended initially),
# you can compare until the test ends at PC $C66E
```

When logs diverge:
1. Find the first differing line
2. That instruction is where your bug manifests
3. The actual bug is likely in a previous instruction
4. Check register values carefully

### nestest Result Codes

nestest writes a result code to address $0002:
- **$00**: All tests passed
- **$01+**: Test failed (code indicates which test)

It also writes a status code to $0003.

### Common nestest Failures

#### Cycle Count Mismatch
**Symptom**: Everything else matches, but CYC: is off

**Causes**:
- Missing page boundary crossing penalty
- Wrong cycle count for instruction
- Branch timing wrong (taken vs not taken)
- Starting cycle count wrong (should start at 7 for automation mode)

**Fix**: Review cycle counting in addressing modes and branches.

#### PPU Mismatch (Can Be Ignored Initially)
**Symptom**: PPU scanline/pixel values differ

**Note**: For CPU-only development, PPU state can be ignored or stubbed with dummy values. The CPU tests don't depend on accurate PPU emulation in automation mode.

#### Flag Mismatch
**Symptom**: P: register differs

**Causes**:
- Incorrect overflow flag calculation (V)
- Wrong carry flag behavior (C)
- Missing flag updates
- B flag not handled correctly in push/pull

**Fix**: Review flag setting logic for each instruction.

#### Register Mismatch
**Symptom**: A, X, Y, or SP differs

**Causes**:
- Incorrect instruction implementation
- Wrong addressing mode
- Missing side effects (e.g., TSX sets N and Z flags)

**Fix**: Review instruction implementation against reference.

#### PC Divergence
**Symptom**: PC is completely wrong, cascade of errors

**Causes**:
- Wrong instruction length (fetched wrong number of bytes)
- Branch calculation error
- JMP/JSR/RTS bug

**Fix**: The error likely occurred several instructions before divergence.

---

## Other CPU Test ROMs

### instr_test_v5
Tests individual instruction behavior.

**Tests**:
1. Basics
2. Implied
3. Immediate
4. Zero page
5. Zero page,xy
6. Absolute
7. Absolute,xy
8. Indirect,x
9. Indirect,y
10. Branches
11. Stack
12. JMP/JSR/RTI/RTS
13. Special

**Usage**:
```bash
# Each ROM is standalone
./yane instr_test-v5/01-basics.nes
# Check result at $6000 (0 = pass, non-zero = fail)
# Error message at $6004+ (null-terminated string)
```

### cpu_timing_test6
Tests instruction timing for all official and unofficial opcodes (except branches and halts).

**Pass condition**: Displays "Passed" on screen

### branch_timing_tests
Tests branch instruction timing including page crossing.

### cpu_interrupts_v2
Tests NMI and IRQ behavior and timing.

**Critical for**:
- Interrupt polling timing
- Interrupt hijacking
- CLI/SEI/PLP delay

### cpu_dummy_reads
Validates that CPU performs correct dummy reads during:
- Page crossing
- Indexed addressing
- RMW instructions

### cpu_reset
Verifies register state after power-up and RESET.

**Tests**:
- Power-up register values
- RESET register changes
- RAM behavior (unpredictable)

---

## Test-Driven Development Approach

### Phase 1: Basic Infrastructure
```rust
#[test]
fn test_cpu_power_on() {
    let cpu = Cpu::new();
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.x, 0x00);
    assert_eq!(cpu.y, 0x00);
    assert_eq!(cpu.sp, 0xFD);
    assert_eq!(cpu.p.interrupt, true);
}

#[test]
fn test_status_flags() {
    let mut flags = StatusFlags::new();
    flags.carry = true;
    flags.zero = true;
    assert_eq!(flags.as_byte(), 0x03);
}
```

### Phase 2: Individual Opcodes
```rust
#[test]
fn test_lda_immediate() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    // LDA #$42
    bus.write(0x0000, 0xA9);
    bus.write(0x0001, 0x42);
    cpu.pc = 0x0000;

    let cycles = cpu.step(&mut bus);

    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.p.zero, false);
    assert_eq!(cpu.p.negative, false);
    assert_eq!(cpu.pc, 0x0002);
    assert_eq!(cycles, 2);
}

#[test]
fn test_lda_sets_zero_flag() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    // LDA #$00
    bus.write(0x0000, 0xA9);
    bus.write(0x0001, 0x00);
    cpu.pc = 0x0000;

    cpu.step(&mut bus);

    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.p.zero, true);
    assert_eq!(cpu.p.negative, false);
}

#[test]
fn test_lda_sets_negative_flag() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    // LDA #$FF
    bus.write(0x0000, 0xA9);
    bus.write(0x0001, 0xFF);
    cpu.pc = 0x0000;

    cpu.step(&mut bus);

    assert_eq!(cpu.a, 0xFF);
    assert_eq!(cpu.p.zero, false);
    assert_eq!(cpu.p.negative, true);
}
```

### Phase 3: Addressing Modes
```rust
#[test]
fn test_absolute_indexed_x_no_page_cross() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    // LDA $0200,X where X=0x10
    bus.write(0x0000, 0xBD);
    bus.write(0x0001, 0x00);
    bus.write(0x0002, 0x02);
    bus.write(0x0210, 0x42);

    cpu.pc = 0x0000;
    cpu.x = 0x10;

    let cycles = cpu.step(&mut bus);

    assert_eq!(cpu.a, 0x42);
    assert_eq!(cycles, 4); // No page cross
}

#[test]
fn test_absolute_indexed_x_page_cross() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    // LDA $01FF,X where X=0x02
    bus.write(0x0000, 0xBD);
    bus.write(0x0001, 0xFF);
    bus.write(0x0002, 0x01);
    bus.write(0x0201, 0x42);

    cpu.pc = 0x0000;
    cpu.x = 0x02;

    let cycles = cpu.step(&mut bus);

    assert_eq!(cpu.a, 0x42);
    assert_eq!(cycles, 5); // Page cross penalty
}
```

### Phase 4: Complex Operations
```rust
#[test]
fn test_adc_overflow() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    // Test: 0x50 + 0x50 = 0xA0 (overflow in signed arithmetic)
    cpu.a = 0x50;
    bus.write(0x0000, 0x69); // ADC #$50
    bus.write(0x0001, 0x50);
    cpu.pc = 0x0000;

    cpu.step(&mut bus);

    assert_eq!(cpu.a, 0xA0);
    assert_eq!(cpu.p.overflow, true);  // +80 + +80 = -96 (overflow)
    assert_eq!(cpu.p.carry, false);
    assert_eq!(cpu.p.negative, true);
}

#[test]
fn test_branch_taken_same_page() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    // BNE $10 (offset +16)
    bus.write(0x0000, 0xD0);
    bus.write(0x0001, 0x10);

    cpu.pc = 0x0000;
    cpu.p.zero = false; // Branch will be taken

    let cycles = cpu.step(&mut bus);

    assert_eq!(cpu.pc, 0x0012); // 0x0002 + 0x10
    assert_eq!(cycles, 3); // Branch taken, no page cross
}

#[test]
fn test_branch_not_taken() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    // BNE $10
    bus.write(0x0000, 0xD0);
    bus.write(0x0001, 0x10);

    cpu.pc = 0x0000;
    cpu.p.zero = true; // Branch will NOT be taken

    let cycles = cpu.step(&mut bus);

    assert_eq!(cpu.pc, 0x0002);
    assert_eq!(cycles, 2); // Branch not taken
}
```

---

## Debugging Strategies

### 1. Binary Search
If nestest fails at line 1000:
- Comment out instructions 500-1000
- Does it pass? Bug is in second half.
- Otherwise, bug is in first half.
- Repeat until you find the problematic instruction.

### 2. Minimal Reproduction
Create a minimal test case:
```rust
#[test]
fn minimal_repro() {
    // Exact state from nestest log where failure occurs
    let mut cpu = Cpu::new();
    cpu.a = 0x42;
    cpu.x = 0x10;
    // ... set exact state

    // Execute single failing instruction
    // Compare result with expected
}
```

### 3. Trace Comparison
```rust
impl Cpu {
    fn trace_execute(&mut self, bus: &mut Bus) {
        println!("Before: PC={:04X} A={:02X} X={:02X} Y={:02X} P={:02X} SP={:02X}",
            self.pc, self.a, self.x, self.y, self.p.as_byte(), self.sp);

        let opcode = bus.read(self.pc);
        println!("Opcode: {:02X}", opcode);

        self.execute(opcode, bus);

        println!("After:  PC={:04X} A={:02X} X={:02X} Y={:02X} P={:02X} SP={:02X}",
            self.pc, self.a, self.x, self.y, self.p.as_byte(), self.sp);
    }
}
```

### 4. Flag-by-Flag Analysis
For flag errors, test each flag independently:
```rust
#[test]
fn test_adc_carry_flag() {
    // Test only carry flag behavior
    let mut cpu = Cpu::new();
    cpu.a = 0xFF;
    cpu.p.carry = true;

    // ADC #$01 should set carry
    // ... test
}
```

---

## Continuous Integration

### Automated Testing
```bash
#!/bin/bash
# test_nestest.sh

./yane > output.log 2>&1

# Check if passed
if tail -1 output.log | grep -q "PASSED"; then
    echo "✓ nestest passed"
    exit 0
else
    echo "✗ nestest failed"
    # Show first difference
    diff nestest/nestest.log output.log | head -20
    exit 1
fi
```

### Rust Integration Tests
```rust
// tests/integration_test.rs

#[test]
fn nestest_automation() {
    let mut emu = Emulator::new();
    emu.load_rom("nestest/nestest.nes");
    emu.cpu.pc = 0xC000; // Automation mode

    // Run until test completes
    while emu.cpu.pc != 0xC66E {
        emu.step();
    }

    let result = emu.bus.read(0x0002);
    assert_eq!(result, 0x00, "nestest failed with code {:02X}", result);
}
```

---

## Performance Testing

Once functionality is correct, measure performance:

```rust
use std::time::Instant;

#[test]
fn benchmark_cpu_performance() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    let start = Instant::now();
    let target_cycles = 1_000_000; // 1 million cycles

    while cpu.cycles < target_cycles {
        cpu.step(&mut bus);
    }

    let duration = start.elapsed();
    let mhz = target_cycles as f64 / duration.as_secs_f64() / 1_000_000.0;

    println!("Emulated speed: {:.2} MHz", mhz);
    assert!(mhz > 10.0, "Too slow! Should easily exceed 10 MHz");
}
```

---

## Success Criteria

### Milestone 1: Basic Instructions Pass
- [ ] LDA/LDX/LDY work in all addressing modes
- [ ] STA/STX/STY work
- [ ] Simple arithmetic (ADC without carry edge cases)

### Milestone 2: All Official Opcodes Pass
- [ ] All 151 official opcodes implemented
- [ ] All addressing modes correct
- [ ] Cycle timing accurate

### Milestone 3: nestest Automation Mode Passes
- [ ] All 5000 lines of automation mode match reference
- [ ] $0002 reads $00 (pass code)
- [ ] No register/flag mismatches

### Milestone 4: Unofficial Opcodes Pass
- [ ] Stable unofficial opcodes implemented
- [ ] Combined operations (LAX, SAX, DCP, ISC, etc.)
- [ ] Unofficial NOPs

### Milestone 5: Full nestest Passes
- [ ] With minimal PPU support
- [ ] Visual output shows "Passed"

---

## References

- nestest ROM: https://github.com/christopherpow/nes-test-roms
- Test ROM documentation: https://www.nesdev.org/wiki/Emulator_tests
- blargg's test ROMs: https://github.com/christopherpow/nes-test-roms
- Nintendulator (reference emulator): https://www.qmtpro.com/~nes/nintendulator/
