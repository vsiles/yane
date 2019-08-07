macro_rules! declare_store_zero_page {
    ($name:ident, $reg:ident) => {
        pub struct $name {
            addr: u8,
            state: usize
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    addr: 0,
                    state: 0
                }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                if self.state == 0 {
                    // read offset from memory
                    self.addr = cpu.read_from_pc();
                    self.state = 1;
                    false
                } else {
                    execute_store!($reg, self.addr, cpu);
                    true
                }
            }
        }
    }
}
