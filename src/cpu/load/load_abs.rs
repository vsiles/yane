macro_rules! declare_load_abs {
    ($name:ident, $reg:ident) => {
        pub struct $name {
            low: u8,
            high: u8,
            imm: u8,
            state: usize
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    low: 0,
                    high: 0,
                    imm: 0,
                    state: 0
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
                    let addr : u16 = mk_addr!(self.low, self.high);
                    self.imm = cpu.mem[addr as usize];
                    execute_load!($reg, self, cpu);
                    true
                }
            }
        }
    }
}
