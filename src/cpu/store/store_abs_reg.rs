macro_rules! declare_store_abs_reg {
    ($name:ident, $reg:ident, $base: ident) => {
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
                    let (low, carry) = self.low.overflowing_add(cpu.$base);
                    self.low = low;
                    self.carry = carry;
                    self.state = 2;
                    false
                } else if self.state == 2 {
                    let addr = mk_addr!(self.low, self.high);
                    let _ = cpu.mem[addr as usize];
                    self.state = 3;
                    if self.carry {
                        self.high = self.high + 1;
                    }
                    false
                } else {
                    let addr : u16 = mk_addr!(self.low, self.high);
                    execute_store!($reg, addr, cpu);
                    true
                }
            }
        }
    }
}
