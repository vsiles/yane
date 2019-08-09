macro_rules! declare_store_zero_page {
    ($mod:ident, $name:ident, $reg:ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;

            const SIZE : usize = 2;

            pub struct $name {
                addr: u8,
                state: usize,
            }

            impl OpCode for $name {
                fn new() -> $name {
                    $name {
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
                    } else {
                        execute_store!($reg, self.addr, cpu);
                        true
                    }
                }

                fn log(&self, cpu: &Cpu) {
                    let pc = (cpu.pc as usize) - SIZE;
                    let code = cpu.mem[pc];
                    print!("{:04X}  {:02X} {:02X}     ST{} ${:02X}", pc, code, 
                           self.addr, stringify!($reg), self.addr);
                    println!(" = {:02X} {: >19}{}", cpu.A, "", cpu)
                } 
            }
        }
    }
}
