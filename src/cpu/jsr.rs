// jump to sub routine
use super::Cpu;
use super::OpCode;

pub struct Jsr {
    low: u8,
    high: u8,
    state: usize,
}

impl OpCode for Jsr {
    fn new() -> Jsr {
        Jsr {
            low: 0,
            high: 0,
            state: 0,
        }
    }

    fn decode(&mut self, cpu: &mut Cpu) -> bool {
        if self.state == 0 {
            // 2    PC     R  fetch low address byte, increment PC
            self.low = cpu.read_from_pc();
            self.state = 1;
            false
        } else if self.state == 1 {
            // 3  $0100,S  R  internal operation (predecrement S?)
            self.state = 2;
            false
        } else if self.state == 2 {
            // 4  $0100,S  W  push PCH on stack, decrement S
            let sp: u16 = mk_addr!(cpu.sp, 0x01);
            let val = (cpu.pc >> 8) & 0xFF;
            cpu.mem.set(sp, val as u8);
            let (sp, _) = cpu.sp.overflowing_sub(1);
            cpu.sp = sp;
            self.state = 3;
            false
        } else if self.state == 3 {
            // 5  $0100,S  W  push PCL on stack, decrement S
            let sp: u16 = mk_addr!(cpu.sp, 0x01);
            let val = cpu.pc & 0xFF;
            cpu.mem.set(sp, val as u8);
            let (sp, _) = cpu.sp.overflowing_sub(1);
            cpu.sp = sp;
            self.state = 4;
            false
        } else {
            // 6    PC     R  copy low address byte to PCL, fetch high address byte to PCH
            self.high = cpu.read_from_pc();
            let addr = mk_addr!(self.low, self.high);
            cpu.pc = addr;
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
            "{:04X}  {:02X} {:02X} {:02X}  JSR ${:04X}",
            pc, code, low, high, addr
        );
        print!("{: >23}{}", "", cpu)
    }
}
