macro_rules! declare_branch {
    ($name:ident, $flag:ident, $val:expr, $mnemo:ident) => {
        pub struct $name {
            imm: u8,
            state: usize,
            carry: bool,
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    imm: 0,
                    state: 0,
                    carry: false,
                }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                if self.state == 0 {
                    self.imm = cpu.read_from_pc();
                    self.state = self.state + 1;
                    false
                } else if self.state == 1 {
                    self.state = self.state + 1;
                    if cpu.flags.$flag == $val {
                        let pc = cpu.pc.overflowing_add(self.imm as u16);
                        cpu.pc = pc.0;
                        self.carry = pc.1;
                        false
                    } else {
                        true
                    }
                } else {
                    if self.carry {
                        cpu.pc = cpu.pc + 0x100
                    }
                    true
                }
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc - 1;
                let code = cpu.mem.get(pc);
                let imm = cpu.mem.get(pc + 1);
                let addr = (pc as usize) + 2 + (imm as usize);
                print!(
                    "{:04X}  {:02X} {:02X}     {} ${:04X}",
                    pc,
                    code,
                    imm,
                    stringify!($mnemo),
                    addr
                );
                print!("{: <23}{}", "", cpu)
            }
        }
    };
}
