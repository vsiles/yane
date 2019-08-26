macro_rules! declare_bin_zero_page {
    ($mod:ident, $name:ident, $reg:ident, $mnemo:ident, $action:expr) => {
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
                        // read data from memory using offset in page 0
                        let imm = $action(cpu.mem.get(self.addr as u16), cpu.$reg);
                        execute_load!($reg, imm, cpu);
                        true
                    }
                }

                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc - 1;
                    let code = cpu.mem.get(pc);
                    let payload = cpu.mem.get(pc + 1);
                    print!(
                        "{:04X}  {:02X} {:02X}     {} ${:02X}",
                        pc,
                        code,
                        payload,
                        stringify!($mnemo),
                        payload
                    );
                    let imm = cpu.mem.get(payload as u16);
                    print!(" = {:02X}{: >20}{}", imm, "", cpu)
                }
            }
        }
    };
}
