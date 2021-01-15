macro_rules! declare_addr_ind_y {
    ($name:ident, $mnemo:ident, $action:ident, $store:expr) => {
        pub struct $name {
            low: u8,
            high: u8,
            carry: bool,
            addr: u8,
            state: usize,
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    low: 0,
                    high: 0,
                    addr: 0,
                    carry: false,
                    state: 0,
                }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                if self.state == 0 {
                    // 2      PC       R  fetch pointer address, increment PC
                    self.addr = cpu.read_from_pc();
                    self.state = 1;
                    false
                } else if self.state == 1 {
                    // 3    pointer    R  fetch effective address low
                    self.low = cpu.mem.get(self.addr as u16);
                    self.addr = self.addr.overflowing_add(1).0;
                    self.state = 2;
                    false
                } else if self.state == 2 {
                    // 4   pointer+1   R  fetch effective address high,
                    //                    add Y to low byte of effective address
                    self.high = cpu.mem.get(self.addr as u16);
                    let (low, carry) = self.low.overflowing_add(cpu.Y);
                    self.low = low;
                    self.carry = carry;
                    self.state = 3;
                    false
                } else if self.state == 3 {
                    // 5   address+Y*  R  read from effective address,
                    //                    fix high byte of effective address
                    let addr: u16 = mk_addr!(self.low, self.high);
                    self.state = 4;
                    if self.carry {
                        self.high = self.high.overflowing_add(1).0;
                        false
                    } else {
                        if $store {
                            false // ST* are always 6 cycles long
                        } else {
                            $action(cpu, addr as usize);
                            true
                        }
                    }
                } else {
                    // 6+  address+Y   R  read from effective address
                    let addr: u16 = mk_addr!(self.low, self.high);
                    $action(cpu, addr as usize);
                    true
                }
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc - 1;
                let code = cpu.mem.get(pc);
                let zp = cpu.mem.get(pc + 1);
                let low = cpu.mem.get(zp as u16);
                let zp2 = zp.overflowing_add(1).0;
                let high = cpu.mem.get(zp2 as u16);
                let base = mk_addr!(low, high);
                let uaddr = (base as usize) + (cpu.Y as usize);
                let addr = (uaddr & 0xFFFF) as u16;
                let imm = cpu.mem.get(addr);
                print!(
                    "{:04X}  {:02X} {:02X}     {} (${:02X}),Y",
                    pc,
                    code,
                    zp,
                    stringify!($mnemo),
                    zp
                );
                print!(
                    " = {:04X} @ {:04X} = {:02X} {: >1}{}",
                    base,
                    addr,
                    imm,
                    "",
                    cpu
                )
            }
        }
    };
}
