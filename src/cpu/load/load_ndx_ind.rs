macro_rules! declare_load_ndx_ind {
    ($mod:ident, $name:ident, $reg:ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;

            const SIZE: u16 = 2;

            pub struct $name {
                low: u8,
                high: u8,
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
                        self.addr = self.addr.overflowing_add(cpu.X).0;
                        self.state = 2;
                        false
                    } else if self.state == 2 {
                        self.low = cpu.mem.get(self.addr as u16);
                        self.addr = self.addr + 1;
                        self.state = 3;
                        false
                    } else if self.state == 3 {
                        self.high = cpu.mem.get(self.addr as u16);
                        self.state = 4;
                        false
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
                    let payload = cpu.mem.get(pc + 1 - SIZE);
                    let addr: u16 = mk_addr!(self.low, self.high);
                    print!(
                        "{:04X}  {:02X} {:02X}     LD{} (${:02X},X)",
                        pc,
                        code,
                        payload,
                        stringify!($reg),
                        payload
                    );
                    let mut old_cpu = cpu.debug_clone();
                    old_cpu.$reg = self.saved;
                    println!(
                        " @ {:02X} = {:04X} = {:02X} {: >3}{}",
                        self.addr - 1,
                        addr,
                        self.imm,
                        "",
                        old_cpu
                    )
                }
            }
        }
    };
}
