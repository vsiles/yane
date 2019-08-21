// return from sub routine
use super::Cpu;
use super::OpCode;

const SIZE: u16 = 1;

pub struct Rts {
    state: usize,
    low: u8,
    high: u8,
    old: u16,
    old_sp: u8,
}

impl OpCode for Rts {
    fn new() -> Rts {
        Rts {
            state: 0,
            low: 0,
            high: 0,
            old: 0,
            old_sp: 0,
        }
    }

    fn decode(&mut self, cpu: &mut Cpu) -> bool {
        if self.state == 0 {
            // 2    PC     R  read next instruction byte (and throw it away)
            self.old = cpu.pc;
            self.old_sp = cpu.sp;
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
            // 4  $0100,S  R  pull PCL from stack, increment S
            let sp: u16 = 0x1000 + cpu.sp as u16;
            self.low = cpu.mem.get(sp);
            let (sp, _) = cpu.sp.overflowing_add(1);
            cpu.sp = sp;
            self.state = 3;
            false
        } else if self.state == 3 {
            // 5  $0100,S  R  pull PCH from stack
            let sp: u16 = 0x1000 + cpu.sp as u16;
            self.high = cpu.mem.get(sp);
            self.state = 4;
            false
        } else {
            // 6    PC     R  increment PC
            let addr = mk_addr!(self.low, self.high);
            cpu.pc = addr + 1;
            true
        }
    }

    fn log(&self, cpu: &Cpu) {
        let pc = self.old - SIZE;
        let code = cpu.mem.get(pc);
        print!("{:04X}  {:02X}        RTS", pc, code);
        let mut old_cpu = cpu.debug_clone();
        old_cpu.sp = self.old_sp;
        print!("{: >29}{}", "", old_cpu)
    }
}
