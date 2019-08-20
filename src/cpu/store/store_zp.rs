macro_rules! declare_store_zero_page {
    ($mod:ident, $name:ident, $reg:ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;

            const SIZE: u16 = 2;

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
                    } else {
                        let addr = self.addr as u16;
                        self.saved = cpu.mem.get(addr);
                        cpu.mem.set(addr, cpu.$reg);
                        true
                    }
                }

                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc - SIZE;
                    let code = cpu.mem.get(pc);
                    print!(
                        "{:04X}  {:02X} {:02X}     ST{} ${:02X}",
                        pc,
                        code,
                        self.addr,
                        stringify!($reg),
                        self.addr
                    );
                    println!(" = {:02X} {: >19}{}", self.saved, "", cpu)
                }
            }
        }
    };
}
