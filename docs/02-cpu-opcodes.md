# CPU Opcodes Reference

## Overview

The NES CPU has 256 possible opcodes (8-bit opcode space). Of these:
- **151 official opcodes** (documented by MOS Technology)
- **105 unofficial opcodes** (undocumented but functional)

**Important**: nestest requires unofficial opcodes to be implemented. Several commercial games also use them.

## Opcode Format

```
Opcode byte format: aaabbbcc

Where:
  aaa = Instruction group (3 bits)
  bbb = Addressing mode (3 bits)
  cc  = Instruction type (2 bits)
```

This pattern creates the instruction matrix, though there are many exceptions.

## Official Opcodes by Category

### Load/Store Operations

| Opcode | Instruction | Addr Mode | Cycles | Notes |
|--------|-------------|-----------|--------|-------|
| A9 | LDA #$nn | Immediate | 2 | Load accumulator |
| A5 | LDA $nn | Zero Page | 3 | |
| B5 | LDA $nn,X | Zero Page,X | 4 | |
| AD | LDA $nnnn | Absolute | 4 | |
| BD | LDA $nnnn,X | Absolute,X | 4+ | +1 if page cross |
| B9 | LDA $nnnn,Y | Absolute,Y | 4+ | +1 if page cross |
| A1 | LDA ($nn,X) | Indexed Indirect | 6 | |
| B1 | LDA ($nn),Y | Indirect Indexed | 5+ | +1 if page cross |
| A2 | LDX #$nn | Immediate | 2 | Load X register |
| A6 | LDX $nn | Zero Page | 3 | |
| B6 | LDX $nn,Y | Zero Page,Y | 4 | |
| AE | LDX $nnnn | Absolute | 4 | |
| BE | LDX $nnnn,Y | Absolute,Y | 4+ | +1 if page cross |
| A0 | LDY #$nn | Immediate | 2 | Load Y register |
| A4 | LDY $nn | Zero Page | 3 | |
| B4 | LDY $nn,X | Zero Page,X | 4 | |
| AC | LDY $nnnn | Absolute | 4 | |
| BC | LDY $nnnn,X | Absolute,X | 4+ | +1 if page cross |
| 85 | STA $nn | Zero Page | 3 | Store accumulator |
| 95 | STA $nn,X | Zero Page,X | 4 | |
| 8D | STA $nnnn | Absolute | 4 | |
| 9D | STA $nnnn,X | Absolute,X | 5 | No page cross penalty |
| 99 | STA $nnnn,Y | Absolute,Y | 5 | No page cross penalty |
| 81 | STA ($nn,X) | Indexed Indirect | 6 | |
| 91 | STA ($nn),Y | Indirect Indexed | 6 | No page cross penalty |
| 86 | STX $nn | Zero Page | 3 | Store X register |
| 96 | STX $nn,Y | Zero Page,Y | 4 | |
| 8E | STX $nnnn | Absolute | 4 | |
| 84 | STY $nn | Zero Page | 3 | Store Y register |
| 94 | STY $nn,X | Zero Page,X | 4 | |
| 8C | STY $nnnn | Absolute | 4 | |

### Register Transfers

| Opcode | Instruction | Cycles | Flags | Notes |
|--------|-------------|--------|-------|-------|
| AA | TAX | 2 | NZ | Transfer A to X |
| A8 | TAY | 2 | NZ | Transfer A to Y |
| 8A | TXA | 2 | NZ | Transfer X to A |
| 98 | TYA | 2 | NZ | Transfer Y to A |
| BA | TSX | 2 | NZ | Transfer SP to X |
| 9A | TXS | 2 | - | Transfer X to SP |

### Stack Operations

| Opcode | Instruction | Cycles | Notes |
|--------|-------------|--------|-------|
| 48 | PHA | 3 | Push accumulator |
| 08 | PHP | 3 | Push processor status (B=1, bit5=1) |
| 68 | PLA | 4 | Pull accumulator (sets NZ) |
| 28 | PLP | 4 | Pull processor status |

### Logical Operations

