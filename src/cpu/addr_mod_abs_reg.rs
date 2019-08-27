macro_rules! declare_addr_abs_reg {
    ($name:ident, $mnemo:ident, $reg:ident, $action:expr) => {
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
                    let (low, carry) = self.low.overflowing_add(cpu.$reg);
                    self.low = low;
                    self.carry = carry;
                    self.state = 2;
                    false
                } else if self.state == 2 {
                    let addr: u16 = mk_addr!(self.low, self.high);
                    if self.carry {
                        self.high = self.high.overflowing_add(1).0;
                        self.state = 3;
                        false
                    } else {
                        $action(cpu, addr as usize);
                        true
                    }
                } else {
                    let addr: u16 = mk_addr!(self.low, self.high);
                    $action(cpu, addr as usize);
                    true
                }
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc - 1;
                let code = cpu.mem.get(pc);
                let low = cpu.mem.get(pc + 1);
                let high = cpu.mem.get(pc + 2);
                let base = mk_addr!(low, high);
                let addr = base.overflowing_add(cpu.$reg as u16).0;
                let imm = cpu.mem.get(addr);
                println!("Debuging:  loading {:#X} from {:#X}", imm, addr);
                print!(
                    "{:04X}  {:02X} {:02X} {:02X}  {} ${:04X},{}",
                    pc,
                    code,
                    low,
                    high,
                    stringify!($mnemo),
                    base,
                    stringify!($reg)
                );
                print!(" @ {:04X} = {:02X} {: >8}{}", addr, imm, "", cpu)
            }
        }
    };
}
