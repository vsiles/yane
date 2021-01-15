macro_rules! declare_transfert {
    ($name:ident, $from:ident, $to:ident) => {
        pub struct $name {}

        impl OpCode for $name {
            fn new() -> $name {
                $name {}
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                let imm = cpu.$from;
                execute_load!($to, imm, cpu);
                true
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc - 1;
                let code = cpu.mem.get(pc);
                print!(
                    "{:04X}  {:02X}        T{}{}",
                    pc,
                    code,
                    stringify!($from),
                    stringify!($to)
                );
                print!("{: <29}{}", "", cpu)
            }
        }
    };
}
