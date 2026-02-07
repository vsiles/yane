# Quick Start Guide

## You Have Everything You Need!

Your project already has the nestest ROM and reference log in the `nestest/` directory:
- ✅ `nestest/nestest.nes` (25 KB) - The test ROM
- ✅ `nestest/nestest.log` (848 KB, 8991 lines) - Reference execution log

## Quick Reference

| What | Value |
|------|-------|
| **Start PC** | $C000 (automation mode) |
| **End PC** | $C66E |
| **Pass/Fail Check** | Read address $0002 (should be $00) |
| **Starting Cycles** | 7 |
| **Log Lines** | 8,991 total |

## Log Format at a Glance

```
C000  4C F5 C5  JMP $C5F5                       A:00 X:00 Y:00 P:24 SP:FD PPU:  0, 21 CYC:7
│     │         │                               │                          │         │
PC    Bytes     Instruction                    CPU Registers              PPU       Cycles
```

**Note**: You can ignore the PPU field for CPU-only development.

## 5-Minute Setup

### 1. Read the Documentation (10 minutes)
```bash
# Start here
cat docs/README.md

# Then read in this order:
cat docs/00-project-overview.md      # Overview
cat docs/05-implementation-roadmap.md # How to build it
cat docs/06-nestest-reference.md      # nestest details
```

### 2. Set Up Your Rust Project (5 minutes)
```bash
# Project structure is already suggested in 05-implementation-roadmap.md
# Follow "Phase 0: Project Setup"

cargo init .  # If not already initialized
```

### 3. Start Implementing (Phase 1)
```bash
# Follow the roadmap in 05-implementation-roadmap.md
# Start with Phase 1: CPU Registers and Status Flags
```

### 4. Test Early and Often
```bash
# After implementing a few opcodes, test immediately
cargo run > my_output.log 2>&1

# Compare with reference
diff nestest/nestest.log my_output.log | head -20
```

## Your First Goal

**Get the first instruction to match:**

Expected (from nestest.log):
```
C000  4C F5 C5  JMP $C5F5                       A:00 X:00 Y:00 P:24 SP:FD PPU:  0, 21 CYC:7
```

Your output should match (PPU can be dummy values):
```
C000  4C F5 C5  JMP $C5F5                       A:00 X:00 Y:00 P:24 SP:FD PPU:XXX,XXX CYC:7
```

## Common First-Time Issues

### Issue 1: Starting State Wrong
**Problem**: First line doesn't match registers

**Fix**: Initialize CPU to nestest automation mode state:
```rust
cpu.a = 0x00;
cpu.x = 0x00;
cpu.y = 0x00;
cpu.sp = 0xFD;
cpu.p = 0x24;    // I flag set (bit 2)
cpu.pc = 0xC000; // Automation mode
cpu.cycles = 7;  // Starting cycle count
```

### Issue 2: Log Format Wrong
**Problem**: Log doesn't match format

**Fix**: Use exact format (see 06-nestest-reference.md for details):
```rust
println!(
    "{:04X}  {:02X} {:02X} {:02X}  {:<30} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PPU:XXX,XXX CYC:{}",
    pc, byte1, byte2, byte3, instruction,
    a, x, y, p, sp, cycles
);
```

### Issue 3: Can't Load ROM
**Problem**: File not found or wrong format

**Fix**:
```rust
// Load the ROM
let rom_data = std::fs::read("nestest/nestest.nes")?;

// Parse iNES header (first 16 bytes)
// Header format: "NES\x1A" + 12 bytes of flags/info

// PRG ROM starts at byte 16
let prg_rom = &rom_data[16..16+16384]; // 16KB

// Map to CPU address space $8000-$BFFF and $C000-$FFFF
// (16KB ROM mirrored twice in 32KB space)
```

## Development Cycle

```
1. Choose next opcode from roadmap
   ↓
2. Write unit test for that opcode
   ↓
3. Implement the opcode
   ↓
4. Run unit test (should pass)
   ↓
5. Run nestest, compare logs
   ↓
6. Fix any differences
   ↓
7. Commit and repeat
```

## Milestone Checklist

### Week 1: Basic CPU
- [ ] CPU struct and registers
- [ ] Status flags (conversion to/from byte)
- [ ] Memory bus (read/write with mirroring)
- [ ] LDA immediate
- [ ] First nestest line matches!

### Week 2: Load/Store Complete
- [ ] All addressing modes
- [ ] All load/store opcodes
- [ ] Transfer instructions
- [ ] Stack operations
- [ ] First 100 nestest lines match

### Week 3: Arithmetic & Control Flow
- [ ] ADC, SBC (no decimal mode!)
- [ ] AND, ORA, EOR
- [ ] Branches (timing correct)
- [ ] JMP, JSR, RTS
- [ ] First 1000 nestest lines match

### Week 4: Complete CPU
- [ ] All official opcodes
- [ ] Unofficial opcodes
- [ ] Interrupts (BRK, NMI, IRQ, RTI)
- [ ] All 8991 nestest lines match!
- [ ] Address $0002 reads $00
- [ ] 🎉 CPU COMPLETE!

## When You Get Stuck

### 1. Check the Docs
- **Hardware quirks**: See `03-hardware-quirks.md`
- **Opcode reference**: See `02-cpu-opcodes.md`
- **nestest details**: See `06-nestest-reference.md`

### 2. Find First Difference
```bash
# Find where your log diverges
diff nestest/nestest.log my_output.log | head -5

# The FIRST different line is where your bug manifests
# The actual bug is likely in a PREVIOUS instruction
```

### 3. Isolate the Bug
```rust
// Write a minimal test for the failing instruction
#[test]
fn test_failing_instruction() {
    let mut cpu = Cpu::new();
    // Set exact state from nestest log
    cpu.a = 0x00;
    cpu.x = 0x00;
    // ... etc

    // Execute the failing instruction
    let cycles = cpu.step(&mut bus);

    // Check result
    assert_eq!(cpu.a, expected_value);
    assert_eq!(cycles, expected_cycles);
}
```

### 4. Ask for Help
- NESDev Forums: https://forums.nesdev.org/
- Provide: PC where it fails, expected vs actual registers
- Include: Your instruction implementation code

## Important Notes

### ✅ DO
- Log BEFORE executing each instruction
- Test each opcode individually first
- Compare logs frequently (every 10-50 instructions)
- Commit working code often
- Take breaks when stuck

### ❌ DON'T
- Don't implement decimal mode (doesn't exist on NES)
- Don't skip unofficial opcodes (nestest requires them)
- Don't ignore cycle counting (must be exact)
- Don't implement PPU yet (CPU first!)
- Don't give up (nestest is achievable!)

## Resources at a Glance

| Resource | Location |
|----------|----------|
| nestest ROM | `nestest/nestest.nes` |
| nestest log | `nestest/nestest.log` |
| Project docs | `docs/` directory |
| Quick ref | This file! |

## Next Steps

1. ✅ You've read this quick start
2. → Read `docs/05-implementation-roadmap.md`
3. → Start Phase 1: CPU Registers
4. → Implement, test, repeat!

---

**Remember**: Everyone's first emulator fails nestest many times. The difference between success and failure is persistence. Keep debugging, keep comparing logs, and you WILL get there! 🚀

Good luck!
