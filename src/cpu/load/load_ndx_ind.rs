macro_rules! declare_load_ndx_ind {
    ($name:ident, $reg:ident) => {
        pub struct $name {
            low: u8,
            high: u8,
            addr: u8,
            imm: u8,
            state: usize,
            size: usize,
        }

        impl OpCode for $name {
            fn new(size: usize) -> $name {
                $name {
                    low: 0,
                    high: 0,
                    addr: 0,
                    imm: 0,
                    state: 0,
                    size: size,
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
                    self.low = cpu.mem[self.addr as usize];
                    self.addr = self.addr + 1;
                    self.state = 3;
                    false
                } else if self.state == 3 {
                    self.high = cpu.mem[self.addr as usize];
                    self.state = 4;
                    false
                } else {
                    let addr : u16 = mk_addr!(self.low, self.high);
                    self.imm = cpu.mem[addr as usize];
                    execute_load!($reg, self, cpu);
                    true
                }
            }
        }
    }
}
