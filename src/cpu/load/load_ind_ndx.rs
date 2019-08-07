macro_rules! decode_load_ind_ndx {
    ($opcode:ident, $cpu:ident) =>
    {{
         if $opcode.state == 0 {
             // read offset from memory
             $opcode.addr = $cpu.read_from_pc();
             $opcode.state = 1;
             false
         } else if $opcode.state == 1 {
             $opcode.low = $cpu.mem[$opcode.addr as usize];
             $opcode.addr = $opcode.addr + 1;
             $opcode.state = 2;
             false
         } else if $opcode.state == 2 {
             $opcode.high = $cpu.mem[$opcode.addr as usize];
             let (low, carry) = $opcode.low.overflowing_add($cpu.y);
             $opcode.low = low;
             $opcode.carry = carry;
             $opcode.state = 3;
             false
         } else if $opcode.state == 3 {
             let addr : u16 = mk_addr!($opcode.low, $opcode.high);
             $opcode.imm = $cpu.mem[addr as usize];
             $opcode.state = 4;
             if $opcode.carry {
                 $opcode.high = $opcode.high + 1;
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

macro_rules! declare_load_ind_ndx {
    ($name:ident, $reg:ident) => {
        pub struct $name {
            low: u8,
            high: u8,
            carry: bool,
            addr: u8,
            imm: u8,
            state: usize
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    low: 0,
                    high: 0,
                    addr: 0,
                    carry: false,
                    imm: 0,
                    state: 0
                }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                decode_load_ind_ndx!(self, cpu)
            }

            fn execute(&self, cpu: &mut Cpu) {
                execute_imm!($reg, self, cpu)
            }
        }
    }
}
