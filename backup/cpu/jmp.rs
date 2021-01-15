use super::Cpu;
use super::OpCode;

pub struct Jmp {
    low: u8,
    high: u8,
    state: usize,
}

impl OpCode for Jmp {
    fn new() -> Jmp {
        Jmp {
            low: 0,
            high: 0,
            state: 0,
        }
    }

    fn decode(&mut self, cpu: &mut Cpu) -> bool {
        if self.state == 0 {
            self.low = cpu.read_from_pc();
            self.state = 1;
            false
        } else {
            self.high = cpu.read_from_pc();
            cpu.pc = mk_addr!(self.low, self.high);
            true
        }
    }

    fn log(&self, cpu: &Cpu) {
        let pc = cpu.pc - 1;
        let code = cpu.mem.get(pc);
        let low = cpu.mem.get(pc + 1);
        let high = cpu.mem.get(pc + 2);
        let addr = mk_addr!(low, high);
        print!(
            "{:04X}  {:02X} {:02X} {:02X}  JMP ${:04X}",
            pc, code, low, high, addr
        );
        print!("{: >23}{}", "", cpu)
    }
}

pub struct JmpInd {
    low: u8,
    high: u8,
    pcl: u8,
    state: usize,
}

impl OpCode for JmpInd {
    fn new() -> JmpInd {
        JmpInd {
            low: 0,
            high: 0,
            pcl: 0,
            state: 0,
        }
    }

    fn decode(&mut self, cpu: &mut Cpu) -> bool {
        if self.state == 0 {
	    // 2     PC      R  fetch pointer address low, increment PC
            self.low = cpu.read_from_pc();
            self.state = 1;
            false
        } else if self.state == 1 {
	    // 3     PC      R  fetch pointer address high, increment PC
            self.high = cpu.read_from_pc();
            self.state = 2;
            false
        } else if self.state == 2 {
            // 4   pointer   R  fetch low address to latch
            let addr = mk_addr!(self.low, self.high);
            self.pcl = cpu.mem.get(addr);
            self.state = 3;
            false
        } else {
            // 5  pointer+1* R  fetch PCH, copy latch to PCL
            // Note: * The PCH will always be fetched from the same page
            //         than PCL, i.e. page boundary crossing is not handled.
            self.low = self.low.overflowing_add(1).0;
            let addr = mk_addr!(self.low, self.high);
            let pch = cpu.mem.get(addr);
            cpu.pc = mk_addr!(self.pcl, pch);
            true
        }
    }

    fn log(&self, cpu: &Cpu) {
        let pc = cpu.pc - 1;
        let code = cpu.mem.get(pc);
        let low = cpu.mem.get(pc + 1);
        let high = cpu.mem.get(pc + 2);
        let addr0 = mk_addr!(low, high);
        let addr1 = mk_addr!(low.overflowing_add(1).0, high);
        let flow = cpu.mem.get(addr0);
        let fhigh = cpu.mem.get(addr1);
        let faddr = mk_addr!(flow, fhigh);
        print!(
            "{:04X}  {:02X} {:02X} {:02X}  JMP (${:04X}) = {:04X}",
            pc, code, low, high, addr0, faddr
        );
        print!("{: >14}{}", "", cpu)
    }
}
