macro_rules! declare_addr_abs_raw {
    ($name:ident, $mnemo:ident, $action:expr, $illegal:expr) => {
        pub struct $name {
            low: u8,
            high: u8,
            state: usize,
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    low: 0,
                    high: 0,
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
                    self.state = 2;
                    false
                } else {
                    let addr = mk_addr!(self.low, self.high);
                    $action(cpu, addr as usize);
                    true
                }
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc - 1;
                let code = cpu.mem.get(pc);
                let low = cpu.mem.get(pc + 1);
                let high = cpu.mem.get(pc + 2);
                let addr = mk_addr!(low, high);
                let val = cpu.mem.get(addr);
                print!(
                    "{:04X}  {:02X} {:02X} {:02X} {}{} ${:04X}",
                    pc,
                    code,
                    low,
                    high,
                    if $illegal { "*" } else { " " },
                    stringify!($mnemo),
                    addr
                );
                print!(" = {:02X} {: >17}{}", val, "", cpu)
            }
        }
    };
}

macro_rules! declare_addr_abs {
    ($name:ident, $mnemo:ident, $action:expr) => {
        declare_addr_abs_raw!($name, $mnemo, $action, false);
    };
    ($name:ident, $mnemo:ident, $action:expr, $illegal:expr) => {
        declare_addr_abs_raw!($name, $mnemo, $action, $illegal);
    };
}

macro_rules! declare_addr_abs2 {
    ($name:ident, $mnemo:ident, $action:expr) => {
        pub struct $name {
            low: u8,
            high: u8,
            addr: u16,
            data: u8,
            state: usize,
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    low: 0,
                    high: 0,
                    addr: 0,
                    data: 0,
                    state: 0,
                }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                if self.state == 0 {
                    // 2    PC     R  fetch low byte of address, increment PC
                    self.low = cpu.read_from_pc();
                    self.state = 1;
                    false
                } else if self.state == 1 {
                    // 3    PC     R  fetch high byte of address, increment PC
                    self.high = cpu.read_from_pc();
                    self.state = 2;
                    false
                } else if self.state == 2 {
                    // 4  address  R  read from effective address
                    self.addr = mk_addr!(self.low, self.high);
                    self.data = cpu.mem.get(self.addr);
                    self.state = 3;
                    false
                } else if self.state == 3 {
                    // 5  address  W  write the value back to effective address,
                    //                and do the operation on it
                    self.state = 4;
                    cpu.mem.set(self.addr, self.data);
                    self.data = $action(cpu, self.data);
                    false
                } else {
                    // 6  address  W  write the new value to effective address
                    cpu.mem.set(self.addr as u16, self.data);
                    true
                }
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc - 1;
                let code = cpu.mem.get(pc);
                let low = cpu.mem.get(pc + 1);
                let high = cpu.mem.get(pc + 2);
                let addr = mk_addr!(low, high);
                let val = cpu.mem.get(addr);
                print!(
                    "{:04X}  {:02X} {:02X} {:02X}  {} ${:04X}",
                    pc,
                    code,
                    low,
                    high,
                    stringify!($mnemo),
                    addr
                );
                print!(" = {:02X} {: >17}{}", val, "", cpu)
            }
        }
    };
}
