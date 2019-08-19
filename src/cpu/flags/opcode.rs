macro_rules! declare_flags_opcode {
    ($mod:ident, $name:ident, $fname:ident, $flag:ident, $val:expr) => {
        pub mod $mod {
            use super::Cpu;
            use super::OpCode;

            const SIZE: u16 = 1;

            pub struct $name {}

            impl OpCode for $name {
                fn new() -> $name {
                    $name {}
                }
                fn decode(&mut self, cpu: &mut Cpu) -> bool {
                    let _ = cpu.mem.get(cpu.pc);
                    cpu.flags.$flag = $val;
                    true
                }
                fn log(&self, cpu: &Cpu) {
                    let pc = cpu.pc - SIZE;
                    let code = cpu.mem.get(pc);
                    print!("{:04X}  {:02X}        {}", pc, code, stringify!($fname));
                    println!("{: >29}{}", "", cpu)
                }
            }
        }
    };
}
