macro_rules! declare_load_imm {
    ($mod:ident, $name:ident, $reg:ident) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;

            pub struct $name {}

            impl OpCode for $name {
                fn new() -> $name {
                    $name {}
                }

                fn decode(&mut self, cpu: &mut Cpu) -> bool {
                    let imm = cpu.read_from_pc();
                    execute_load!($reg, imm, cpu);
                    true
                }

                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc - 1;
                    let code = cpu.mem.get(pc);
                    let imm = cpu.mem.get(pc + 1);
                    print!(
                        "{:04X}  {:02X} {:02X}     LD{} #${:02X}",
                        pc,
                        code,
                        imm,
                        stringify!($reg),
                        imm
                    );
                    print!("{: <24}{}", "", cpu);
                }
            }
        }
    };
}
