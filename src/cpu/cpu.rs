use super::flags::CpuFlags;

pub struct Cpu {
    pub pc: u16,
    pub sp: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub flags: CpuFlags,
    pub mem: std::vec::Vec<u8>,
}

pub fn new(mem: std::vec::Vec<u8>) -> Cpu {
    Cpu {
        pc: 0,
        sp: 0,
        a: 0,
        x: 0,
        y: 0,
        mem: mem,
        flags: CpuFlags::new()
    }
}

impl Cpu {
    pub fn read_from_pc(&mut self) -> u8 {
        let op = self.mem[self.pc as usize];
        self.pc = self.pc + 1;
        op
    }
}

