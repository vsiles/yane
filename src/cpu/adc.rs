pub mod adc_imm {
    use super::super::Cpu;
    use super::super::OpCode;
    use super::super::flags::CpuFlags;

    const SIZE: u16 = 2;

    pub struct AdcImm {
        saved: u8,
        oldf: CpuFlags,
    }

    impl OpCode for AdcImm {
        fn new() -> AdcImm {
            AdcImm {
                saved: 0,
                oldf: CpuFlags::new(),
            }
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            // https://stackoverflow.com/questions/29193303/6502-emulation-proper-way-to-implement-adc-and-sbc
            self.oldf = cpu.flags.clone();
            self.saved = cpu.A;
            let imm : usize = cpu.read_from_pc() as usize;
            let a : usize = cpu.A as usize;
            let val : usize = a + imm + (if cpu.flags.carry { 1 } else { 0 });
            cpu.flags.overflow = ((a ^ val) & (imm ^ val) & 0x80) == 0x80;
            cpu.flags.carry = (val & 0x100) == 0x100;
            cpu.flags.zero = (val & 0xFF) == 0;
            cpu.flags.negative = (val & 0x80) == 0x80;
            cpu.A = (val & 0xFF) as u8;
            true
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - SIZE;
            let code = cpu.mem.get(pc);
            let imm = cpu.mem.get(pc + 1);
            print!(
                "{:04X}  {:02X} {:02X}     ADC #${:02X}",
                pc,
                code,
                imm,
                imm
            );
            let mut old_cpu = cpu.debug_clone();
            old_cpu.A = self.saved;
            old_cpu.flags = self.oldf.clone();
            print!("{: <24}{}", "", old_cpu);
        }
    }
}
