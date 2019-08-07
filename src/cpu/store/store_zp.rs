macro_rules! decode_store_zero_page {
    ($opcode:ident, $cpu:ident) =>
    {{
        if $opcode.state == 0 {
            // read offset from memory
            $opcode.addr = $cpu.read_from_pc();
            $opcode.state = 1;
            false
        } else {
            true
        }
    }};
}

macro_rules! declare_store_zero_page {
    ($name:ident, $reg:ident) => {
        pub struct $name {
            addr: u8,
            state: usize
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    addr: 0,
                    state: 0
                }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                decode_store_zero_page!(self, cpu)
            }

            fn execute(&self, cpu: &mut Cpu) {
                execute_store!($reg, self, cpu)
            }
        }
    }
}
