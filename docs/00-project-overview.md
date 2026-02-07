# YANE - Yet Another NES Emulator

## Project Goals

This is an educational NES emulator written in Rust with the following priorities:

1. **Cycle Accuracy**: Emulate the NES CPU cycle-by-cycle for accurate timing
2. **Pass nestest**: The emulator must pass the comprehensive nestest CPU test suite
3. **Clean Architecture**: Maintainable, well-documented Rust code
4. **Educational Focus**: Code clarity over performance optimizations

## Initial Scope

### Phase 1: CPU Implementation (Current Focus)
- Complete 6502 CPU emulation (Ricoh 2A03 variant)
- All official opcodes
- All unofficial/illegal opcodes (required for nestest)
- Cycle-accurate execution
- Proper interrupt handling (NMI, IRQ, RESET)
- Hardware bug emulation

### Phase 2: Minimal PPU
- Basic rendering to pass simple tests
- No advanced features initially
- Just enough to visualize nestest results

### Phase 3: Minimal APU
- Basic audio support
- Low priority for initial development

### Phase 4: Cartridge/Mapper Support
- Start with NROM (mapper 0)
- Expand to common mappers as needed

## Technical Specifications

### Target Hardware: NTSC NES
- CPU: Ricoh 2A03 (6502 without decimal mode)
- Clock Speed: ~1.79 MHz (559 nanoseconds per cycle)
- Master Clock: 21.47727 MHz (divided by 12)

### Memory Map Overview
```
$0000-$07FF: 2KB Internal RAM
$0800-$1FFF: Mirrors of RAM
$2000-$2007: PPU Registers
$2008-$3FFF: Mirrors of PPU registers (every 8 bytes)
$4000-$4017: APU and I/O registers
$4020-$FFFF: Cartridge space
  $6000-$7FFF: Cartridge RAM (typically)
  $8000-$FFFF: Cartridge ROM
```

### Interrupt Vectors
```
$FFFA-$FFFB: NMI handler address
$FFFC-$FFFD: RESET handler address
$FFFE-$FFFF: IRQ/BRK handler address
```

## Development Approach

### Test-Driven Development
- Use nestest as primary validation
- Run test ROM at address $C000
- Compare execution log against Nintendulator reference
- Each instruction implementation must be verified

### Incremental Implementation
1. Basic CPU structure and registers
2. Simple instructions (LDA, STA, transfers)
3. Arithmetic and logic operations
4. Branching and flow control
5. Stack operations
6. Interrupts
7. Unofficial opcodes

### Debugging Support
- Logging capability for instruction execution
- Cycle counting and verification
- Register state dumps
- Memory inspection tools

## Resources

- Primary Reference: https://www.nesdev.org/
- Test ROM: nestest (https://github.com/christopherpow/nes-test-roms)
- Reference Log: Nintendulator output for comparison
- 6502 Instruction Reference: http://www.6502.org/

## Project Structure (Proposed)

```
yane/
├── src/
│   ├── main.rs           # Entry point
│   ├── cpu/
│   │   ├── mod.rs        # CPU module
│   │   ├── registers.rs  # CPU registers
│   │   ├── opcodes.rs    # Opcode implementations
│   │   ├── addressing.rs # Addressing modes
│   │   └── interrupts.rs # Interrupt handling
│   ├── memory/
│   │   ├── mod.rs        # Memory bus
│   │   └── ram.rs        # RAM implementation
│   ├── ppu/              # PPU (minimal initially)
│   ├── apu/              # APU (minimal initially)
│   ├── cartridge/        # ROM loading and mappers
│   └── emulator.rs       # Main emulator loop
├── tests/
│   └── nestest/          # Test ROM and validation
└── docs/
    ├── 00-project-overview.md
    ├── 01-cpu-implementation.md
    ├── 02-cpu-opcodes.md
    ├── 03-hardware-quirks.md
    └── 04-testing-guide.md
```

## Success Criteria

### Milestone 1: Basic CPU ✅ **PHASE 1 COMPLETE**
- [x] All registers implemented
- [x] Status flags with proper trait implementations
- [x] Power-up and reset behavior
- [x] 17 tests passing (10 unit + 7 integration)
- [x] Zero warnings (clippy --all-targets)
- [ ] Memory bus working (Phase 2)
- [ ] Simple instructions execute (Phase 3+)
- [ ] Can load and run a basic ROM (Phase 9+)

### Milestone 2: Complete CPU
- [ ] All official opcodes implemented
- [ ] All unofficial opcodes implemented
- [ ] Cycle-accurate timing
- [ ] Interrupt handling works

### Milestone 3: Pass nestest
- [ ] Execution matches reference log
- [ ] All CPU tests pass
- [ ] Automated test validation

### Milestone 4: Visual Output
- [ ] Minimal PPU renders test results
- [ ] Can see nestest output on screen

## Next Steps

### ✅ Completed
1. ~~Read CPU implementation guide (01-cpu-implementation.md)~~
2. ~~Set up project structure in Rust~~
3. ~~Implement basic CPU skeleton~~ **PHASE 1 COMPLETE**
   - CPU registers and StatusFlags implemented
   - Trait implementations (From<u8>, etc.)
   - Comprehensive tests (17 passing)

### 🔄 Current: Phase 2
4. **Implement Memory Bus** (see 05-implementation-roadmap.md Phase 2)
   - Create `src/bus.rs`
   - 2KB RAM with mirroring
   - ROM space mapping
   - CPU read/write methods

### 📋 Upcoming
5. Study opcode reference (02-cpu-opcodes.md)
6. Review hardware quirks (03-hardware-quirks.md)
7. Begin opcode implementation with tests (Phase 3+)
