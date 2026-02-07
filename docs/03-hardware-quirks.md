# NES CPU Hardware Quirks and Bugs

## Overview

The NES CPU has several hardware quirks that must be emulated for accurate behavior. These are not bugs in the traditional sense—they're characteristics of the actual silicon implementation that some software depends on.

## Critical Hardware Quirks

### 1. JMP Indirect Page Boundary Bug

**The Bug**: When JMP ($nnnn) crosses a page boundary, the high byte is fetched from the wrong address.

**Normal Behavior**:
```
JMP ($10FF) should:
  1. Read low byte from $10FF
  2. Read high byte from $1100
```

**Actual Behavior**:
```
JMP ($10FF) actually:
  1. Read low byte from $10FF
  2. Read high byte from $1000 (wraps within page!)
```

**Implementation**:
```rust
fn jmp_indirect(&mut self, bus: &mut Bus) {
    let ptr_low = self.fetch_byte(bus);
    let ptr_high = self.fetch_byte(bus);
    let ptr = u16::from_le_bytes([ptr_low, ptr_high]);

    // Read target address with page boundary bug
    let target_low = bus.read(ptr);

    // Bug: if low byte is 0xFF, high byte wraps to page start
    let ptr_high_addr = if ptr & 0xFF == 0xFF {
        ptr & 0xFF00  // Wrap to start of same page
    } else {
        ptr + 1       // Normal increment
    };

    let target_high = bus.read(ptr_high_addr);
    self.pc = u16::from_le_bytes([target_low, target_high]);
}
```

**Impact**: Some games and test ROMs verify this behavior.

**Test Case**:
```
Setup:
  $10FF = $34
  $1000 = $12
  $1100 = $56

JMP ($10FF) will jump to $1234, not $5634
```

---

### 2. No Decimal Mode

**The Quirk**: The Ricoh 2A03/2A07 lacks BCD (Binary Coded Decimal) arithmetic support present in the original MOS 6502.

**Behavior**:
- SED and CLD instructions execute (2 cycles each)
- The D flag can be set and cleared
- **But ADC and SBC always operate in binary mode**

**Implementation**:
```rust
// The D flag exists but has no effect
impl Cpu {
    fn adc(&mut self, value: u8) {
        // NO decimal mode logic needed for NES
        // Always use binary arithmetic

        let a = self.a as u16;
        let val = value as u16;
        let carry = if self.p.carry { 1 } else { 0 };

        let result = a + val + carry;

        // Set carry if result > 255
        self.p.carry = result > 0xFF;

        let result = result as u8;

        // Set overflow: (A^result) & (value^result) & 0x80
        self.p.overflow = ((self.a ^ result) & (value ^ result) & 0x80) != 0;

        self.a = result;
        self.update_nz_flags(result);
    }
}
```

**Impact**: Simplifies ADC/SBC implementation. Just ignore the D flag entirely in arithmetic.

---

### 3. Interrupt Hijacking

**The Bug**: If NMI occurs during the first 4 cycles of BRK or IRQ handling, the wrong vector is fetched.

**Scenario**:
```
1. IRQ or BRK begins execution (7-cycle sequence)
2. NMI edge detected during cycles 1-4
3. Push sequence completes (PC and P pushed to stack)
4. BUT: NMI vector is fetched instead of IRQ vector
5. Stack shows B=0 (hardware interrupt), but came from BRK
```

**Timing Details**:
```
BRK/IRQ Sequence:
  Cycle 1-2: Fetch/dummy read
  Cycle 3:   Push PCH
  Cycle 4:   Push PCL
  Cycle 5:   Push P (B flag determines source)
  Cycle 6:   Fetch vector low byte   <- NMI can hijack here
  Cycle 7:   Fetch vector high byte  <- or here
```

