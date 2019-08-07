macro_rules! decode_load_imm {
    ($opcode:ident, $cpu:ident) =>
    {{
        $opcode.imm = $cpu.read_from_pc();
        true
    }};
}

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
                decode_load_imm!(self, cpu)
            }

            fn execute(&self, cpu: &mut Cpu) {
                execute_load!($reg, self, cpu)
            }
        }
    }
}
