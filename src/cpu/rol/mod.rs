pub mod rol_a {
    use super::super::Cpu;
    use super::super::OpCode;

    pub struct RolA {}

    impl OpCode for RolA {
        fn new() -> RolA {
            RolA {}
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            let imm = (cpu.A << 1) | (if cpu.flags.carry { 0x01 } else { 0 });
            cpu.flags.carry = (cpu.A & 0x80) != 0;
            execute_load!(A, imm, cpu);
            true
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            print!("{:04X}  {:02X}        ROL A", pc, code,);
            print!("{: <27}{}", "", cpu);
        }
    }
}
