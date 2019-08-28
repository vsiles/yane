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

macro_rules! declare_addr_zero_page_reg2 {
    ($name:ident, $mnemo:ident, $action:expr) => {
        pub struct $name {
            addr: u8,
            imm: u8,
            state: usize,
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    addr: 0,
                    state: 0,
                    imm: 0,
                }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                if self.state == 0 {
                    // 2     PC      R  fetch address, increment PC
                    self.addr = cpu.read_from_pc();
                    self.state = 1;
                    false
                } else if self.state == 1 {
                    // 3   address   R  read from address, add index register X to it
                    self.addr = self.addr.overflowing_add(cpu.X).0;
                    self.state = 2;
                    false
                } else if self.state == 2 {
                    // 4  address+X* R  read from effective address
                    self.imm = cpu.mem.get(self.addr as u16);
                    self.state = 3;
                    false
                } else if self.state == 3 {
                    // 5  address+X* W  write the value back to effective address,
                    //                  and do the operation on it
                    cpu.mem.set(self.addr as u16, self.imm);
                    self.imm = $action(cpu, self.imm);
                    self.state = 4;
                    false
                } else {
                    // 6  address+X* W  write the new value to effective address
                    // Note: * The high byte of the effective address is always zero,
                    //        i.e. page boundary crossings are not handled.
                    cpu.mem.set(self.addr as u16, self.imm);
                    true
                }
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc - 1;
                let code = cpu.mem.get(pc);
                let addr = cpu.mem.get(pc + 1);
                let faddr = addr.overflowing_add(cpu.X).0;
                let imm = cpu.mem.get(faddr as u16);
                print!(
                    "{:04X}  {:02X} {:02X}     {} ${:02X},X",
                    pc,
                    code,
                    addr,
                    stringify!($mnemo),
                    addr,
                );
                print!(" @ {:02X} = {:02X}{: >13}{}", faddr, imm, "", cpu)
            }
        }
    };
}
