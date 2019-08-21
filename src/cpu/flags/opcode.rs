macro_rules! declare_flags_opcode {
    ($mod:ident, $name:ident, $fname:ident, $flag:ident, $val:expr) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;
            use super::flags::CpuFlags;

            const SIZE: u16 = 1;

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
                    let _ = cpu.mem.get(cpu.pc);
                    cpu.flags.$flag = $val;
                    true
                }
                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc - SIZE;
                    let code = cpu.mem.get(pc);
                    print!("{:04X}  {:02X}        {}", pc, code, stringify!($fname));
                    let mut old_cpu = cpu.debug_clone();
                    old_cpu.flags = self.oldf.clone();
                    println!("{: >29}{}", "", old_cpu)
                }
            }
        }
    };
}
