use super::cpu::Cpu;

pub trait OpCode {
    // returns true when decode is done
    fn decode(&mut self, cpu: &mut Cpu) -> bool;
    // initialize the opcode structure
    fn new() -> Self
    where
        Self: Sized;
    // log info
    fn log(&self, cpu: &Cpu) {
        print!("{:04X}", cpu.pc)
    }
}
