macro_rules! declare_store_abs_reg {
    ($mod:ident, $name:ident, $reg:ident, $base: ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;

            const SIZE : usize = 3;

            pub struct $name {
                low: u8,
                high: u8,
                carry: bool,
                state: usize,
                old: u8,
            }

            impl OpCode for $name {
                fn new() -> $name {
                    $name {
                        low: 0,
                        high: 0,
                        carry: false,
                        state: 0,
                        old: 0
                    }
                }

                fn decode(&mut self, cpu: &mut Cpu) -> bool {
                    if self.state == 0 {
                        self.low = cpu.read_from_pc();
                        self.state = 1;
                        false
                    } else if self.state == 1 {
                        self.high = cpu.read_from_pc();
                        let (low, carry) = self.low.overflowing_add(cpu.$base);
                        self.low = low;
                        self.carry = carry;
                        self.state = 2;
                        false
                    } else if self.state == 2 {
                        let addr = mk_addr!(self.low, self.high);
                        self.old = cpu.mem[addr as usize];
                        self.state = 3;
                        if self.carry {
                            self.high = self.high + 1;
                        }
                        false
                    } else {
                        let addr : u16 = mk_addr!(self.low, self.high);
                        self.old = cpu.mem[addr as usize];
                        execute_store!($reg, addr, cpu);
                        true
                    }
                }
                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc;
                    let upc : usize = pc as usize;
                    let code = cpu.mem[upc - SIZE];
                    let low = cpu.mem[upc + 1 - SIZE];
                    let high = cpu.mem[upc + 2 - SIZE];
                    let base = mk_addr!(low, high);
                    let addr : u16 = mk_addr!(self.low, self.high);
                    print!("{:04X}  {:02X} {:02X} {:02X}  ST{} ${:04X}", pc, code, 
                           self.low, self.high, stringify!($reg), base);
                    println!(",{} @ {:04X} = {:02X} {: >8}{}", stringify!($base),
                        addr, self.old, "", cpu)
                }
            }
        }
    }
}
