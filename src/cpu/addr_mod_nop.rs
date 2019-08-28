macro_rules! declare_addr_nop_raw {
    ($name:ident, $mnemo: ident, $illegal:expr) => {
        pub struct $name {}

        impl OpCode for $name {
            fn new() -> $name {
                $name {}
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                let _ = cpu.mem.get(cpu.pc);
                true
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc - 1;
                let code = cpu.mem.get(pc);
                print!("{:04X}  {:02X}       {}{}", pc, code,
                       if $illegal { "*" } else { " " },
                       stringify!($mnemo));
                print!("{: <29}{}", "", cpu);
            }
        }
    };
}

macro_rules! declare_addr_nop {
    ($name:ident, $mnemo: ident) => {
        declare_addr_nop_raw!($name, $mnemo, false);
    };
    ($name:ident, $mnemo: ident, $illegal:expr) => {
        declare_addr_nop_raw!($name, $mnemo, $illegal);
    };
}