| Opcode | Instruction | Addr Mode | Cycles | Notes |
|--------|-------------|-----------|--------|-------|
| 29 | AND #$nn | Immediate | 2 | A = A & value |
| 25 | AND $nn | Zero Page | 3 | Sets NZ flags |
| 35 | AND $nn,X | Zero Page,X | 4 | |
| 2D | AND $nnnn | Absolute | 4 | |
| 3D | AND $nnnn,X | Absolute,X | 4+ | |
| 39 | AND $nnnn,Y | Absolute,Y | 4+ | |
| 21 | AND ($nn,X) | Indexed Indirect | 6 | |
| 31 | AND ($nn),Y | Indirect Indexed | 5+ | |
| 09 | ORA #$nn | Immediate | 2 | A = A | value |
| 05 | ORA $nn | Zero Page | 3 | Sets NZ flags |
| 15 | ORA $nn,X | Zero Page,X | 4 | |
| 0D | ORA $nnnn | Absolute | 4 | |
| 1D | ORA $nnnn,X | Absolute,X | 4+ | |
| 19 | ORA $nnnn,Y | Absolute,Y | 4+ | |
| 01 | ORA ($nn,X) | Indexed Indirect | 6 | |
| 11 | ORA ($nn),Y | Indirect Indexed | 5+ | |
| 49 | EOR #$nn | Immediate | 2 | A = A ^ value |
| 45 | EOR $nn | Zero Page | 3 | Sets NZ flags |
| 55 | EOR $nn,X | Zero Page,X | 4 | |
| 4D | EOR $nnnn | Absolute | 4 | |
| 5D | EOR $nnnn,X | Absolute,X | 4+ | |
| 59 | EOR $nnnn,Y | Absolute,Y | 4+ | |
| 41 | EOR ($nn,X) | Indexed Indirect | 6 | |
| 51 | EOR ($nn),Y | Indirect Indexed | 5+ | |
| 24 | BIT $nn | Zero Page | 3 | N=bit7, V=bit6, Z=(A&val==0) |
| 2C | BIT $nnnn | Absolute | 4 | |

### Arithmetic Operations

| Opcode | Instruction | Addr Mode | Cycles | Notes |
|--------|-------------|-----------|--------|-------|
| 69 | ADC #$nn | Immediate | 2 | A = A + value + C |
| 65 | ADC $nn | Zero Page | 3 | Sets NVZC flags |
| 75 | ADC $nn,X | Zero Page,X | 4 | No decimal mode on NES! |
| 6D | ADC $nnnn | Absolute | 4 | |
| 7D | ADC $nnnn,X | Absolute,X | 4+ | |
| 79 | ADC $nnnn,Y | Absolute,Y | 4+ | |
| 61 | ADC ($nn,X) | Indexed Indirect | 6 | |
| 71 | ADC ($nn),Y | Indirect Indexed | 5+ | |
| E9 | SBC #$nn | Immediate | 2 | A = A - value - (1-C) |
| E5 | SBC $nn | Zero Page | 3 | Sets NVZC flags |
| F5 | SBC $nn,X | Zero Page,X | 4 | No decimal mode on NES! |
| ED | SBC $nnnn | Absolute | 4 | |
| FD | SBC $nnnn,X | Absolute,X | 4+ | |
| F9 | SBC $nnnn,Y | Absolute,Y | 4+ | |
| E1 | SBC ($nn,X) | Indexed Indirect | 6 | |
| F1 | SBC ($nn),Y | Indirect Indexed | 5+ | |

### Increment/Decrement

| Opcode | Instruction | Addr Mode | Cycles | Notes |
|--------|-------------|-----------|--------|-------|
| E6 | INC $nn | Zero Page | 5 | Memory++ (sets NZ) |
| F6 | INC $nn,X | Zero Page,X | 6 | |
| EE | INC $nnnn | Absolute | 6 | |
| FE | INC $nnnn,X | Absolute,X | 7 | |
| C6 | DEC $nn | Zero Page | 5 | Memory-- (sets NZ) |
| D6 | DEC $nn,X | Zero Page,X | 6 | |
| CE | DEC $nnnn | Absolute | 6 | |
| DE | DEC $nnnn,X | Absolute,X | 7 | |
| E8 | INX | Implicit | 2 | X++ (sets NZ) |
| C8 | INY | Implicit | 2 | Y++ (sets NZ) |
| CA | DEX | Implicit | 2 | X-- (sets NZ) |
| 88 | DEY | Implicit | 2 | Y-- (sets NZ) |

### Shifts & Rotates

