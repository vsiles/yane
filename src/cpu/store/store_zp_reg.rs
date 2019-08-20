macro_rules! declare_store_zero_page_reg {
    ($mod:ident, $name:ident, $reg:ident, $base:ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;

            // const SIZE : usize = 2;

            pub struct $name {
                addr: u8,
                state: usize,
                saved: u8,
            }

            impl OpCode for $name {
                fn new() -> $name {
                    $name {
                        addr: 0,
                        state: 0,
                        saved: 0,
                    }
                }

                fn decode(&mut self, cpu: &mut Cpu) -> bool {
                    if self.state == 0 {
                        // read offset from memory
                        self.addr = cpu.read_from_pc();
                        self.state = 1;
                        false
                    } else if self.state == 1 {
                        // compute final offset. Wrapping on page 0
                        self.addr = self.addr.overflowing_add(cpu.$base).0;
                        self.state = 2;
                        false
                    } else {
                        let addr = self.addr as u16;
                        self.saved = cpu.mem.get(addr);
                        cpu.mem.set(addr, cpu.$reg);
                        true
                    }
                }
            }
        }
    };
}
