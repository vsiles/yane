macro_rules! declare_load_zero_page {
    ($name:ident, $reg:ident) => {
        pub struct $name {
            addr: u8,
            imm: u8,
            state: usize,
            size: usize,
        }

        impl OpCode for $name {
            fn new(size: usize) -> $name {
                $name {
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
                } else {
                    // read data from memory using offset in page 0
                    self.imm = cpu.mem[self.addr as usize];
                    execute_load!($reg, self, cpu);
                    true
                }
            }
        }
    }
}
