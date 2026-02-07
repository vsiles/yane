# Claude Code Preferences for YANE

## Project Configuration

### Rust Settings
- **Edition**: 2024 (prefer latest edition)
- **Target**: Cycle-accurate NES emulator
- **Style**: Educational code - clarity over performance optimizations

### Code Style Preferences
- Public fields for CPU registers (easier testing/debugging)
- **Use trait implementations over custom methods**: `impl From<T>` instead of `from_xxx()` functions
- **Boolean assertions**: Use `assert!(value)` not `assert_eq!(value, true)`
- Comprehensive inline documentation
- Both unit tests (inline) and integration tests (tests/)
- Detailed comments explaining NES hardware quirks

### Project Structure
```
yane/
├── src/
│   ├── lib.rs              # Library root
│   ├── main.rs             # Manual testing/demo
│   └── cpu/
│       └── mod.rs          # CPU implementation
├── tests/
│   └── cpu_tests.rs        # Integration tests
├── nestest/
│   ├── nestest.nes         # Test ROM
│   └── nestest.log         # Reference log
└── docs/                   # Comprehensive documentation
```

## Implementation Notes

### Phase 1 Complete ✓
- CPU registers (a, x, y, sp, pc)
- StatusFlags (without B flag - correct per NES spec)
- Power-up and reset behavior
- Flag update helpers
- 17 tests all passing

### Key Design Decisions

#### StatusFlags
- **No B flag field** - B flag only exists when pushed to stack (NES hardware quirk)
- **Bits 4/5 ignored** in `From<u8>` - they're phantom bits
- **Trait implementations**: `impl From<u8>` and `impl From<StatusFlags> for u8`
- Public fields for easy testing
- Kept `as_byte()` for explicit conversion readability

#### CPU
- **Public fields** - chosen for easy testing and debugging during development
- **cycles as u64** - won't overflow during normal emulation
- **reset(pc_start)** - allows setting PC for nestest automation mode ($C000)

## Testing Philosophy
- Test each feature immediately after implementation
- Compare against nestest reference log
- Both unit tests (internal) and integration tests (external API)
- **No warnings or errors allowed**
- Always use `cargo clippy --all-targets -- -D warnings` to check all targets
- Use `assert!(bool)` and `assert!(!bool)` instead of `assert_eq!(bool, true/false)`

## Documentation
Complete documentation in `docs/`:
- 00-project-overview.md - Goals and architecture
- 01-cpu-implementation.md - CPU details
- 02-cpu-opcodes.md - All 256 opcodes
- 03-hardware-quirks.md - NES-specific bugs to emulate
- 04-testing-guide.md - nestest usage
- 05-implementation-roadmap.md - Step-by-step plan
- 06-nestest-reference.md - Detailed nestest info
- QUICK-START.md - Beginner guide

## Next Phase: Memory Bus
- Create `src/bus.rs`
- 2KB RAM with mirroring
- ROM loading (simple for now)
- CPU read/write methods

## Remember
- Always use edition 2024
- Zero warnings/errors policy
- Test immediately after implementation
- Educational code style (clarity first)
- Document all NES hardware quirks
