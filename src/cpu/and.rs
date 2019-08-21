pub mod and_imm {
    use super::super::Cpu;
    use super::super::OpCode;
    use super::super::flags::CpuFlags;

    const SIZE: u16 = 2;

    pub struct AndImm {
        imm: u8,
        saved: u8,
        oldf: CpuFlags,
    }

    impl OpCode for AndImm {
        fn new() -> AndImm {
            AndImm {
                imm: 0,
                saved: 0,
                oldf: CpuFlags::new(),
            }
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            self.oldf = cpu.flags.clone();
            self.saved = cpu.A;
            self.imm = cpu.read_from_pc() & cpu.A;
            execute_load!(A, self, cpu);
            true
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - SIZE;
            let code = cpu.mem.get(pc);
            let imm = cpu.mem.get(pc + 1);
            print!(
                "{:04X}  {:02X} {:02X}     AND #${:02X}",
                pc,
                code,
                imm,
                imm
            );
            let mut old_cpu = cpu.debug_clone();
            old_cpu.A = self.saved;
            old_cpu.flags = self.oldf.clone();
            println!("{: <24}{}", "", old_cpu);
        }
    }
}
