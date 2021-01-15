// return from irq routine
use super::Cpu;
use super::OpCode;

pub struct Rti {
    state: usize,
    low: u8,
    high: u8,
}

impl OpCode for Rti {
    fn new() -> Rti {
        Rti {
            state: 0,
            low: 0,
            high: 0,
        }
    }

    fn decode(&mut self, cpu: &mut Cpu) -> bool {
        if self.state == 0 {
            // 2    PC     R  read next instruction byte (and throw it away)
            let _ = cpu.mem.get(cpu.pc);
            self.state = 1;
            false
        } else if self.state == 1 {
            // 3  $0100,S  R  increment S
            let (sp, _) = cpu.sp.overflowing_add(1);
            cpu.sp = sp;
            self.state = 2;
            false
        } else if self.state == 2 {
            // 4  $0100,S  R  pull P from stack, increment S
            // https://wiki.nesdev.com/w/index.php/Status_flags
            let sp: u16 = mk_addr!(cpu.sp, 0x01);
            let pflags = cpu.mem.get(sp);
            cpu.flags.update(pflags);
            let (sp, _) = cpu.sp.overflowing_add(1);
            cpu.sp = sp;
            self.state = 3;
            false
        } else if self.state == 3 {
            // 5  $0100,S  R  pull PCL from stack, increment S
            let sp: u16 = mk_addr!(cpu.sp, 0x01);
            self.low = cpu.mem.get(sp);
            let (sp, _) = cpu.sp.overflowing_add(1);
            cpu.sp = sp;
            self.state = 4;
            false
        } else {
            // 5  $0100,S  R  pull PCH from stack
            let sp: u16 = mk_addr!(cpu.sp, 0x01);
            self.high = cpu.mem.get(sp);
            cpu.pc = mk_addr!(self.low, self.high);
            true
        }
    }

    fn log(&self, cpu: &Cpu) {
        let pc = cpu.pc - 1;
        let code = cpu.mem.get(pc);
        print!("{:04X}  {:02X}        RTI", pc, code);
        print!("{: >29}{}", "", cpu)
    }
}
