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
