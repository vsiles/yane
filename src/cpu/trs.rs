
macro_rules! declare_transfert {
    ($mod:ident, $name:ident, $from:ident, $to:ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;

            pub struct $name {
                imm: u8,
            }

            impl OpCode for $name {
                fn new() -> $name {
                    $name {
                        imm: 0,
                    }
                }

                fn decode(&mut self, cpu: &mut Cpu) -> bool {
                    self.imm = cpu.$from;
                    execute_load!($to, self, cpu);
                    true
                }

                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc - 1;
                    let code = cpu.mem.get(pc);
                    print!(
                        "{:04X}  {:02X}        T{}{}",
                        pc,
                        code,
                        stringify!($from),
                        stringify!($to)
                    );
                    print!("{: <29}{}", "", cpu)
                }
            }
        }
    };
}
