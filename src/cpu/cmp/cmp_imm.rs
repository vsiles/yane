macro_rules! declare_cmp_imm {
    ($mod:ident, $name:ident, $mnemo:ident, $reg:ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;
            use super::flags::CpuFlags;

            const SIZE: u16 = 2;

            pub struct $name {
                oldf: CpuFlags,
            }

            impl OpCode for $name {
                fn new() -> $name {
                    $name {
                        oldf: CpuFlags::new(),
                    }
                }

                fn decode(&mut self, cpu: &mut Cpu) -> bool {
                    self.oldf = cpu.flags.clone();
                    let imm = cpu.read_from_pc();
                    let (res, _) = cpu.$reg.overflowing_sub(imm);
                    cpu.flags.carry = cpu.$reg >= imm;
                    cpu.flags.zero = cpu.$reg == imm;
                    cpu.flags.negative = (res & 0x80) != 0;
                    true
                }

                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc - SIZE;
                    let code = cpu.mem.get(pc);
                    let imm = cpu.mem.get(pc + 1);
                    print!(
                        "{:04X}  {:02X} {:02X}     {} #${:02X}",
                        pc,
                        code,
                        imm,
                        stringify!($mnemo),
                        imm
                    );
                    let mut old_cpu = cpu.debug_clone();
                    old_cpu.flags = self.oldf.clone();
                    println!("{: <24}{}", "", old_cpu);
                }
            }
        }
    };
}
