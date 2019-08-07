use super::cpu::Cpu;

pub trait OpCode {
    // returns true when decode is done
    fn decode(&mut self, cpu: &mut Cpu) -> bool;
    // apply effects to the CPU
    fn execute(&self, cpu: &mut Cpu);
    // initialize the opcode structure
    fn new() -> Self where Self:Sized;
}
