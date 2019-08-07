macro_rules! declare_load_imm {
    ($name:ident, $reg:ident) => {
        pub struct $name {
            imm: u8,
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name { imm: 0 }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                self.imm = cpu.read_from_pc();
                execute_load!($reg, self, cpu);
                true
            }
        }
    }
}
