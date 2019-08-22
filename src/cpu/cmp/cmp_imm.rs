macro_rules! declare_cmp_imm {
    ($mod:ident, $name:ident, $mnemo:ident, $reg:ident) => {
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
                    let (res, _) = cpu.$reg.overflowing_sub(imm);
                    cpu.flags.carry = cpu.$reg >= imm;
                    cpu.flags.zero = cpu.$reg == imm;
                    cpu.flags.negative = (res & 0x80) != 0;
                    true
                }

                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc - 1;
                    let code = cpu.mem.get(pc);
                    let imm = cpu.mem.get(pc + 1);
                    print!(
                        "{:04X}  {:02X} {:02X}     {} #${:02X}",
                        pc,
                        code,
                        imm,
                        stringify!($mnemo),
                        imm
                    );
                    print!("{: <24}{}", "", cpu)
                }
            }
        }
    };
}
