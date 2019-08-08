macro_rules! declare_load_imm {
    ($mod:ident, $name:ident, $reg:ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;

            const SIZE : usize = 2;

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
                    self.imm = cpu.read_from_pc();
                    execute_load!($reg, self, cpu);
                    true
                }

                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc;
                    let upc : usize = pc as usize;
                    let code = cpu.mem[upc - SIZE];
                    let imm = self.imm;
                    print!("{:04X}  {:02X} {:02X}     LDA #${:02X}", pc, code, imm, imm);
                    println!("{: <24}{}", "", cpu)
                } 
            }
        }
    }
}
