macro_rules! decode_load_zero_page_reg {
    ($opcode:ident, $cpu:ident, $base:ident) =>
    {{
        if $opcode.state == 0 {
            // read offset from memory
            $opcode.addr = $cpu.read_from_pc();
            $opcode.state = 1;
            false
        } else if $opcode.state == 1 {
            // compute final offset. Wrapping on page 0
            $opcode.addr = $opcode.addr.overflowing_add($cpu.$base).0;
            $opcode.state = 2;
            false
        } else {
            // read data from memory using offset
            $opcode.imm = $cpu.mem[$opcode.addr as usize];
            true
        }
    }};
}

macro_rules! declare_load_zero_page_reg {
    ($name:ident, $reg:ident, $base:ident) => {
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
                decode_load_zero_page_reg!(self, cpu, $base)
            }

            fn execute(&self, cpu: &mut Cpu) {
                execute_imm!($reg, self, cpu)
            }
        }
    }
}
