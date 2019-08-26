use super::Cpu;
use super::OpCode;

pub struct CmpNdxInd {
    low: u8,
    high: u8,
    addr: u8,
    state: usize,
}

impl OpCode for CmpNdxInd {
    fn new() -> CmpNdxInd {
        CmpNdxInd {
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
            let (res, _) = cpu.A.overflowing_sub(imm);
            cpu.flags.carry = cpu.A >= imm;
            cpu.flags.zero = cpu.A == imm;
            cpu.flags.negative = (res & 0x80) != 0;
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
            "{:04X}  {:02X} {:02X}     CMP (${:02X},X)",
            pc,
            code,
            payload,
            payload
        );
        print!(" @ {:02X} = {:04X} = {:02X} {: >3}{}", addr, faddr, imm, "", cpu)
    }
}
