// TODO: Could be implemented using ADC(~input) as ~input is -input -1
pub mod sbc_imm {
    use super::super::Cpu;
    use super::super::OpCode;

    pub struct SbcImm {}

    impl OpCode for SbcImm {
        fn new() -> SbcImm {
            SbcImm {}
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            // https://stackoverflow.com/questions/29193303/6502-emulation-proper-way-to-implement-adc-and-sbc
            let imm : u8 = cpu.read_from_pc();
            let a : u8 = cpu.A;
            let (val0, _) = a.overflowing_sub(imm);
            let (val, _) = val0.overflowing_sub(if cpu.flags.carry { 0 } else { 1 });
            let res = (a as i32) - (imm as i32) - (if cpu.flags.carry { 0 } else { 1 });
            cpu.flags.overflow = ((a ^ val) & (a ^ imm) & 0x80) == 0x80;
            cpu.flags.carry = res >= 0;
            cpu.flags.zero = val == 0;
            cpu.flags.negative = (val & 0x80) == 0x80;
            cpu.A = val;
            true
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            let imm = cpu.mem.get(pc + 1);
            print!(
                "{:04X}  {:02X} {:02X}     SBC #${:02X}",
                pc,
                code,
                imm,
                imm
            );
            print!("{: <24}{}", "", cpu)
        }
    }
}
