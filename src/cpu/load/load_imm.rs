macro_rules! declare_load_imm {
    ($name:ident, $reg:ident) => {
        pub struct $name {
            imm: u8,
            size: usize,
        }

        impl OpCode for $name {
            fn new(size: usize) -> $name {
                $name {
                    imm: 0,
                    size: size,
                }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                self.imm = cpu.read_from_pc();
                execute_load!($reg, self, cpu);
                true
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc;
                let upc : usize = pc as usize;
                let code = cpu.mem[upc - self.size];
                let imm = self.imm;
                print!("{:04X}  {:02X} {:02X}    LDA #${:02X}", pc, code, imm, imm);
                println!("{: <24}{}", "", cpu)
            } 
        }
    }
}
