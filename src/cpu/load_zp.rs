macro_rules! decode_load_zero_page {
    ($opcode:ident, $cpu:ident) =>
    {{
        if $opcode.state == 0 {
            // read offset from memory
            $opcode.addr = $cpu.read_from_pc();
            $opcode.state = 1;
            false
        } else {
            // read data from memory using offset in page 0
            $opcode.imm = $cpu.mem[$opcode.addr as usize];
            true
        }
    }};
}

macro_rules! declare_load_zero_page {
    ($name:ident, $reg:ident) => {
        pub struct $name {
            addr: u8,
            imm: u8,
            state: usize
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    addr: 0,
                    imm: 0,
                    state: 0
                }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                decode_load_zero_page!(self, cpu)
            }

            fn execute(&self, cpu: &mut Cpu) {
                execute_imm!($reg, self, cpu)
            }
        }
    }
}
