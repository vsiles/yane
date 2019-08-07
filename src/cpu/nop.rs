use super::opcode::OpCode;
use super::cpu::Cpu;

pub struct Nop {
}

impl OpCode for Nop {
    fn new(_: usize) -> Nop {
        Nop {}
    }
    fn decode(&mut self, _: &mut Cpu) -> bool {
        true
    }
    fn log(&self, _: &Cpu) {}
}
