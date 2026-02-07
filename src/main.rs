use yane::cpu::Cpu;

fn main() {
    println!("YANE - Yet Another NES Emulator");
    println!("================================\n");

    // Create new CPU with power-up state
    let cpu = Cpu::new();

    println!("CPU initialized (power-up state):");
    println!("  A:  {:02X}", cpu.a);
    println!("  X:  {:02X}", cpu.x);
    println!("  Y:  {:02X}", cpu.y);
    println!("  SP: {:02X}", cpu.sp);
    println!("  P:  {:02X} (I={})",
        cpu.p.as_byte(),
        if cpu.p.interrupt { "1" } else { "0" }
    );
    println!("  PC: {:04X}", cpu.pc);
    println!("  Cycles: {}\n", cpu.cycles);

    // Test reset (like nestest automation mode)
    let mut cpu2 = Cpu::new();
    cpu2.reset(0xC000);

    println!("CPU after reset to $C000:");
    println!("  PC: {:04X}", cpu2.pc);
    println!("  SP: {:02X}", cpu2.sp);
    println!("  P:  {:02X}", cpu2.p.as_byte());
    println!();

    println!("Phase 1 Complete: CPU registers and status flags implemented!");
    println!("Next step: Run 'cargo test' to verify implementation");
}
