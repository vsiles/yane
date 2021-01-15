macro_rules! declare_flags_opcode {
    ($name:ident, $fname:ident, $flag:ident, $val:expr) => {
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
                let pc = cpu.pc - 1;
                let code = cpu.mem.get(pc);
                print!("{:04X}  {:02X}        {}", pc, code, stringify!($fname));
                print!("{: >29}{}", "", cpu)
            }
        }
    };
}
