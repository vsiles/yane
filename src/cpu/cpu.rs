use super::flags::CpuFlags;
use std::fmt;
use super::super::memory::Memory;

#[allow(non_snake_case)]
pub struct Cpu {
    pub pc: u16,
    pub sp: u8,
    pub A: u8,
    pub X: u8,
    pub Y: u8,
    pub flags: CpuFlags,
    pub mem: Memory,
}

pub fn new(mem: Memory) -> Cpu {
    let cpu = Cpu {
        pc: 0xFFFC, // reset vector
        sp: 0xFD,
        A: 0,
        X: 0,
        Y: 0,
        mem: mem,
        flags: CpuFlags::new(),
    };
    println!("DEBUG: 0xbffc = {:#x}", cpu.mem.get(0xbffc));
    println!("DEBUG: 0xbffd = {:#x}", cpu.mem.get(0xbffd));
    println!("DEBUG: 0xbffe = {:#x}", cpu.mem.get(0xbffe));
    println!("DEBUG: 0xfffc = {:#x}", cpu.mem.get(0xfffc));
    println!("DEBUG: 0xfffd = {:#x}", cpu.mem.get(0xfffd));
    println!("DEBUG: 0xfffe = {:#x}", cpu.mem.get(0xfffe));
    // TODO: 
    // cpu.mem[0x4017] = 0x00; // frame irq enabled
    // cpu.mem[0x4015] = 0x00; // all channels disabled
    // for i in 0..0x14 {
    //     cpu.mem[0x4000 + i] = 0x00
    // }
    cpu
}

impl Cpu {
    pub fn read_from_pc(&mut self) -> u8 {
        let op = self.mem.get(self.pc);
        self.pc = self.pc + 1;
        op
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            self.A,
            self.X,
            self.Y,
            self.flags.to_p(),
            self.sp
        )
    }
}
