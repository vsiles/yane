# nestest ROM Reference

## Overview

This document provides detailed information about the nestest ROM files in the `nestest/` directory and how to use them for CPU validation.

## Files

### nestest.nes
- **Type**: iNES ROM image
- **Size**: 25 KB (24,592 bytes)
- **PRG ROM**: 1x 16KB bank
- **CHR ROM**: 1x 8KB bank
- **Mapper**: 0 (NROM)
- **Mirroring**: Horizontal
- **Format**: Standard iNES with 16-byte header

### nestest.log
- **Type**: Reference execution log from Nintendulator emulator
- **Size**: 848 KB (867,821 bytes)
- **Lines**: 8,991 lines
- **Coverage**: Complete test execution (both automation and full modes)

## Log File Format

### Complete Format Specification

```
C000  4C F5 C5  JMP $C5F5                       A:00 X:00 Y:00 P:24 SP:FD PPU:  0, 21 CYC:7
│     │         │                               │                          │         │
│     │         │                               └─ CPU registers (before)  │         └─ Total CPU cycles
│     │         └─ Disassembled instruction                                └─ PPU state (scanline, pixel)
│     └─ Instruction bytes (1-3 bytes, space-padded)
└─ Program Counter (PC)
```

### Field Details

#### Program Counter (PC)
- **Format**: 4 hex digits
- **Example**: `C000`
- **Notes**: Shows PC before instruction execution

#### Instruction Bytes
- **Format**: Up to 3 bytes in hex, space-padded
- **Examples**:
  - `4C F5 C5` (3-byte instruction: JMP absolute)
  - `A2 00   ` (2-byte instruction: LDX immediate)
  - `EA      ` (1-byte instruction: NOP)

#### Disassembled Instruction
- **Format**: Mnemonic + operands, left-padded to ~30 characters
- **Unofficial opcodes**: Prefixed with `*`
- **Examples**:
  - `JMP $C5F5` (official)
  - `*NOP $A9 = 00` (unofficial)
  - `STX $00 = 00` (shows value at address)

#### CPU Registers
- **Format**: `A:XX X:XX Y:XX P:XX SP:XX`
- **Values**: All in hexadecimal
- **Timing**: Shows state BEFORE instruction executes
- **P register**: Combined status flags as single byte

#### PPU State
- **Format**: `PPU:SSS,PPP`
- **SSS**: Scanline number (0-261)
- **PPP**: Pixel/dot position on scanline (0-340)
- **Notes**: Can be ignored for CPU-only testing in automation mode

#### CPU Cycles
- **Format**: `CYC:NNNNN`
- **Value**: Total CPU cycles executed since start
- **Starting value**: 7 (in automation mode at PC=$C000)
- **Notes**: This is the cumulative cycle count, not cycles per instruction

### Example Lines

#### Official Instruction
```
C5F5  A2 00     LDX #$00                        A:00 X:00 Y:00 P:24 SP:FD PPU:  0, 30 CYC:10
```
- PC = $C5F5
- Opcode $A2 (LDX immediate), operand $00
- Loads X register with value $00
- Registers before: A=$00, X=$00, Y=$00, P=$24 (00100100 = I flag set), SP=$FD
- PPU at scanline 0, pixel 30
- Total 10 cycles executed so far

#### Unofficial Instruction
```
C6BD  04 A9    *NOP $A9 = 00                    A:AA X:97 Y:4E P:EF SP:F9 PPU:128, 98 CYC:14582
```
- PC = $C6BD
- Opcode $04 (unofficial NOP, zero page), operand $A9
- Marked with `*` to indicate unofficial opcode
- Shows the value at address $A9 is $00
- 14,582 total cycles executed

#### Instruction with Memory Read
```
C5F7  86 00     STX $00 = 00                    A:00 X:00 Y:00 P:26 SP:FD PPU:  0, 36 CYC:12
```
- Shows value at target address: `$00 = 00`
- The value after `=` is what's currently at that address before the store

## Test Modes

### Automation Mode (Recommended for Initial Development)

**Start Address**: $C000

**Characteristics**:
- Self-contained CPU testing
- No PPU interaction required
- Tests all addressing modes
- Tests all official opcodes
- Tests many unofficial opcodes
- Tests flag behavior
- Tests cycle timing

**End Condition**: PC reaches $C66E

