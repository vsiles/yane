use super::opcode::OpCode;
use super::cpu::Cpu;

pub struct Nop {
}

impl OpCode for Nop {
    fn new() -> Nop {
        Nop {}
    }
    fn decode(&mut self, _: &mut Cpu) -> bool {
        true
    }
    fn execute(&self, _: &mut Cpu) {}
}
