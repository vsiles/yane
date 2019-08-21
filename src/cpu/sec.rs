use super::cpu::Cpu;
use super::opcode::OpCode;

const SIZE: usize = 1;

pub struct Sec {}

impl OpCode for Sec {
    fn new() -> Sec {
        Sec {}
    }
    fn decode(&mut self, cpu: &mut Cpu) -> bool {
        let _ = cpu.mem[cpu.pc as usize];
        cpu.flags.carry = true;
        true
    }
    fn log(&self, cpu: &Cpu) {
        let pc: usize = (cpu.pc as usize) - SIZE;
        let code = cpu.mem[pc];
        print!("{:04X}  {:02X}        SEC", pc, code);
        print!("{: >29}{}", "", cpu)
    }
}
