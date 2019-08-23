macro_rules! declare_load_ind_ndx {
    ($mod:ident, $name:ident, $reg:ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;

            pub struct $name {
                low: u8,
                high: u8,
                carry: bool,
                addr: u8,
                imm: u8,
                state: usize,
            }

            impl OpCode for $name {
                fn new() -> $name {
                    $name {
                        low: 0,
                        high: 0,
                        addr: 0,
                        carry: false,
                        imm: 0,
                        state: 0,
                    }
                }

                fn decode(&mut self, cpu: &mut Cpu) -> bool {
                    if self.state == 0 {
                        // read offset from memory
                        self.addr = cpu.read_from_pc();
                        self.state = 1;
                        false
                    } else if self.state == 1 {
                        self.low = cpu.mem.get(self.addr as u16);
                        self.addr = self.addr + 1;
                        self.state = 2;
                        false
                    } else if self.state == 2 {
                        self.high = cpu.mem.get(self.addr as u16);
                        let (low, carry) = self.low.overflowing_add(cpu.Y);
                        self.low = low;
                        self.carry = carry;
                        self.state = 3;
                        false
                    } else if self.state == 3 {
                        let addr: u16 = mk_addr!(self.low, self.high);
                        self.imm = cpu.mem.get(addr);
                        self.state = 4;
                        if self.carry {
                            self.high = self.high + 1;
                            false
                        } else {
                            true
                        }
                    } else {
                        let addr: u16 = mk_addr!(self.low, self.high);
                        self.imm = cpu.mem.get(addr);
                        execute_load!($reg, self.imm, cpu);
                        true
                    }
                }

                fn log(&self, _cpu: &Cpu) {
                    print!("TODO LOAD IND NDX")
                    // let pc = cpu.pc - 1;
                    // let code = cpu.mem.get(pc);
                    // let low = cpu.mem.get(pc + 1);
                    // let high = cpu.mem.get(pc + 2);
                    // let addr: u16 = mk_addr!(low, high);
                    // print!(
                    //     "{:04X}  {:02X} {:02X}     LD{} $({:02X}),Y",
                    //     pc,
                    //     code,
                    //     self.addr - 1,
                    //     stringify!($reg),
                    //     self.addr - 1
                    // );
                    // print!(
                    //     " = {:04X} @ {:04X} = {:02X} {: >1}{}",
                    //     addr,
                    //     addr - (cpu.Y as u16),
                    //     self.imm,
                    //     "",
                    //     cpu
                    // )
                }
            }
        }
    };
}