| Opcode | Instruction | Addr Mode | Cycles | Notes |
|--------|-------------|-----------|--------|-------|
| 0A | ASL A | Accumulator | 2 | Shift left, bit 0 = 0 |
| 06 | ASL $nn | Zero Page | 5 | Sets NZC flags |
| 16 | ASL $nn,X | Zero Page,X | 6 | |
| 0E | ASL $nnnn | Absolute | 6 | |
| 1E | ASL $nnnn,X | Absolute,X | 7 | |
| 4A | LSR A | Accumulator | 2 | Shift right, bit 7 = 0 |
| 46 | LSR $nn | Zero Page | 5 | Sets NZC flags |
| 56 | LSR $nn,X | Zero Page,X | 6 | |
| 4E | LSR $nnnn | Absolute | 6 | |
| 5E | LSR $nnnn,X | Absolute,X | 7 | |
| 2A | ROL A | Accumulator | 2 | Rotate left through carry |
| 26 | ROL $nn | Zero Page | 5 | Sets NZC flags |
| 36 | ROL $nn,X | Zero Page,X | 6 | |
| 2E | ROL $nnnn | Absolute | 6 | |
| 3E | ROL $nnnn,X | Absolute,X | 7 | |
| 6A | ROR A | Accumulator | 2 | Rotate right through carry |
| 66 | ROR $nn | Zero Page | 5 | Sets NZC flags |
| 76 | ROR $nn,X | Zero Page,X | 6 | |
| 6E | ROR $nnnn | Absolute | 6 | |
| 7E | ROR $nnnn,X | Absolute,X | 7 | |

### Jumps & Calls

| Opcode | Instruction | Addr Mode | Cycles | Notes |
|--------|-------------|-----------|--------|-------|
| 4C | JMP $nnnn | Absolute | 3 | Jump to address |
| 6C | JMP ($nnnn) | Indirect | 5 | Has page boundary bug! |
| 20 | JSR $nnnn | Absolute | 6 | Jump to subroutine |
| 60 | RTS | Implicit | 6 | Return from subroutine |

### Branches

All branches: 2 cycles if not taken, 3 if taken (same page), 4 if taken (page cross)

| Opcode | Instruction | Test | Notes |
|--------|-------------|------|-------|
| 90 | BCC label | C = 0 | Branch if carry clear |
| B0 | BCS label | C = 1 | Branch if carry set |
| F0 | BEQ label | Z = 1 | Branch if equal (zero) |
| D0 | BNE label | Z = 0 | Branch if not equal |
| 30 | BMI label | N = 1 | Branch if minus (negative) |
| 10 | BPL label | N = 0 | Branch if plus (positive) |
| 50 | BVC label | V = 0 | Branch if overflow clear |
| 70 | BVS label | V = 1 | Branch if overflow set |

### Comparisons

All comparisons perform a subtraction (A/X/Y - value) without storing result, only setting flags.

| Opcode | Instruction | Addr Mode | Cycles | Flags |
|--------|-------------|-----------|--------|-------|
| C9 | CMP #$nn | Immediate | 2 | NZC |
| C5 | CMP $nn | Zero Page | 3 | Compare with A |
| D5 | CMP $nn,X | Zero Page,X | 4 | C=1 if A>=value |
| CD | CMP $nnnn | Absolute | 4 | Z=1 if A==value |
| DD | CMP $nnnn,X | Absolute,X | 4+ | N=bit7 of result |
| D9 | CMP $nnnn,Y | Absolute,Y | 4+ | |
| C1 | CMP ($nn,X) | Indexed Indirect | 6 | |
| D1 | CMP ($nn),Y | Indirect Indexed | 5+ | |
| E0 | CPX #$nn | Immediate | 2 | Compare with X |
| E4 | CPX $nn | Zero Page | 3 | Sets NZC |
| EC | CPX $nnnn | Absolute | 4 | |
| C0 | CPY #$nn | Immediate | 2 | Compare with Y |
| C4 | CPY $nn | Zero Page | 3 | Sets NZC |
| CC | CPY $nnnn | Absolute | 4 | |

### Flag Operations

| Opcode | Instruction | Cycles | Operation |
|--------|-------------|--------|-----------|
| 18 | CLC | 2 | Clear carry flag |
| 38 | SEC | 2 | Set carry flag |
| 58 | CLI | 2 | Clear interrupt disable |
| 78 | SEI | 2 | Set interrupt disable |
| D8 | CLD | 2 | Clear decimal (no effect on NES) |
| F8 | SED | 2 | Set decimal (no effect on NES) |
| B8 | CLV | 2 | Clear overflow flag |

