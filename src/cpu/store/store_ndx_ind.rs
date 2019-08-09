macro_rules! declare_store_ndx_ind {
    ($mod:ident, $name:ident, $reg:ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;

            // const SIZE : usize = 2;

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
                        self.low = cpu.mem[self.addr as usize];
                        self.addr = self.addr + 1;
                        self.state = 3;
                        false
                    } else if self.state == 3 {
                        self.high = cpu.mem[self.addr as usize];
                        self.state = 4;
                        false
                    } else {
                        let addr: u16 = mk_addr!(self.low, self.high);
                        execute_store!($reg, addr, cpu);
                        true
                    }
                }
            }
        }
    };
}
