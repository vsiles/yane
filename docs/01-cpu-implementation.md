# CPU Implementation Guide

## Overview

The NES CPU is a Ricoh 2A03 (NTSC) or 2A07 (PAL), which is based on the MOS 6502 processor with one critical difference: **no decimal mode**. The D (decimal) flag exists and can be set/cleared, but it has no effect on arithmetic operations.

## CPU Architecture

### Registers

```rust
// Proposed Rust structure
pub struct Cpu {
    // 8-bit registers
    a: u8,           // Accumulator
    x: u8,           // X index register
    y: u8,           // Y index register
    sp: u8,          // Stack pointer (points to $0100-$01FF)
    p: StatusFlags,  // Processor status flags

    // 16-bit register
    pc: u16,         // Program counter

    // Cycle counting
    cycles: u64,     // Total cycles executed
}
```

### Status Flags Register (P)

Layout: `NV-B DIZC` (bits 7-0)

```rust
pub struct StatusFlags {
    carry:     bool,  // C (bit 0): Carry flag
    zero:      bool,  // Z (bit 1): Zero flag
    interrupt: bool,  // I (bit 2): Interrupt disable
    decimal:   bool,  // D (bit 3): Decimal mode (no effect on NES!)
    overflow:  bool,  // V (bit 6): Overflow flag
    negative:  bool,  // N (bit 7): Negative flag
}
```

**Special Notes on Status Flags:**

1. **B Flag (bit 4)**: This is NOT a real CPU register flag. It only appears when P is pushed to the stack:
   - B = 1 when pushed by PHP or BRK instruction
   - B = 0 when pushed by hardware interrupt (NMI/IRQ)
   - Bit 5 is always set to 1 when pushed
   - This allows distinguishing IRQ from BRK in interrupt handlers

2. **Decimal Flag**: Can be set/cleared with SED/CLD but has NO effect on ADC/SBC operations

### Power-Up State

```
A:  $00
X:  $00
Y:  $00
SP: $FD (which is $100 - 3)
P:  $34 (I flag set, others clear)
PC: Loaded from vector at $FFFC-$FFFD
```

### Reset State

After RESET (not power-up):
- A, X, Y, P: Unchanged
- SP: Decremented by 3
- I flag: Set to 1
- PC: Loaded from $FFFC-$FFFD

## Clock Timing

### NTSC (Target for Initial Implementation)
- Master Clock: 21.47727 MHz
- CPU Clock: ~1.79 MHz (master ÷ 12)
- Cycle Duration: ~559 nanoseconds

### Critical Timing Rule
**"Every cycle on 6502 is either a read or a write cycle"**

This means:
- No idle cycles
- Every instruction takes at least 2 cycles (opcode fetch + execute)
- Memory operations add cycles
- Page boundary crossings add "oops" cycles
- Dummy reads/writes occur during certain operations

## Addressing Modes

The 6502 has 13 addressing modes (including variations):

### 1. Implied/Implicit
No operand needed. Operation is implied by opcode.
```
Examples: NOP, CLC, SEC, RTS
```

### 2. Accumulator
Operates directly on accumulator.
```
Examples: ASL A, LSR A, ROL A, ROR A
```

### 3. Immediate
Uses the next byte as the value (not an address).
```
Format: #$nn
Example: LDA #$42    ; Load literal value $42
Cycles: 2
```

### 4. Zero Page
Uses an 8-bit address in the zero page ($0000-$00FF).
```
Format: $nn
Example: LDA $80     ; Load from address $0080
Cycles: 3 (read) or 3 (write)
```

### 5. Zero Page,X
Zero page address + X register.
```
Format: $nn,X
Example: LDA $80,X   ; Load from address ($80 + X) & $FF
Cycles: 4
Note: Wraps within zero page (high byte always $00)
```

### 6. Zero Page,Y
Zero page address + Y register (only for LDX/STX).
```
Format: $nn,Y
Example: LDX $80,Y
Cycles: 4
```

### 7. Relative
8-bit signed offset for branch instructions.
```
Format: label
Example: BNE label
Cycles: 2 (not taken), 3 (taken, same page), 4 (taken, page cross)
```

### 8. Absolute
Uses a 16-bit address.
```
Format: $nnnn
Example: LDA $8000
Cycles: 4 (read) or 4 (write)
```

