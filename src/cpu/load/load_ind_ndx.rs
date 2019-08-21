macro_rules! declare_load_ind_ndx {
    ($mod:ident, $name:ident, $reg:ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;

            const SIZE: u16 = 2;

            pub struct $name {
                low: u8,
                high: u8,
                carry: bool,
                addr: u8,
                imm: u8,
                state: usize,
                saved: u8,
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
                        saved: 0,
                    }
                }

                fn decode(&mut self, cpu: &mut Cpu) -> bool {
                    if self.state == 0 {
                        self.saved = cpu.$reg;
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
                        execute_load!($reg, self, cpu);
                        true
                    }
                }

                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc - SIZE;
                    let code = cpu.mem.get(pc);
                    let addr: u16 = mk_addr!(self.low, self.high);
                    print!(
                        "{:04X}  {:02X} {:02X}     LD{} $({:02X}),Y",
                        pc,
                        code,
                        self.addr - 1,
                        stringify!($reg),
                        self.addr - 1
                    );
                    let mut old_cpu = cpu.debug_clone();
                    old_cpu.$reg = self.saved;
                    print!(
                        " = {:04X} @ {:04X} = {:02X} {: >1}{}",
                        addr,
                        addr - (cpu.Y as u16),
                        self.imm,
                        "",
                        old_cpu
                    )
                }
            }
        }
    };
}
