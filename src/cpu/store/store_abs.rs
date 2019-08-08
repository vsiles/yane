macro_rules! declare_store_abs {
    ($mod:ident, $name:ident, $reg:ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;

            const SIZE : usize = 3;

            pub struct $name {
                low: u8,
                high: u8,
                state: usize,
            }

            impl OpCode for $name {
                fn new() -> $name {
                    $name {
                        low: 0,
                        high: 0,
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
                        let addr : u16 = mk_addr!(self.low, self.high);
                        execute_store!($reg, addr, cpu);
                        true
                    }
                }

                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc;
                    let upc : usize = pc as usize;
                    let code = cpu.mem[upc - SIZE];
                    let addr : u16 = mk_addr!(self.low, self.high);
                    print!("{:04X}  {:02X} {:02X} {:02X}  ST{} ${:04X}", pc, code, 
                           self.low, self.high, stringify!($reg), addr);
                    println!(" = {:02X} {: >17}{}", cpu.$reg, "", cpu)
                } 
            }
        }
    }
}
