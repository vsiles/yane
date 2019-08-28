macro_rules! declare_addr_abs_reg {
    ($name:ident, $mnemo:ident, $reg:ident, $action:expr) => {
        pub struct $name {
            low: u8,
            high: u8,
            carry: bool,
            state: usize,
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    low: 0,
                    high: 0,
                    carry: false,
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
                    let (low, carry) = self.low.overflowing_add(cpu.$reg);
                    self.low = low;
                    self.carry = carry;
                    self.state = 2;
                    false
                } else if self.state == 2 {
                    let addr: u16 = mk_addr!(self.low, self.high);
                    if self.carry {
                        self.high = self.high.overflowing_add(1).0;
                        self.state = 3;
                        false
                    } else {
                        $action(cpu, addr as usize);
                        true
                    }
                } else {
                    let addr: u16 = mk_addr!(self.low, self.high);
                    $action(cpu, addr as usize);
                    true
                }
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc - 1;
                let code = cpu.mem.get(pc);
                let low = cpu.mem.get(pc + 1);
                let high = cpu.mem.get(pc + 2);
                let base = mk_addr!(low, high);
                let addr = base.overflowing_add(cpu.$reg as u16).0;
                let imm = cpu.mem.get(addr);
                print!(
                    "{:04X}  {:02X} {:02X} {:02X}  {} ${:04X},{}",
                    pc,
                    code,
                    low,
                    high,
                    stringify!($mnemo),
                    base,
                    stringify!($reg)
                );
                print!(" @ {:04X} = {:02X} {: >8}{}", addr, imm, "", cpu)
            }
        }
    };
}

macro_rules! declare_addr_abs_reg2 {
    ($name:ident, $mnemo:ident, $action:expr) => {
        pub struct $name {
            low: u8,
            high: u8,
            carry: bool,
            state: usize,
            imm: u8,
            addr: u16,
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    low: 0,
                    high: 0,
                    carry: false,
                    state: 0,
                    imm: 0,
                    addr: 0,
                }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                if self.state == 0 {
		    // 2    PC       R  fetch low byte of address, increment PC
                    self.low = cpu.read_from_pc();
                    self.state = 1;
                    false
                } else if self.state == 1 {
		    // 3    PC       R  fetch high byte of address,
		    //                  add index register X to low address byte,
		    //                  increment PC
                    self.high = cpu.read_from_pc();
                    let (low, carry) = self.low.overflowing_add(cpu.X);
                    self.low = low;
                    self.carry = carry;
                    self.state = 2;
                    false
                } else if self.state == 2 {
		    // 4  address+X* R  read from effective address,
		    //                  fix the high byte of effective address
                    self.addr = mk_addr!(self.low, self.high);
                    self.imm = cpu.mem.get(self.addr);
                    if self.carry {
                        self.high = self.high.overflowing_add(1).0;
                    }
                    self.state = 3;
                    false
                } else if self.state == 3 {
                    // 5  address+X  R  re-read from effective address
                    self.addr = mk_addr!(self.low, self.high);
                    self.imm = cpu.mem.get(self.addr);
                    self.state = 4;
                    false
                } else if self.state == 4 {
                    // 6  address+X  W  write the value back to effective address,
                    //                  and do the operation on it
                    cpu.mem.set(self.addr, self.imm);
                    self.imm = $action(cpu, self.imm);
                    self.state = 5;
                    false
                } else {
                    // 7  address+X  W  write the new value to effective address
                    // Notes: * The high byte of the effective address may be invalid
                    //         at this time, i.e. it may be smaller by $100.
                    cpu.mem.set(self.addr, self.imm);
                    true
                }
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc - 1;
                let code = cpu.mem.get(pc);
                let low = cpu.mem.get(pc + 1);
                let high = cpu.mem.get(pc + 2);
                let base = mk_addr!(low, high);
                let addr = base.overflowing_add(cpu.X as u16).0;
                let imm = cpu.mem.get(addr);
                print!(
                    "{:04X}  {:02X} {:02X} {:02X}  {} ${:04X},X",
                    pc,
                    code,
                    low,
                    high,
                    stringify!($mnemo),
                    base,
                );
                print!(" @ {:04X} = {:02X} {: >8}{}", addr, imm, "", cpu)
            }
        }
    };
}
