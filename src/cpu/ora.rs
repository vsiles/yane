pub mod ora_imm {
    use super::super::Cpu;
    use super::super::OpCode;

    pub struct OraImm {
        imm: u8,
    }

    impl OpCode for OraImm {
        fn new() -> OraImm {
            OraImm {
                imm: 0,
            }
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            self.imm = cpu.read_from_pc() | cpu.A;
            execute_load!(A, self, cpu);
            true
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            let imm = cpu.mem.get(pc + 1);
            print!(
                "{:04X}  {:02X} {:02X}     ORA #${:02X}",
                pc,
                code,
                imm,
                imm
            );
            print!("{: <24}{}", "", cpu);
        }
    }
}
