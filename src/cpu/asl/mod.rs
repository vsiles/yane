pub mod asl_a {
    use super::super::Cpu;
    use super::super::OpCode;

    pub struct AslA {}

    impl OpCode for AslA {
        fn new() -> AslA {
            AslA {}
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            cpu.flags.carry = (cpu.A & 0x80) != 0;
            let imm = cpu.A << 1;
            execute_load!(A, imm, cpu);
            true
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            print!("{:04X}  {:02X}        ASL A", pc, code,);
            print!("{: <27}{}", "", cpu);
        }
    }
}
