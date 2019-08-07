macro_rules! decode_load_abs_reg {
    ($opcode:ident, $cpu:ident, $base:ident) =>
    {{
        if $opcode.state == 0 {
            $opcode.low = $cpu.read_from_pc();
            $opcode.state = 1;
            false
        } else if $opcode.state == 1 {
            $opcode.high = $cpu.read_from_pc();
            let (low, carry) = $opcode.low.overflowing_add($cpu.$base);
            $opcode.low = low;
            $opcode.carry = carry;
            $opcode.state = 2;
            false
        } else if $opcode.state == 2 {
            let addr : u16 = mk_addr!($opcode.low, $opcode.high);
            $opcode.imm = $cpu.mem[addr as usize];
            if $opcode.carry {
                $opcode.high = $opcode.high + 1;
                $opcode.state = 3;
                false
            } else {
                true
            }
        } else {
            let addr : u16 = mk_addr!($opcode.low, $opcode.high);
            $opcode.imm = $cpu.mem[addr as usize];
            true
        }
    }};
}

macro_rules! declare_load_abs_reg {
    ($name:ident, $reg:ident, $base: ident) => {
        pub struct $name {
            low: u8,
            high: u8,
            carry: bool,
            imm: u8,
            state: usize
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    low: 0,
                    high: 0,
                    carry: false,
                    imm: 0,
                    state: 0
                }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                decode_load_abs_reg!(self, cpu, $base)
            }

            fn execute(&self, cpu: &mut Cpu) {
                execute_imm!($reg, self, cpu)
            }
        }
    }
}