### 9. Absolute,X
Absolute address + X register.
```
Format: $nnnn,X
Example: LDA $8000,X
Cycles: 4+ (add 1 if page boundary crossed on reads)
        5 (always for writes and RMW instructions)
```

### 10. Absolute,Y
Absolute address + Y register.
```
Format: $nnnn,Y
Example: LDA $8000,Y
Cycles: 4+ (add 1 if page boundary crossed on reads)
        5 (always for writes)
```

### 11. Indirect (JMP only)
16-bit pointer to the actual address.
```
Format: ($nnnn)
Example: JMP ($FFFC)
Cycles: 5

HARDWARE BUG: If the low byte is $FF, high byte is fetched from
$nn00 instead of $nn+1|00. Example:
  JMP ($10FF) fetches low byte from $10FF, high byte from $1000
  instead of $1100.
```

### 12. Indexed Indirect (d,X)
Zero page address + X, then read 16-bit pointer.
```
Format: ($nn,X)
Example: LDA ($80,X)
Process:
  1. Add X to $80 (wraps in zero page)
  2. Read 16-bit address from that location
  3. Access final address
Cycles: 6
```

### 13. Indirect Indexed (d),Y
Read 16-bit pointer from zero page, then add Y.
```
Format: ($nn),Y
Example: LDA ($80),Y
Process:
  1. Read 16-bit address from $80
  2. Add Y to that address
  3. Access final address
Cycles: 5+ (add 1 if page boundary crossed on reads)
        6 (always for writes)

Note: This is the most common indexed mode on NES due to
abundant zero page space.
```

## Memory Access Patterns

### Page Boundary Crossing ("Oops Cycle")

When an indexed addressing mode crosses a page boundary:
- **Read instructions**: Add 1 cycle (re-read with corrected high byte)
- **Write/RMW instructions**: Always take the extra cycle (dummy read)

Example:
```
LDA $10FF,X  where X = $01
  - Calculated address: $1100 (crossed from page $10 to $11)
  - Cycle 1: Fetch opcode
  - Cycle 2: Fetch $FF (low byte)
  - Cycle 3: Fetch $10 (high byte)
  - Cycle 4: Read from $10FF + carry = $1000 (dummy, wrong page)
  - Cycle 5: Re-read from $1100 (correct)
```

### Dummy Reads and Writes

The 6502 performs dummy memory accesses during certain operations:

1. **Read-Modify-Write (RMW) instructions**:
   ```
   INC $80
     Cycle 1: Fetch opcode
     Cycle 2: Fetch address
     Cycle 3: Read from address
     Cycle 4: Write original value back (dummy write)
     Cycle 5: Write modified value
   ```

2. **Indexed addressing**:
   - Dummy read when fixing page crossing

3. **Stack operations**:
   - PHA: Dummy read from stack before write
   - PLA: Dummy read, increment SP, then real read

## Instruction Cycle Breakdown

### Minimum 2 Cycles
All instructions take at least 2 cycles:
- Cycle 1: Fetch opcode
- Cycle 2+: Execute

### Memory Operations
- Each read: +1 cycle
- Each write: +1 cycle

### Stack Operations
- Push: +1 cycle (access stack memory)
- Pull: +2 cycles (dummy read + actual read)

### RMW Operations
Extra cycle for dummy write back:
```
ASL, LSR, ROL, ROR (memory)
INC, DEC (memory)
```

### Branch Instructions
- Not taken: 2 cycles
- Taken (same page): 3 cycles
- Taken (page cross): 4 cycles

## Interrupt Handling

### Interrupt Types

1. **RESET**: Hardware reset
2. **NMI**: Non-maskable interrupt (edge-triggered)
3. **IRQ**: Interrupt request (level-triggered)
4. **BRK**: Software interrupt

### Interrupt Priority

If multiple interrupts occur simultaneously:
1. RESET (highest)
2. NMI
3. IRQ/BRK (lowest)

### Interrupt Polling

**Critical Detail**: Interrupts are polled at the end of each instruction, but "it's really the status of the interrupt lines at the end of the second-to-last cycle that matters."

### Interrupt Sequence (7 cycles)

