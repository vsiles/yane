macro_rules! declare_decr {
    ($mod:ident, $name:ident, $reg:ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;
            use super::flags::CpuFlags;

            const SIZE: u16 = 1;

            pub struct $name {
                saved: u8,
                oldf: CpuFlags,
            }

            impl OpCode for $name {
                fn new() -> $name {
                    $name {
                        saved: 0,
                        oldf: CpuFlags::new(),
                    }
                }

                fn decode(&mut self, cpu: &mut Cpu) -> bool {
                    self.oldf = cpu.flags.clone();
                    self.saved = cpu.$reg;
                    let val = cpu.$reg.overflowing_sub(1);
                    cpu.$reg = val.0;
                    cpu.flags.zero = cpu.$reg == 0;
                    cpu.flags.negative = (cpu.$reg & 0x80) != 0;
                    true
                }

                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc - SIZE;
                    let code = cpu.mem.get(pc);
                    print!(
                        "{:04X}  {:02X}        DE{}",
                        pc,
                        code,
                        stringify!($reg)
                    );
                    let mut old_cpu = cpu.debug_clone();
                    old_cpu.$reg = self.saved;
                    old_cpu.flags = self.oldf.clone();
                    print!("{: <29}{}", "", old_cpu);
                }
            }
        }
    };
}
