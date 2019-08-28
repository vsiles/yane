macro_rules! declare_addr_imm_raw {
    ($name:ident, $mnemo: ident, $action:expr, $illegal:expr) => {
        pub struct $name {}

        impl OpCode for $name {
            fn new() -> $name {
                $name {}
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                let imm = cpu.read_from_pc() as usize;
                $action(cpu, imm);
                true
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc - 1;
                let code = cpu.mem.get(pc);
                let imm = cpu.mem.get(pc + 1);
                print!("{:04X}  {:02X} {:02X}    {}{} #${:02X}", pc, code, imm,
                       if $illegal { "*" } else { " " },
                       stringify!($mnemo), imm);
                print!("{: <24}{}", "", cpu);
            }
        }
    };
}

macro_rules! declare_addr_imm {
    ($name:ident, $mnemo:ident, $action:expr) => {
        declare_addr_imm_raw!($name, $mnemo, $action, false);
    };
    ($name:ident, $mnemo:ident, $action:expr, $illegal:expr) => {
        declare_addr_imm_raw!($name, $mnemo, $action, $illegal);
    };
}