**Implementation**:
```rust
impl Cpu {
    fn check_interrupts(&mut self, bus: &mut Bus) {
        // Check for NMI edge (high-to-low transition)
        let nmi_triggered = /* detect NMI edge */;

        // Check for IRQ level
        let irq_triggered = /* check IRQ pin */ && !self.p.interrupt;

        if nmi_triggered {
            self.handle_interrupt(bus, InterruptType::NMI);
            self.nmi_pending = false;
        } else if irq_triggered {
            self.handle_interrupt(bus, InterruptType::IRQ);
        }
    }

    fn handle_interrupt(&mut self, bus: &mut Bus, int_type: InterruptType) {
        // During vector fetch, re-check for NMI hijacking
        if self.nmi_pending {
            int_type = InterruptType::NMI; // Hijack!
        }

        // ... rest of interrupt handling
    }
}
```

**Impact**: Rare but documented behavior. Some test ROMs check for this.

---

### 4. B Flag Behavior

**The Quirk**: The B "flag" doesn't exist as a CPU register bit—it only appears when P is pushed to the stack.

**Behavior**:
```
When P is pushed (PHP, BRK, IRQ, NMI):
  Bit 5: Always set to 1
  Bit 4 (B): Set based on source:
    - PHP instruction: B = 1
    - BRK instruction: B = 1
    - IRQ hardware:    B = 0
    - NMI hardware:    B = 0
```

**Implementation**:
```rust
fn push_status(&mut self, bus: &mut Bus, b_flag: bool) {
    let mut status = self.p.as_byte();

    // Bit 5 always set when pushed
    status |= 0x20;

    // Bit 4 set based on source
    if b_flag {
        status |= 0x10;  // Software interrupt/PHP
    } else {
        status &= !0x10; // Hardware interrupt
    }

    self.push_byte(bus, status);
}

// Usage:
// PHP:  push_status(bus, true)
// BRK:  push_status(bus, true)
// IRQ:  push_status(bus, false)
// NMI:  push_status(bus, false)
```

**Impact**: Critical for distinguishing IRQ from BRK in interrupt handlers. Many ROMs check this.

---

### 5. Interrupt Polling Timing

**The Quirk**: Interrupts are polled at the end of each instruction, but "it's really the status of the interrupt lines at the end of the second-to-last cycle that matters."

**Behavior**:
```
Instruction execution:
  Cycle 1: Fetch opcode
  Cycle 2: Execute...
  Cycle N-1: Second-to-last cycle <- Interrupt polled here
  Cycle N: Last cycle
  Next: Check interrupt flags, dispatch if needed
```

**Implementation**:
```rust
impl Cpu {
    fn step(&mut self, bus: &mut Bus) -> u8 {
        let opcode = self.fetch_byte(bus);
        let cycles = self.execute(opcode, bus);

        // Poll interrupts on second-to-last cycle
        // For simplicity, poll after instruction completes
        self.poll_interrupts(bus);

        cycles
    }

    fn poll_interrupts(&mut self, bus: &mut Bus) {
        // NMI edge-triggered
        if self.nmi_edge_detected {
            self.nmi_pending = true;
            self.nmi_edge_detected = false;
        }

        // Dispatch highest priority pending interrupt
        if self.nmi_pending {
            self.handle_nmi(bus);
        } else if self.irq_line_low && !self.p.interrupt {
            self.handle_irq(bus);
        }
    }
}
```

**Impact**: Affects exact timing of when interrupts are serviced.

---

### 6. CLI/SEI/PLP Interrupt Delay

**The Quirk**: After CLI, SEI, or PLP changes the I flag, interrupt polling happens BEFORE the change takes effect.

**Behavior**:
```
Example: SEI followed by IRQ

1. Execute SEI instruction
2. Poll interrupts (I flag still 0)  <- IRQ can trigger here
3. Set I flag to 1
4. Next instruction begins
```

This means an IRQ can still trigger immediately after SEI if it was pending.

