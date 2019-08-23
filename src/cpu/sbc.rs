// TODO: Could be implemented using ADC(~input) as ~input is -input -1
use super::super::Cpu;
use super::super::OpCode;

pub struct SbcImm {}

impl OpCode for SbcImm {
    fn new() -> SbcImm {
        SbcImm {}
    }

    fn decode(&mut self, cpu: &mut Cpu) -> bool {
        // https://stackoverflow.com/questions/29193303/6502-emulation-proper-way-to-implement-adc-and-sbc
        let imm: u8 = cpu.read_from_pc();
        let a: u8 = cpu.A;
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
        print!("{:04X}  {:02X} {:02X}     SBC #${:02X}", pc, code, imm, imm);
        print!("{: <24}{}", "", cpu)
    }
}

pub struct SbcNdxInd {
    low: u8,
    high: u8,
    addr: u8,
    state: usize,
}

impl OpCode for SbcNdxInd {
    fn new() -> SbcNdxInd {
        SbcNdxInd {
            low: 0,
            high: 0,
            addr: 0,
            state: 0,
        }
    }

    fn decode(&mut self, cpu: &mut Cpu) -> bool {
        if self.state == 0 {
            // read offset from memory
            self.addr = cpu.read_from_pc();
            self.state = 1;
            false
        } else if self.state == 1 {
            self.addr = self.addr.overflowing_add(cpu.X).0;
            self.state = 2;
            false
        } else if self.state == 2 {
            self.low = cpu.mem.get(self.addr as u16);
            self.addr = self.addr.overflowing_add(1).0;
            self.state = 3;
            false
        } else if self.state == 3 {
            self.high = cpu.mem.get(self.addr as u16);
            self.state = 4;
            false
        } else {
            let addr: u16 = mk_addr!(self.low, self.high);
            let imm = cpu.mem.get(addr);
            let a: u8 = cpu.A;
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
    }

    fn log(&self, cpu: &Cpu) {
        let pc = cpu.pc - 1;
        let code = cpu.mem.get(pc);
        let payload = cpu.mem.get(pc + 1);
        let addr = payload.overflowing_add(cpu.X).0;
        let low = cpu.mem.get(addr as u16);
        let high = cpu.mem.get(addr.overflowing_add(1).0 as u16);
        let faddr = mk_addr!(low, high);
        let imm = cpu.mem.get(faddr);

        print!(
            "{:04X}  {:02X} {:02X}     SBC (${:02X},X)",
            pc,
            code,
            payload,
            payload
        );
        print!(" @ {:02X} = {:04X} = {:02X} {: >3}{}", addr, faddr, imm, "", cpu)
    }
}
