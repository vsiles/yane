macro_rules! declare_store_zero_page {
    ($mod:ident, $name:ident, $reg:ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;

            pub struct $name {
                addr: u8,
                state: usize,
            }

            impl OpCode for $name {
                fn new() -> $name {
                    $name { addr: 0, state: 0 }
                }

                fn decode(&mut self, cpu: &mut Cpu) -> bool {
                    if self.state == 0 {
                        // read offset from memory
                        self.addr = cpu.read_from_pc();
                        self.state = 1;
                        false
                    } else {
                        let addr = self.addr as u16;
                        cpu.mem.set(addr, cpu.$reg);
                        true
                    }
                }

                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc - 1;
                    let code = cpu.mem.get(pc);
                    let addr = cpu.mem.get(pc + 1);
                    let old = cpu.mem.get(addr as u16);
                    print!(
                        "{:04X}  {:02X} {:02X}     ST{} ${:02X}",
                        pc,
                        code,
                        addr,
                        stringify!($reg),
                        addr
                    );
                    print!(" = {:02X} {: >19}{}", old, "", cpu)
                }
            }
        }
    };
}
