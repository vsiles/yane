pub mod and_imm {
    use super::super::Cpu;
    use super::super::OpCode;

    pub struct AndImm {}

    impl OpCode for AndImm {
        fn new() -> AndImm {
            AndImm {}
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            let imm = cpu.read_from_pc() & cpu.A;
            execute_load!(A, imm, cpu);
            true
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            let imm = cpu.mem.get(pc + 1);
            print!(
                "{:04X}  {:02X} {:02X}     AND #${:02X}",
                pc,
                code,
                imm,
                imm
            );
            print!("{: <24}{}", "", cpu)
        }
    }
}
