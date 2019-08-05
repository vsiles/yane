use std::fmt;

pub struct CpuFlags {
    pub carry: bool,
    pub zero: bool,
    pub int_disable: bool,
    pub decimal_mode: bool,
    pub brk: bool,
    pub overflow: bool,
    pub negative: bool
}

impl CpuFlags {
    fn new() -> CpuFlags {
        CpuFlags {
            carry: false,
            zero: false,
            int_disable: false,
            decimal_mode: false,
            brk: false,
            overflow: false,
            negative: false
        }
    }
}

impl fmt::Display for CpuFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(C{} Z{} I{} D{} B{} O{} N{})",
            if self.carry { 1 } else { 0 },
            if self.zero { 1 } else { 0 },
            if self.int_disable { 1 } else { 0 },
            if self.decimal_mode { 1 } else { 0 },
            if self.brk { 1 } else { 0 },
            if self.overflow { 1 } else { 0 },
            if self.negative { 1 } else { 0 })
    }
}

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

pub trait OpCode {
    // returns true when decode is done
    fn decode(&mut self, cpu: &mut Cpu) -> bool;
    // apply effects to the CPU
    fn execute(&self, cpu: &mut Cpu);
    // initialize the opcode structure
    fn new() -> Self where Self:Sized;
}

// NOP
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

pub struct LDAImm {
    imm: u8,
}

impl OpCode for LDAImm {
    fn new() -> LDAImm {
        LDAImm { imm: 0 }
    }

    fn decode(&mut self, cpu: &mut Cpu) -> bool {
        self.imm = cpu.read_from_pc();
        true
    }

    fn execute(&self, cpu: &mut Cpu) {
        cpu.a = self.imm;
        cpu.flags.zero = cpu.a == 0;
        cpu.flags.negative = (cpu.a & 0x80) != 0;
    }
}

pub struct LDAZeroPage {
    addr: u8,
    imm: u8,
    state: usize,
}

impl OpCode for LDAZeroPage {
    fn new() -> LDAZeroPage {
        LDAZeroPage {
            addr: 0,
            imm: 0,
            state: 0
        }
    }

    fn decode(&mut self, cpu: &mut Cpu) -> bool {
        if self.state == 0 {
            // read offset from memory
            self.addr = cpu.read_from_pc();
            self.state = self.state + 1;
            false
        } else {
            // read data from memory using offset in page 0
            self.imm = cpu.mem[self.addr as usize];
            true
        }
    }

    fn execute(&self, cpu: &mut Cpu) {
        cpu.a = self.imm;
        cpu.flags.zero = cpu.a == 0;
        cpu.flags.negative = (cpu.a & 0x80) != 0;
    }
}

pub struct LDAZeroPageX {
    addr: u8,
    imm: u8,
    state: usize,
}

impl OpCode for LDAZeroPageX {
    fn new() -> LDAZeroPageX {
        LDAZeroPage {
            addr: 0,
            imm: 0,
            state: 0
        }
    }

    fn decode(&mut self, cpu: &mut Cpu) -> bool {
        if self.state == 0 {
            // read offset from memory
            self.addr = cpu.read_from_pc();
            self.state = self.state + 1;
            false
        } else {
            // read data from memory using offset in page 0
            self.imm = cpu.mem[self.addr as usize];
            true
        }
    }

    fn execute(&self, cpu: &mut Cpu) {
        cpu.a = self.imm;
        cpu.flags.zero = cpu.a == 0;
        cpu.flags.negative = (cpu.a & 0x80) != 0;
    }
}