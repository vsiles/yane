macro_rules! declare_load_abs_reg {
    ($mod:ident, $name:ident, $reg:ident, $base: ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;

            const SIZE: u16 = 3;

            pub struct $name {
                low: u8,
                high: u8,
                carry: bool,
                imm: u8,
                state: usize,
                saved: u8,
            }

            impl OpCode for $name {
                fn new() -> $name {
                    $name {
                        low: 0,
                        high: 0,
                        carry: false,
                        imm: 0,
                        state: 0,
                        saved: 0,
                    }
                }

                fn decode(&mut self, cpu: &mut Cpu) -> bool {
                    if self.state == 0 {
                        self.saved = cpu.$reg;
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
                        let addr: u16 = mk_addr!(self.low, self.high);
                        self.imm = cpu.mem.get(addr);
                        if self.carry {
                            self.high = self.high + 1;
                            self.state = 3;
                            false
                        } else {
                            execute_load!($reg, self, cpu);
                            true
                        }
                    } else {
                        let addr: u16 = mk_addr!(self.low, self.high);
                        self.imm = cpu.mem.get(addr);
                        execute_load!($reg, self, cpu);
                        true
                    }
                }

                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc - SIZE;
                    let code = cpu.mem.get(pc);
                    let low = cpu.mem.get(pc + 1 - SIZE);
                    let high = cpu.mem.get(pc + 2 - SIZE);
                    let base = mk_addr!(low, high);
                    let addr = mk_addr!(self.low, self.high);
                    print!(
                        "{:04X}  {:02X} {:02X} {:02X}  LD{} ${:04X},{}",
                        pc,
                        code,
                        self.low,
                        self.high,
                        stringify!($reg),
                        base,
                        stringify!($base)
                    );
                    let mut old_cpu = cpu.debug_clone();
                    old_cpu.$reg = self.saved;
                    print!(" @ {:04X} = {:02X} {: >8}{}", addr, self.imm, "", old_cpu)
                }
            }
        }
    };
}
