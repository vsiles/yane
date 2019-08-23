pub mod ror_a {
    use super::super::Cpu;
    use super::super::OpCode;

    pub struct RorA {}

    impl OpCode for RorA {
        fn new() -> RorA {
            RorA {}
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            let imm: u8 = (cpu.A >> 1) | (if cpu.flags.carry { 0x80 } else { 0 });
            cpu.flags.carry = (cpu.A & 0x01) != 0;
            execute_load!(A, imm, cpu);
            true
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            print!("{:04X}  {:02X}        ROR A", pc, code,);
            print!("{: <27}{}", "", cpu);
        }
    }
}