### System Operations

| Opcode | Instruction | Cycles | Notes |
|--------|-------------|--------|-------|
| 00 | BRK | 7 | Software interrupt |
| 40 | RTI | 6 | Return from interrupt |
| EA | NOP | 2 | No operation |

## Unofficial Opcodes

**Required for nestest and some commercial games!**

### Categories of Unofficial Opcodes

1. **Combined Operations**: Perform two operations in one instruction
2. **NOPs**: Various multi-byte NOPs with different timings
3. **Unstable Operations**: KIL (halts CPU), weird behaviors

### Commonly Used Unofficial Opcodes

#### LAX (Load A and X)
Loads a value into both A and X simultaneously.

| Opcode | Addr Mode | Cycles |
|--------|-----------|--------|
| A7 | LAX $nn | 3 |
| B7 | LAX $nn,Y | 4 |
| AF | LAX $nnnn | 4 |
| BF | LAX $nnnn,Y | 4+ |
| A3 | LAX ($nn,X) | 6 |
| B3 | LAX ($nn),Y | 5+ |

#### SAX (Store A AND X)
Stores A & X (bitwise AND) to memory.

| Opcode | Addr Mode | Cycles |
|--------|-----------|--------|
| 87 | SAX $nn | 3 |
| 97 | SAX $nn,Y | 4 |
| 8F | SAX $nnnn | 4 |
| 83 | SAX ($nn,X) | 6 |

#### DCP (Decrement and Compare)
Decrements memory, then compares with A (DEC + CMP).

| Opcode | Addr Mode | Cycles |
|--------|-----------|--------|
| C7 | DCP $nn | 5 |
| D7 | DCP $nn,X | 6 |
| CF | DCP $nnnn | 6 |
| DF | DCP $nnnn,X | 7 |
| DB | DCP $nnnn,Y | 7 |
| C3 | DCP ($nn,X) | 8 |
| D3 | DCP ($nn),Y | 8 |

#### ISC/ISB (Increment and Subtract with Carry)
Increments memory, then subtracts from A (INC + SBC).

| Opcode | Addr Mode | Cycles |
|--------|-----------|--------|
| E7 | ISC $nn | 5 |
| F7 | ISC $nn,X | 6 |
| EF | ISC $nnnn | 6 |
| FF | ISC $nnnn,X | 7 |
| FB | ISC $nnnn,Y | 7 |
| E3 | ISC ($nn,X) | 8 |
| F3 | ISC ($nn),Y | 8 |

#### SLO (Shift Left and OR)
Shifts left, then ORs with A (ASL + ORA).

| Opcode | Addr Mode | Cycles |
|--------|-----------|--------|
| 07 | SLO $nn | 5 |
| 17 | SLO $nn,X | 6 |
| 0F | SLO $nnnn | 6 |
| 1F | SLO $nnnn,X | 7 |
| 1B | SLO $nnnn,Y | 7 |
| 03 | SLO ($nn,X) | 8 |
| 13 | SLO ($nn),Y | 8 |

#### RLA (Rotate Left and AND)
Rotates left, then ANDs with A (ROL + AND).

| Opcode | Addr Mode | Cycles |
|--------|-----------|--------|
| 27 | RLA $nn | 5 |
| 37 | RLA $nn,X | 6 |
| 2F | RLA $nnnn | 6 |
| 3F | RLA $nnnn,X | 7 |
| 3B | RLA $nnnn,Y | 7 |
| 23 | RLA ($nn,X) | 8 |
| 33 | RLA ($nn),Y | 8 |

#### SRE (Shift Right and EOR)
Shifts right, then XORs with A (LSR + EOR).

| Opcode | Addr Mode | Cycles |
|--------|-----------|--------|
| 47 | SRE $nn | 5 |
| 57 | SRE $nn,X | 6 |
| 4F | SRE $nnnn | 6 |
| 5F | SRE $nnnn,X | 7 |
| 5B | SRE $nnnn,Y | 7 |
| 43 | SRE ($nn,X) | 8 |
| 53 | SRE ($nn),Y | 8 |

#### RRA (Rotate Right and Add)
Rotates right, then adds to A (ROR + ADC).

| Opcode | Addr Mode | Cycles |
|--------|-----------|--------|
| 67 | RRA $nn | 5 |
| 77 | RRA $nn,X | 6 |
| 6F | RRA $nnnn | 6 |
| 7F | RRA $nnnn,X | 7 |
| 7B | RRA $nnnn,Y | 7 |
| 63 | RRA ($nn,X) | 8 |
| 73 | RRA ($nn),Y | 8 |

