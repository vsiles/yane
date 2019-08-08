use super::Cpu;
use super::OpCode;

const SIZE : usize = 3;

pub struct Jmp {
    low: u8,
    high: u8,
    state: usize,
    old: u16
}

impl OpCode for Jmp {
    fn new() -> Jmp {
        Jmp {
            low: 0,
            high: 0,
            state: 0,
            old: 0
        }
    }

    fn decode(&mut self, cpu: &mut Cpu) -> bool {
        if self.state == 0 {
            self.low = cpu.read_from_pc();
            self.state = 1;
            false
        } else {
            self.high = cpu.read_from_pc();
            self.old = cpu.pc;
            cpu.pc = mk_addr!(self.low, self.high);
            true
        }
    }

    fn log(&self, cpu: &Cpu) {
        let pc = self.old;
        let code = cpu.mem[pc as usize - SIZE];
        let addr = mk_addr!(self.low, self.high);
        print!("{:04X}  {:02X} {:02X} {:02X}  JMP ${:04X}", pc, code, 
               self.low, self.high, addr);
        println!("{: >23}{}", "", cpu)
    } 
}
