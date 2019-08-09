use super::opcode::OpCode;
use super::cpu::Cpu;

const SIZE: usize = 1;

pub struct Nop {
}

impl OpCode for Nop {
    fn new() -> Nop {
        Nop { }
    }
    fn decode(&mut self, cpu: &mut Cpu) -> bool {
        let _ = cpu.mem[cpu.pc as usize];
        true
    }
    fn log(&self, cpu: &Cpu) {
        let pc : usize = (cpu.pc as usize) - SIZE;
        let code = cpu.mem[pc];
        print!("{:04X}  {:02X}        NOP", pc, code);
        println!("{: >29}{}", "", cpu)
    }
}
