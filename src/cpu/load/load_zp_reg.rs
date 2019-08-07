macro_rules! declare_load_zero_page_reg {
    ($name:ident, $reg:ident, $base:ident) => {
        pub struct $name {
            addr: u8,
            imm: u8,
            state: usize,
            size: usize,
        }

        impl OpCode for $name {
            fn new(size: usize) -> $name {
                $name {
                    addr: 0,
                    imm: 0,
                    state: 0,
                    size: size,
                }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                if self.state == 0 {
                    // read offset from memory
                    self.addr = cpu.read_from_pc();
                    self.state = 1;
                    false
                } else if self.state == 1 {
                    // compute final offset. Wrapping on page 0
                    self.addr = self.addr.overflowing_add(cpu.$base).0;
                    self.state = 2;
                    false
                } else {
                    // read data from memory using offset
                    self.imm = cpu.mem[self.addr as usize];
                    execute_load!($reg, self, cpu);
                    true
                }
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc;
                let upc : usize = pc as usize;
                let code = cpu.mem[upc - self.size];
                let payload = cpu.mem[upc + 1 - self.size];
                print!("{:04X}  {:02X} {:02X}     LDA ${:02X},{}", pc, code, 
                    payload, payload, stringify!($base));
                println!(" @ {:02X} = {:02X}{: >13}{}", self.addr, self.imm,
                    "", cpu)
            } 
        }
    }
}