```
For NMI or IRQ:
  Cycle 1: Dummy read of next instruction byte (PC not incremented)
  Cycle 2: Dummy read of next instruction byte (PC not incremented)
  Cycle 3: Push PCH to stack
  Cycle 4: Push PCL to stack
  Cycle 5: Push P to stack (B=0 for hardware interrupts)
  Cycle 6: Fetch vector low byte
  Cycle 7: Fetch vector high byte, set I flag (IRQ only)
```

For BRK (software interrupt):
```
  Cycle 1: Fetch BRK opcode
  Cycle 2: Dummy read (PC already incremented past signature byte)
  Cycle 3: Push PCH to stack
  Cycle 4: Push PCL to stack
  Cycle 5: Push P to stack (B=1, bit 5=1)
  Cycle 6: Fetch IRQ vector low byte
  Cycle 7: Fetch IRQ vector high byte, set I flag
```

### Interrupt Vectors

```
$FFFA-$FFFB: NMI vector
$FFFC-$FFFD: RESET vector
$FFFE-$FFFF: IRQ/BRK vector (shared!)
```

### NMI Edge Detection

NMI is edge-triggered (high-to-low transition):
- Internal signal goes high during φ1 after edge detected
- Stays high until NMI handled
- Multiple NMI edges before handling = only one NMI serviced

### IRQ Level Detection

IRQ is level-triggered:
- Checked during φ1 if signal was low during prior φ2
- I flag masks IRQ (but not NMI or BRK)

### Interrupt Hijacking

**Critical Bug to Emulate**:

If NMI occurs during the first 4 cycles of BRK or IRQ handler:
- The push sequence completes normally
- But the vector fetched is from NMI instead of IRQ
- Stack has B=0 (indicating hardware interrupt)

This happens because NMI can interrupt the IRQ sequence before vector fetch.

### CLI/SEI/PLP Delay

After CLI, SEI, or PLP changes the I flag:
- The change takes effect AFTER interrupt polling
- This can delay interrupt handling by one instruction

## Implementation Strategy

### Phase 1: Basic Structure
1. Define CPU struct with registers
2. Implement status flag manipulation
3. Create memory bus interface
4. Implement fetch-decode-execute loop

### Phase 2: Simple Instructions
1. Load/Store: LDA, LDX, LDY, STA, STX, STY
2. Transfers: TAX, TAY, TXA, TYA, TSX, TXS
3. Stack: PHA, PLA, PHP, PLP
4. Implement immediate and zero page addressing

### Phase 3: Arithmetic & Logic
1. ADC, SBC (without decimal mode)
2. AND, ORA, EOR
3. CMP, CPX, CPY
4. INC, DEC, INX, DEX, INY, DEY

### Phase 4: Shifts & Rotates
1. ASL, LSR, ROL, ROR
2. Both accumulator and memory versions

### Phase 5: Branches & Jumps
1. Conditional branches: BCC, BCS, BEQ, BNE, BMI, BPL, BVC, BVS
2. JMP (absolute and indirect)
3. JSR, RTS

### Phase 6: Flags & Control
1. CLC, SEC, CLI, SEI, CLD, SED, CLV
2. NOP
3. BIT

### Phase 7: Interrupts
1. BRK
2. RTI
3. NMI handling
4. IRQ handling

### Phase 8: Unofficial Opcodes
Implement all unofficial opcodes (see 02-cpu-opcodes.md for details)

## Cycle Counting Implementation

### Approach
Track cycles explicitly for each operation:

```rust
impl Cpu {
    fn step(&mut self, bus: &mut Bus) -> u8 {
        let opcode = self.fetch_byte(bus);
        let cycles = self.execute(opcode, bus);
        self.cycles += cycles as u64;
        cycles
    }

    fn fetch_byte(&mut self, bus: &mut Bus) -> u8 {
        let byte = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }
}
```

### Timing Verification
- Log each instruction with cycle count
- Compare against nestest reference log
- Verify page boundary crossing detection
- Verify branch timing (taken vs not taken)

## Testing Approach

1. **Unit tests**: Test each opcode individually
2. **Integration tests**: Small hand-written programs
3. **nestest**: Comprehensive validation
4. **Automated log comparison**: Compare execution trace with reference

See `04-testing-guide.md` for detailed testing procedures.