**Implementation**:
```rust
fn execute_sei(&mut self) {
    // Don't set I flag yet
    self.pending_interrupt_disable = true;
}

fn step(&mut self, bus: &mut Bus) -> u8 {
    let opcode = self.fetch_byte(bus);
    let cycles = self.execute(opcode, bus);

    // Poll interrupts BEFORE applying pending flag changes
    self.poll_interrupts(bus);

    // Now apply pending changes
    if self.pending_interrupt_disable {
        self.p.interrupt = true;
        self.pending_interrupt_disable = false;
    }
    if self.pending_interrupt_enable {
        self.p.interrupt = false;
        self.pending_interrupt_enable = false;
    }

    cycles
}
```

**Impact**: Affects precise interrupt timing in edge cases.

---

### 7. Dummy Reads and Writes

**The Quirk**: "Every cycle on 6502 is either a read or a write cycle." This causes observable dummy memory accesses.

**Read-Modify-Write Instructions**:
```
INC $80 performs:
  Cycle 1: Fetch opcode
  Cycle 2: Fetch address
  Cycle 3: Read from $80
  Cycle 4: Write original value back to $80  <- Dummy write!
  Cycle 5: Write incremented value to $80
```

**Impact**:
- Memory-mapped I/O can be triggered by dummy reads
- Dummy writes can affect hardware registers
- Some mapper hardware detects these accesses

**Implementation**:
```rust
fn inc_memory(&mut self, bus: &mut Bus, addr: u16) {
    let value = bus.read(addr);                    // Real read
    bus.write(addr, value);                        // Dummy write
    let result = value.wrapping_add(1);
    bus.write(addr, result);                       // Real write
    self.update_nz_flags(result);
}
```

---

### 8. Page Crossing Behavior

**The Quirk**: Indexed addressing modes may perform a dummy read from an incorrect address when crossing page boundaries.

**Example: LDA $10FF,X where X = $01**
```
Cycle 1: Fetch opcode
Cycle 2: Fetch $FF (address low byte)
Cycle 3: Fetch $10 (address high byte)
Cycle 4: Read from $10FF + X without carry = $1000  <- Dummy read!
Cycle 5: Read from $1100 (correct address)         <- Real read
```

**Implementation**:
```rust
fn absolute_indexed(&mut self, bus: &mut Bus, index: u8) -> (u16, bool) {
    let base_low = self.fetch_byte(bus);
    let base_high = self.fetch_byte(bus);
    let base = u16::from_le_bytes([base_low, base_high]);

    let addr = base.wrapping_add(index as u16);

    // Check if page boundary crossed
    let page_crossed = (base & 0xFF00) != (addr & 0xFF00);

    if page_crossed {
        // Dummy read from incorrect address
        let dummy_addr = (base & 0xFF00) | ((base + index as u16) & 0xFF);
        let _ = bus.read(dummy_addr);
    }

    (addr, page_crossed)
}
```

**Impact**:
- Read instructions: +1 cycle if page crossed
- Write/RMW instructions: Always +1 cycle, dummy read occurs
- Memory-mapped hardware sees the dummy read

---

### 9. Stack Pointer Behavior

**The Quirk**: Stack pointer doesn't wrap between pages—it wraps within page $01.

**Behavior**:
```
Stack at $0100-$01FF
SP wraps: $00 -> $FF, $FF -> $00
Always in page $01: $0100 + SP
```

**Implementation**:
```rust
impl Cpu {
    fn push_byte(&mut self, bus: &mut Bus, value: u8) {
        let addr = 0x0100 | (self.sp as u16);
        bus.write(addr, value);
        self.sp = self.sp.wrapping_sub(1); // Wraps $00 -> $FF
    }

    fn pop_byte(&mut self, bus: &mut Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1); // Wraps $FF -> $00
        let addr = 0x0100 | (self.sp as u16);
        bus.read(addr)
    }
}
```

**Impact**: Stack operations always confined to $0100-$01FF.

---

### 10. Open Bus Behavior

**The Quirk**: Reading from unmapped memory returns the last value on the data bus.

