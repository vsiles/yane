macro_rules! declare_load_abs_reg {
    ($name:ident, $reg:ident, $base: ident) => {
        pub struct $name {
            low: u8,
            high: u8,
            carry: bool,
            imm: u8,
            state: usize
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    low: 0,
                    high: 0,
                    carry: false,
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
                    let (low, carry) = self.low.overflowing_add(cpu.$base);
                    self.low = low;
                    self.carry = carry;
                    self.state = 2;
                    false
                } else if self.state == 2 {
                    let addr : u16 = mk_addr!(self.low, self.high);
                    self.imm = cpu.mem[addr as usize];
                    if self.carry {
                        self.high = self.high + 1;
                        self.state = 3;
                        false
                    } else {
                        execute_load!($reg, self, cpu);
                        true
                    }
                } else {
                    let addr : u16 = mk_addr!(self.low, self.high);
                    self.imm = cpu.mem[addr as usize];
                    println!("Loading {:#x} into reg", self.imm);
                    execute_load!($reg, self, cpu);
                    true
                }
            }
        }
    }
}
