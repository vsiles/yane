macro_rules! declare_load_zero_page {
    ($mod:ident, $name:ident, $reg:ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;

            const SIZE: u16 = 2;

            pub struct $name {
                addr: u8,
                imm: u8,
                state: usize,
                saved: u8,
            }

            impl OpCode for $name {
                fn new() -> $name {
                    $name {
                        addr: 0,
                        imm: 0,
                        state: 0,
                        saved: 0,
                    }
                }

                fn decode(&mut self, cpu: &mut Cpu) -> bool {
                    if self.state == 0 {
                        self.saved = cpu.$reg;
                        // read offset from memory
                        self.addr = cpu.read_from_pc();
                        self.state = 1;
                        false
                    } else {
                        // read data from memory using offset in page 0
                        self.imm = cpu.mem.get(self.addr as u16);
                        execute_load!($reg, self, cpu);
                        true
                    }
                }

                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc - SIZE;
                    let code = cpu.mem.get(pc);
                    let payload = cpu.mem.get(pc + 1);
                    print!(
                        "{:04X}  {:02X} {:02X}     LD{} ${:02X}",
                        pc,
                        code,
                        payload,
                        stringify!($reg),
                        payload
                    );
                    let mut old_cpu = cpu.debug_clone();
                    old_cpu.$reg = self.saved;
                    print!(" = {:02X}{: >20}{}", self.imm, "", old_cpu)
                }
            }
        }
    };
}
