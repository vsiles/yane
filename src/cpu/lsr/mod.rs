pub mod lsr_a {
    use super::super::Cpu;
    use super::super::OpCode;

    pub struct LsrA {}

    impl OpCode for LsrA {
        fn new() -> LsrA {
            LsrA {}
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            cpu.flags.carry = (cpu.A & 0x01) != 0;
            let imm = cpu.A >> 1;
            execute_load!(A, imm, cpu);
            true
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            print!("{:04X}  {:02X}        LSR A", pc, code,);
            print!("{: <27}{}", "", cpu);
        }
    }
}