### Unofficial NOPs

Multiple opcodes act as NOPs with various timings and byte lengths:

| Opcode(s) | Bytes | Cycles | Notes |
|-----------|-------|--------|-------|
| 1A, 3A, 5A, 7A, DA, FA | 1 | 2 | Single-byte NOP |
| 80, 82, 89, C2, E2 | 2 | 2 | Two-byte NOP (immediate) |
| 04, 44, 64 | 2 | 3 | Two-byte NOP (zero page) |
| 14, 34, 54, 74, D4, F4 | 2 | 4 | Two-byte NOP (zero page,X) |
| 0C | 3 | 4 | Three-byte NOP (absolute) |
| 1C, 3C, 5C, 7C, DC, FC | 3 | 4+ | Three-byte NOP (absolute,X) |

### Unstable Unofficial Opcodes

#### KIL/HLT/JAM (Halt CPU)
Opcodes: 02, 12, 22, 32, 42, 52, 62, 72, 92, B2, D2, F2

These halt the CPU. Only a RESET can restart execution.
- **Do not implement** unless targeting specific test ROMs
- Real hardware hangs until powered off

#### Highly Unstable Opcodes
Some unofficial opcodes have unpredictable behavior:
- **ANE** ($8B): A = (A | magic) & X & immediate
- **LXA** ($AB): A,X = (A | magic) & immediate
- **TAS** ($9B): Unstable store operation
- **SHX** ($9E), **SHY** ($9C), **SHA** ($93, $9F): Unstable indexed stores

**Recommendation**: Implement stable opcodes for nestest. Research unstable ones only if specific games require them.

## Opcode Implementation Priority

### Phase 1: Core Operations (nestest basics)
1. Load/Store: LDA, STA, LDX, STX, LDY, STY
2. Transfers: TAX, TXA, TAY, TYA, TSX, TXS
3. Stack: PHA, PLA, PHP, PLP
4. Flags: CLC, SEC, CLI, SEI, CLV, CLD, SED
5. NOP

### Phase 2: Logic & Arithmetic
1. AND, ORA, EOR, BIT
2. ADC, SBC
3. CMP, CPX, CPY

### Phase 3: Increment/Decrement & Shifts
1. INC, DEC, INX, INY, DEX, DEY
2. ASL, LSR, ROL, ROR

### Phase 4: Control Flow
1. Branches: BCC, BCS, BEQ, BNE, BMI, BPL, BVC, BVS
2. Jumps: JMP (absolute and indirect)
3. JSR, RTS
4. BRK, RTI

### Phase 5: Unofficial Opcodes
Implement stable unofficial opcodes in this order:
1. LAX, SAX
2. DCP, ISC
3. SLO, RLA, SRE, RRA
4. Unofficial NOPs
5. Skip unstable opcodes initially

## Testing Each Opcode

For each opcode, verify:
1. **Correct operation**: Result matches expected value
2. **Flag updates**: N, Z, C, V set correctly
3. **Cycle count**: Exact cycle timing
4. **Page crossing**: Additional cycle when applicable
5. **Edge cases**: Wraparound, overflow, etc.

Example test case for ADC:
```rust
#[test]
fn test_adc_immediate() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    // Test: 0x50 + 0x50 = 0xA0 (no carry in, sets N flag)
    cpu.a = 0x50;
    cpu.p.carry = false;
    bus.write(0x0000, 0x69); // ADC #$50
    bus.write(0x0001, 0x50);

    let cycles = cpu.step(&mut bus);

    assert_eq!(cpu.a, 0xA0);
    assert_eq!(cpu.p.negative, true);
    assert_eq!(cpu.p.zero, false);
    assert_eq!(cpu.p.carry, false);
    assert_eq!(cpu.p.overflow, true); // 0x50 + 0x50 overflows in signed
    assert_eq!(cycles, 2);
}
```

## Reference Materials

- Complete opcode matrix: https://www.nesdev.org/wiki/CPU_unofficial_opcodes
- Instruction behavior: https://www.nesdev.org/obelisk-6502-guide/
- Cycle timing: https://www.nesdev.org/wiki/Cycle_counting
- nestest ROM: https://github.com/christopherpow/nes-test-roms