**Behavior**:
- Read from unmapped address
- Returns whatever was last on the data bus (usually last byte fetched)
- Cartridge hardware can affect this

**Implementation**:
```rust
struct Bus {
    last_read: u8, // Track last value on bus

    fn read(&mut self, addr: u16) -> u8 {
        let value = match addr {
            0x0000..=0x1FFF => self.ram[addr as usize & 0x07FF],
            0x2000..=0x3FFF => self.read_ppu_register(addr),
            // ...
            _ => self.last_read, // Open bus: return last value
        };
        self.last_read = value;
        value
    }
}
```

**Impact**: Some games rely on open bus behavior for random number generation or detection.

---

## Power-Up and Reset Quirks

### Power-Up State

```
Register Values:
  A:  $00
  X:  $00
  Y:  $00
  SP: $FD
  P:  $34 (I=1, others=0)
  PC: Load from $FFFC-$FFFD

Memory:
  RAM: Unpredictable (random)
  Best practice: Don't rely on RAM contents
```

### RESET Behavior

```
After RESET (not power-up):
  A, X, Y: Unchanged
  P: I flag set to 1 (others unchanged)
  SP: Decremented by 3
  PC: Load from $FFFC-$FFFD

Memory: Unchanged
```

**Implementation**:
```rust
impl Cpu {
    fn power_on(&mut self, bus: &Bus) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.p = StatusFlags::from_byte(0x34);
        self.pc = self.read_word(bus, 0xFFFC);
        self.cycles = 0;
    }

    fn reset(&mut self, bus: &Bus) {
        // A, X, Y unchanged
        self.p.interrupt = true;
        self.sp = self.sp.wrapping_sub(3);
        self.pc = self.read_word(bus, 0xFFFC);
        self.cycles += 7; // RESET takes 7 cycles
    }
}
```

---

## Memory Mirroring

### RAM Mirroring
```
$0000-$07FF: 2KB internal RAM
$0800-$0FFF: Mirror of $0000-$07FF
$1000-$17FF: Mirror of $0000-$07FF
$1800-$1FFF: Mirror of $0000-$07FF
```

**Implementation**:
```rust
fn read_ram(&self, addr: u16) -> u8 {
    self.ram[(addr & 0x07FF) as usize] // Mask to 2KB
}
```

### PPU Register Mirroring
```
$2000-$2007: 8 PPU registers
$2008-$3FFF: Mirrors (every 8 bytes)
```

**Implementation**:
```rust
fn read_ppu_register(&mut self, addr: u16) -> u8 {
    let reg = addr & 0x2007; // Mirror every 8 bytes
    // ... PPU register access
}
```

---

## Testing Hardware Quirks

### Test ROMs for Quirks

1. **nestest**: Tests JMP indirect bug, B flag, timing
2. **cpu_interrupts_v2**: Tests interrupt hijacking, timing
3. **cpu_dummy_reads**: Validates dummy read behavior
4. **branch_timing_tests**: Page crossing penalties

### Verification Checklist

- [ ] JMP ($xxFF) wraps within page
- [ ] D flag has no effect on ADC/SBC
- [ ] B flag correct in PHP/BRK/IRQ/NMI
- [ ] Interrupt polling timing accurate
- [ ] CLI/SEI/PLP delay implemented
- [ ] Dummy reads/writes occur correctly
- [ ] Page crossing adds correct cycles
- [ ] Stack wraps within $0100-$01FF
- [ ] Power-up state correct
- [ ] RESET decrements SP by 3
- [ ] RAM mirroring works
- [ ] PPU register mirroring works

---

## References

- NESdev Wiki CPU Quirks: https://www.nesdev.org/wiki/CPU
- Visual 6502: http://www.visual6502.org/
- 6502 Timing: http://www.6502.org/tutorials/6502opcodes.html
- nestest ROM: https://github.com/christopherpow/nes-test-roms
