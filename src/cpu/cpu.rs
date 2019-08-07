use super::flags::CpuFlags;
use std::fmt;

#[allow(non_snake_case)]
pub struct Cpu {
    pub pc: u16,
    pub sp: u8,
    pub A: u8,
    pub X: u8,
    pub Y: u8,
    pub flags: CpuFlags,
    pub mem: std::vec::Vec<u8>,
}

pub fn new(mem: std::vec::Vec<u8>) -> Cpu {
    Cpu {
        pc: 0,
        sp: 0,
        A: 0,
        X: 0,
        Y: 0,
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

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            self.A, self.X, self.Y, self.flags.to_p(), self.sp)
    }
}