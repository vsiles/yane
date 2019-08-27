macro_rules! declare_addr_zero_page_reg {
    ($name:ident, $mnemo:ident, $reg:ident, $action:expr) => {
        pub struct $name {
            addr: u8,
            state: usize,
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name { addr: 0, state: 0 }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                if self.state == 0 {
                    // read offset from memory
                    self.addr = cpu.read_from_pc();
                    self.state = 1;
                    false
                } else if self.state == 1 {
                    // compute final offset. Wrapping on page 0
                    self.addr = self.addr.overflowing_add(cpu.$reg).0;
                    self.state = 2;
                    false
                } else {
                    // read data from memory using offset
                    $action(cpu, self.addr as usize);
                    true
                }
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc - 1;
                let code = cpu.mem.get(pc);
                let addr = cpu.mem.get(pc + 1);
                let faddr = addr.overflowing_add(cpu.$reg).0;
                let imm = cpu.mem.get(faddr as u16);
                print!(
                    "{:04X}  {:02X} {:02X}     {} ${:02X},{}",
                    pc,
                    code,
                    addr,
                    stringify!($mnemo),
                    addr,
                    stringify!($reg)
                );
                print!(" @ {:02X} = {:02X}{: >13}{}", faddr, imm, "", cpu);
            }
        }
    };
}
