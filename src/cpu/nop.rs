use super::cpu::Cpu;
use super::opcode::OpCode;

const SIZE: u16 = 1;

pub struct Nop {}

impl OpCode for Nop {
    fn new() -> Nop {
        Nop {}
    }
    fn decode(&mut self, cpu: &mut Cpu) -> bool {
        let _ = cpu.mem.get(cpu.pc);
        true
    }
    fn log(&self, cpu: &Cpu) {
        let pc = cpu.pc - SIZE;
        let code = cpu.mem.get(pc);
        print!("{:04X}  {:02X}        NOP", pc, code);
        print!("{: >29}{}", "", cpu)
    }
}
