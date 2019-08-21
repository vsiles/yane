macro_rules! declare_load_imm {
    ($mod:ident, $name:ident, $reg:ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;
            use super::flags::CpuFlags;

            const SIZE: u16 = 2;

            pub struct $name {
                imm: u8,
                saved: u8,
                oldf: CpuFlags,
            }

            impl OpCode for $name {
                fn new() -> $name {
                    $name {
                        imm: 0,
                        saved: 0,
                        oldf: CpuFlags::new(),
                    }
                }

                fn decode(&mut self, cpu: &mut Cpu) -> bool {
                    self.oldf = cpu.flags.clone();
                    self.saved = cpu.$reg;
                    self.imm = cpu.read_from_pc();
                    execute_load!($reg, self, cpu);
                    true
                }

                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc - SIZE;
                    let code = cpu.mem.get(pc);
                    let imm = self.imm;
                    print!(
                        "{:04X}  {:02X} {:02X}     LD{} #${:02X}",
                        pc,
                        code,
                        imm,
                        stringify!($reg),
                        imm
                    );
                    let mut old_cpu = cpu.debug_clone();
                    old_cpu.$reg = self.saved;
                    old_cpu.flags = self.oldf.clone();
                    print!("{: <24}{}", "", old_cpu);
                }
            }
        }
    };
}
