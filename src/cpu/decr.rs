macro_rules! declare_decr {
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
                    let val = cpu.$reg.overflowing_sub(1);
                    cpu.$reg = val.0;
                    cpu.flags.zero = cpu.$reg == 0;
                    cpu.flags.negative = (cpu.$reg & 0x80) != 0;
                    true
                }

                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc - 1;
                    let code = cpu.mem.get(pc);
                    print!("{:04X}  {:02X}        DE{}", pc, code, stringify!($reg));
                    print!("{: <29}{}", "", cpu)
                }
            }
        }
    };
}
