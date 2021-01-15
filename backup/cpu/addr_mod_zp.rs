macro_rules! declare_addr_zero_page_raw {
    ($name:ident, $mnemo:ident, $action:expr, $illegal:expr) => {
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
                } else {
                    $action(cpu, self.addr as usize);
                    true
                }
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc - 1;
                let code = cpu.mem.get(pc);
                let addr = cpu.mem.get(pc + 1);
                let old = cpu.mem.get(addr as u16);
                print!(
                    "{:04X}  {:02X} {:02X}    {}{} ${:02X}",
                    pc,
                    code,
                    addr,
                    if $illegal { "*" } else { " " },
                    stringify!($mnemo),
                    addr
                );
                print!(" = {:02X} {: >19}{}", old, "", cpu)
            }
        }
    };
}

macro_rules! declare_addr_zero_page {
    ($name:ident, $mnemo:ident, $action:expr) => {
        declare_addr_zero_page_raw!($name, $mnemo, $action, false);
    };
    ($name:ident, $mnemo:ident, $action:expr, $illegal:expr) => {
        declare_addr_zero_page_raw!($name, $mnemo, $action, $illegal);
    };
}

macro_rules! declare_addr_zero_page2 {
    ($name:ident, $mnemo:ident, $action:expr) => {
        pub struct $name {
            addr: u8,
            data: u8,
            state: usize,
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    addr: 0,
                    data: 0,
                    state: 0,
                }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                if self.state == 0 {
                    // 2     PC      R  fetch address, increment PC
                    self.addr = cpu.read_from_pc();
                    self.state = 1;
                    false
                } else if self.state == 1 {
                    // 3  address  R  read from effective address
                    self.state = 2;
                    self.data = cpu.mem.get(self.addr as u16);
                    false
                } else if self.state == 2 {
                    // 4  address  W  write the value back to effective address,
                    //                and do the operation on it
                    self.state = 3;
                    cpu.mem.set(self.addr as u16, self.data);
                    self.data = $action(cpu, self.data);
                    false
                } else {
                    // 5  address  W  write the new value to effective address
                    cpu.mem.set(self.addr as u16, self.data);
                    true
                }
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc - 1;
                let code = cpu.mem.get(pc);
                let addr = cpu.mem.get(pc + 1);
                let old = cpu.mem.get(addr as u16);
                print!(
                    "{:04X}  {:02X} {:02X}     {} ${:02X}",
                    pc,
                    code,
                    addr,
                    stringify!{$mnemo},
                    addr
                );
                print!(" = {:02X} {: >19}{}", old, "", cpu)
            }
        }
    };
}
