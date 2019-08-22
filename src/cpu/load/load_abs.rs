macro_rules! declare_load_abs {
    ($mod:ident, $name:ident, $reg:ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;

            pub struct $name {
                low: u8,
                high: u8,
                imm: u8,
                state: usize,
            }

            impl OpCode for $name {
                fn new() -> $name {
                    $name {
                        low: 0,
                        high: 0,
                        imm: 0,
                        state: 0,
                    }
                }

                fn decode(&mut self, cpu: &mut Cpu) -> bool {
                    if self.state == 0 {
                        self.low = cpu.read_from_pc();
                        self.state = 1;
                        false
                    } else if self.state == 1 {
                        self.high = cpu.read_from_pc();
                        self.state = 2;
                        false
                    } else {
                        let addr: u16 = mk_addr!(self.low, self.high);
                        self.imm = cpu.mem.get(addr);
                        execute_load!($reg, self, cpu);
                        true
                    }
                }

                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc - 1;
                    let code = cpu.mem.get(pc);
                    let low = cpu.mem.get(pc + 1);
                    let high = cpu.mem.get(pc + 2);
                    let addr = mk_addr!(low, high);
                    let val = cpu.mem.get(addr); 
                    print!(
                        "{:04X}  {:02X} {:02X} {:02X}  LD{} ${:04X}",
                        pc,
                        code,
                        low,
                        high,
                        stringify!($reg),
                        addr
                    );
                    print!(" = {:02X} {: >17}{}", val, "", cpu)
                }
            }
        }
    };
}
