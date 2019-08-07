macro_rules! decode_load_abs {
    ($opcode:ident, $cpu:ident) =>
    {{
        if $opcode.state == 0 {
            $opcode.low = $cpu.read_from_pc();
            $opcode.state = 1;
            false
        } else if $opcode.state == 1 {
            $opcode.high = $cpu.read_from_pc();
            $opcode.state = 2;
            false
        } else {
            let addr : u16 = mk_addr!($opcode.low, $opcode.high);
            $opcode.imm = $cpu.mem[addr as usize];
            true
        }
    }};
}

macro_rules! declare_load_abs {
    ($name:ident, $reg:ident) => {
        pub struct $name {
            low: u8,
            high: u8,
            imm: u8,
            state: usize
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    low: 0,
                    high: 0,
                    imm: 0,
                    state: 0
                }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                decode_load_abs!(self, cpu)
            }

            fn execute(&self, cpu: &mut Cpu) {
                execute_imm!($reg, self, cpu)
            }
        }
    }
}
