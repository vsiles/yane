# YANE Documentation

Welcome to the documentation for YANE (Yet Another NES Emulator)!

## Reading Order

This documentation is designed to be read in the following order:

### 1. [Project Overview](00-project-overview.md)
Start here! This document provides:
- Project goals and scope
- Technical specifications
- High-level architecture
- Success criteria and milestones

### 2. [CPU Implementation Guide](01-cpu-implementation.md)
Deep dive into CPU emulation:
- Register architecture
- Addressing modes (all 13 modes)
- Clock timing and cycle counting
- Interrupt handling (NMI, IRQ, BRK, RTI)
- Implementation strategy by phase

### 3. [CPU Opcodes Reference](02-cpu-opcodes.md)
Complete opcode reference:
- All 151 official opcodes
- 105 unofficial opcodes (required for nestest!)
- Cycle counts and addressing modes
- Implementation priority order
- Testing strategies for each opcode

### 4. [Hardware Quirks and Bugs](03-hardware-quirks.md)
NES-specific hardware behaviors to emulate:
- JMP indirect page boundary bug
- No decimal mode
- Interrupt hijacking
- B flag behavior
- Dummy reads and writes
- Page crossing timing
- Open bus behavior
- Memory mirroring

### 5. [Testing Guide](04-testing-guide.md)
Comprehensive testing approach:
- nestest setup and usage
- Log comparison techniques
- Other test ROMs (instr_test, cpu_timing, etc.)
- Debugging strategies
- Test-driven development examples
- Continuous integration

### 6. [nestest Reference](06-nestest-reference.md)
Detailed nestest ROM documentation:
- File format and specifications
- Complete log format breakdown
- Test modes (automation vs full)
- Step-by-step usage guide
- Log comparison strategies
- Common failure patterns
- Test coverage details

### 7. [Implementation Roadmap](05-implementation-roadmap.md)
Step-by-step implementation guide:
- 11 phases from project setup to minimal PPU
- Detailed code examples for each phase
- Testing checkpoints
- Weekly development goals
- Daily routine suggestions

## Quick Start

1. **Read the overview** (5 minutes)
   - Understand the project scope and goals

2. **Skim the CPU guide** (15 minutes)
   - Get familiar with 6502 architecture
   - Understand addressing modes

3. **Follow the roadmap** (ongoing)
   - Implement phase by phase
   - Test continuously
   - Reference other docs as needed

## Key Concepts

### Cycle Accuracy
Every instruction takes an exact number of CPU cycles. This must be emulated precisely for accurate timing, especially for:
- PPU synchronization (later)
- APU timing (later)
- Passing nestest

### nestest Is Your North Star
The nestest ROM is the primary validation tool. Your goal is simple:
1. Run nestest in automation mode (start at $C000)
2. Compare execution log with reference
3. Debug until logs match 100%
4. Pass indicator: $0002 reads $00

### Hardware Quirks Matter
The NES CPU has several hardware bugs and quirks that real games depend on:
- JMP indirect bug (affects some games)
- B flag distinction (IRQ vs BRK handlers)
- Interrupt hijacking (rare but tested)
- Dummy reads/writes (affects memory-mapped I/O)

These must be emulated correctly. See [Hardware Quirks](03-hardware-quirks.md).

### Unofficial Opcodes Are Required
While undocumented, many unofficial opcodes are stable and used by:
- nestest (requires them to pass)
- Some commercial games
- Homebrew software

Implement stable unofficial opcodes. Skip unstable ones initially. See [Opcodes Reference](02-cpu-opcodes.md#unofficial-opcodes).

## Common Questions

### Do I need to implement decimal mode?
**No.** The NES CPU lacks decimal mode support. The D flag exists but has no effect on ADC/SBC. Always use binary arithmetic.

### What about timing?
**Cycle-accurate timing is critical.** Every instruction must take the exact number of cycles, including:
- Page boundary crossing penalties (+1 cycle)
- Branch timing (2 not taken, 3 taken same page, 4 taken page cross)
- RMW operations (extra cycle for dummy write)

### When should I start on the PPU?
**After CPU passes nestest automation mode.** Get the CPU 100% correct first. A working CPU takes about 3-4 weeks. Then move to minimal PPU for visual output.

### Which test ROMs should I use?
Priority order:
1. **nestest** - Start here, use as primary validation
2. **instr_test_v5** - Individual instruction testing
3. **cpu_timing_test6** - Cycle timing validation
4. **branch_timing_tests** - Branch-specific timing
5. **cpu_interrupts_v2** - Interrupt behavior

### What about mappers?
**Start with Mapper 0 (NROM).** This is the simplest mapper and sufficient for nestest. Implement other mappers later as needed for specific games.

### How do I debug when nestest fails?
See [Testing Guide - Debugging Strategies](04-testing-guide.md#debugging-strategies). Key approach:
1. Find first line where logs diverge
2. The bug is likely in a previous instruction
3. Create minimal reproduction test case
4. Test that specific instruction in isolation
5. Fix and re-run nestest

## Development Workflow

### Recommended Daily Cycle
1. **Choose next feature** from roadmap
2. **Write test** for the feature
3. **Implement** minimally to pass test
4. **Run nestest** to check for regressions
5. **Debug** any failures
6. **Commit** working code
7. **Repeat**

### Testing Philosophy
- Test early, test often
- Write unit tests for each opcode
- Run nestest after every change
- Compare logs methodically
- Fix bugs immediately

### Code Quality
- Prioritize correctness over performance
- Document hardware quirks in comments
- Keep code readable (it's educational!)
- Use types to prevent errors (u8 vs u16)
- Don't over-optimize initially

## Resources

### Primary References
- **NESdev Wiki**: https://www.nesdev.org/ (primary source)
- **6502.org**: http://www.6502.org/ (6502 reference)
- **Visual 6502**: http://www.visual6502.org/ (see the silicon!)

### Test ROMs
- **GitHub**: https://github.com/christopherpow/nes-test-roms
- **nestest log**: Included in test ROM repository

### Community
- **NESDev Forums**: https://forums.nesdev.org/
- **Discord**: Many active NES dev communities
- **Reddit**: r/EmuDev

## Document Updates

These documents are based on information from nesdev.org as of February 2025. Key sources:
- CPU architecture and behavior
- Opcode tables and cycle counts
- Test ROM descriptions
- Known hardware quirks

## Next Steps

1. ✅ Read this README
2. → Read [Project Overview](00-project-overview.md)
3. → Read [Implementation Roadmap](05-implementation-roadmap.md)
4. → Start coding!

Good luck with your NES emulator journey! 🎮

---

*Note: This is an educational project. The focus is on learning and accuracy, not performance. Take your time, understand each concept, and enjoy the process of building a working emulator!*