**Result Check**:
- Read byte at address $0002
- $00 = All tests passed
- Non-zero = Test failed (failure code)

**Log Coverage**: Approximately lines 1-5000 cover the main CPU tests in automation mode

### Full Mode (Requires PPU)

**Start Address**: RESET vector (loaded from $FFFC-$FFFD)

**Characteristics**:
- Tests CPU with PPU interaction
- Displays results on screen
- Requires minimal PPU implementation
- More comprehensive integration testing

**Recommended**: Implement after CPU passes automation mode

## Using nestest for Development

### Initial CPU-Only Testing

1. **Load the ROM**:
   ```rust
   let rom_data = std::fs::read("nestest/nestest.nes")?;
   // Parse iNES header (bytes 0-15)
   // Load PRG ROM starting at byte 16
   ```

2. **Set starting PC**:
   ```rust
   cpu.pc = 0xC000;  // Automation mode
   ```

3. **Initialize registers** (automation mode expects):
   ```rust
   cpu.a = 0x00;
   cpu.x = 0x00;
   cpu.y = 0x00;
   cpu.sp = 0xFD;
   cpu.p = 0x24;  // I flag set
   cpu.cycles = 7;  // Starting cycle count
   ```

4. **Run until completion**:
   ```rust
   while cpu.pc != 0xC66E {
       log_state(&cpu, &bus);
       cpu.step(&mut bus);
   }
   ```

5. **Check result**:
   ```rust
   let result = bus.read(0x0002);
   assert_eq!(result, 0x00);  // Pass
   ```

### Log Comparison

#### Method 1: Full Log Comparison
```bash
./your_emulator > output.log 2>&1
diff nestest/nestest.log output.log
```

#### Method 2: Line-by-Line with Context
```bash
diff -y nestest/nestest.log output.log | less
```

#### Method 3: First Difference Only
```bash
diff nestest/nestest.log output.log | head -20
```

#### Method 4: Automated Script
```bash
#!/bin/bash
./your_emulator > output.log 2>&1

# Find first difference
FIRST_DIFF=$(diff nestest/nestest.log output.log | grep -m1 "^<" | head -1)

if [ -z "$FIRST_DIFF" ]; then
    echo "✓ All lines match!"
    # Check result code
    RESULT=$(grep "PASSED\|FAILED" output.log | tail -1)
    echo "$RESULT"
else
    echo "✗ First difference found:"
    echo "$FIRST_DIFF"
    diff nestest/nestest.log output.log | head -30
fi
```

### Handling PPU Field

Since automation mode doesn't require PPU, you have three options:

#### Option 1: Ignore PPU in Comparison
Strip PPU field from both logs before comparing:
```bash
sed 's/PPU:[^C]*//g' nestest/nestest.log > ref_no_ppu.log
sed 's/PPU:[^C]*//g' output.log > out_no_ppu.log
diff ref_no_ppu.log out_no_ppu.log
```

#### Option 2: Use Placeholder PPU Values
Output dummy PPU values in your log:
```rust
println!("... PPU:  0,  0 CYC:{}", cycles);
```
Then manually verify CPU state only.

#### Option 3: Implement Minimal PPU Counter
Track PPU cycles (3x CPU cycles) and scanline/pixel:
```rust
ppu_cycles = cpu_cycles * 3;
scanline = ppu_cycles / 341;
pixel = ppu_cycles % 341;
```

## Common Test Failures

### Early Failures (First 100 Instructions)

If nestest fails very early:
- Check power-up state (registers initialized correctly)
- Verify PC starts at $C000
- Check cycle counter starts at 7
- Verify basic load/store opcodes (LDA, STA, LDX, STX)

### Mid-Range Failures (Instructions 100-1000)

Common causes:
- Addressing mode bugs (indexed, indirect)
- Flag calculation errors (especially overflow)
- Cycle counting off (page crossing)
- Branch timing incorrect

### Late Failures (Instructions 1000+)

Usually indicates:
- Unofficial opcode missing or wrong
- Rare edge case in flag behavior
- Interrupt handling issue
- Stack operation bug

### Cycle Drift

If cycles diverge gradually:
- Page boundary crossing not adding cycles
- Branch timing wrong (taken vs not taken)
- RMW instructions missing dummy write cycle
- Addressing mode taking wrong number of cycles

## nestest Test Coverage

