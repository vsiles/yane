macro_rules! declare_addr_ind_x {
    ($name:ident, $mnemo:ident, $action:ident) => {

        pub struct $name {
            low: u8,
            high: u8,
            addr: u8,
            state: usize,
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    low: 0,
                    high: 0,
                    addr: 0,
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
                    self.addr = self.addr.overflowing_add(cpu.X).0;
                    self.state = 2;
                    false
                } else if self.state == 2 {
                    self.low = cpu.mem.get(self.addr as u16);
                    self.addr = self.addr.overflowing_add(1).0;
                    self.state = 3;
                    false
                } else if self.state == 3 {
                    self.high = cpu.mem.get(self.addr as u16);
                    self.state = 4;
                    false
                } else {
                    let addr: u16 = mk_addr!(self.low, self.high);
                    $action(cpu, addr as usize);
                    true
                }
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc - 1;
                let code = cpu.mem.get(pc);
                let payload = cpu.mem.get(pc + 1);
                let addr = payload.overflowing_add(cpu.X).0;
                let low = cpu.mem.get(addr as u16);
                let high = cpu.mem.get(addr.overflowing_add(1).0 as u16);
                let faddr = mk_addr!(low, high);
                let imm = cpu.mem.get(faddr);

                print!(
                    "{:04X}  {:02X} {:02X}     {} (${:02X},X)",
                    pc,
                    code,
                    payload,
                    stringify!($mnemo),
                    payload
                );
                print!(" @ {:02X} = {:04X} = {:02X} {: >3}{}", addr, faddr, imm, "", cpu)
            }
        }
    };
}