### Addressing Modes Tested
- ✓ Immediate
- ✓ Zero Page
- ✓ Zero Page,X
- ✓ Zero Page,Y
- ✓ Absolute
- ✓ Absolute,X
- ✓ Absolute,Y
- ✓ Indirect (JMP only)
- ✓ Indexed Indirect (d,X)
- ✓ Indirect Indexed (d),Y
- ✓ Relative (branches)
- ✓ Accumulator
- ✓ Implied

### Instruction Categories Tested
- ✓ Load/Store (LDA, LDX, LDY, STA, STX, STY)
- ✓ Transfers (TAX, TXA, TAY, TYA, TSX, TXS)
- ✓ Stack (PHA, PLA, PHP, PLP)
- ✓ Logic (AND, ORA, EOR, BIT)
- ✓ Arithmetic (ADC, SBC)
- ✓ Increment/Decrement (INC, DEC, INX, INY, DEX, DEY)
- ✓ Shifts/Rotates (ASL, LSR, ROL, ROR)
- ✓ Comparisons (CMP, CPX, CPY)
- ✓ Branches (BCC, BCS, BEQ, BNE, BMI, BPL, BVC, BVS)
- ✓ Jumps/Calls (JMP, JSR, RTS)
- ✓ Interrupts (BRK, RTI)
- ✓ Flags (CLC, SEC, CLI, SEI, CLD, SED, CLV)
- ✓ No-op (NOP)

### Unofficial Opcodes Tested
- ✓ LAX (Load A and X)
- ✓ SAX (Store A AND X)
- ✓ DCP (DEC + CMP)
- ✓ ISC (INC + SBC)
- ✓ SLO (ASL + ORA)
- ✓ RLA (ROL + AND)
- ✓ SRE (LSR + EOR)
- ✓ RRA (ROR + ADC)
- ✓ Unofficial NOPs (various forms)

### Hardware Quirks Tested
- ✓ Page boundary crossing (+1 cycle)
- ✓ Branch timing (taken/not taken, page cross)
- ✓ Dummy reads in addressing modes
- ✓ Dummy writes in RMW instructions
- ✓ Flag behavior (N, V, Z, C)
- ✓ Indirect JMP bug (tested in full mode)

## Tips for Passing nestest

### 1. Start Simple
Implement opcodes in this order:
1. LDA immediate
2. LDA zero page
3. STA zero page
4. Other load/store variants
5. Branch to the first test failure

### 2. Log Everything
Print every instruction before execution to match nestest.log format exactly.

### 3. Compare Early and Often
Don't wait until the end—compare logs after every 100 instructions implemented.

### 4. Fix First Divergence
The first line that differs is your bug. Fix it before continuing.

### 5. Test Individual Instructions
When a bug is found, write a unit test for that specific instruction.

### 6. Ignore PPU Initially
Focus purely on CPU registers and cycle count. PPU can be dummy values.

### 7. Use Binary Search
If you can't find the bug, binary search the opcode implementations to isolate it.

## Success Criteria

### Phase 1: First 100 Lines Match
- Basic load/store working
- Simple addressing modes correct
- No cycle drift

### Phase 2: First 1000 Lines Match
- Most addressing modes working
- Arithmetic and logic correct
- Flags calculated properly

### Phase 3: All Lines Match
- All official opcodes work
- All required unofficial opcodes work
- Cycle-perfect timing
- Result at $0002 is $00

### Phase 4: Visual Confirmation (Full Mode)
- Minimal PPU implemented
- Screen displays "PASSED"
- Complete integration test passes

## Additional Resources

- **nestest documentation**: Included in nes-test-roms repository
- **Nintendulator**: Reference emulator used to generate the log
  - http://www.qmtpro.com/~nes/nintendulator/
- **nesdev Forums**: Community help for nestest issues
  - https://forums.nesdev.org/

## Quick Reference

| Item | Value |
|------|-------|
| Automation Start PC | $C000 |
| Automation End PC | $C66E |
| Result Address | $0002 |
| Pass Code | $00 |
| Initial Cycles | 7 |
| Total Log Lines | 8,991 |
| ROM Size | 25 KB |
| Mapper | 0 (NROM) |
| Required Unofficial | Yes |
| Required PPU (auto) | No |
| Required PPU (full) | Yes (minimal) |

---

**Remember**: nestest is your best friend. When it passes, your CPU is very likely correct!
