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

macro_rules! decode_load_imm {
    ($opcode:ident, $cpu:ident) =>
    {{
        $opcode.imm = $cpu.read_from_pc();
        true
    }};
}

macro_rules! execute_imm {
    ($reg:ident, $opcode:ident, $cpu:ident) =>
    {{
        let imm = $opcode.imm;
        $cpu.$reg = imm;
        $cpu.flags.zero = imm == 0;
        $cpu.flags.negative = (imm & (0x80 as u8)) != 0 
    }};
}

macro_rules! declare_load_imm {
    ($name:ident, $reg:ident) => {
        pub struct $name {
            imm: u8,
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name { imm: 0 }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                decode_load_imm!(self, cpu)
            }

            fn execute(&self, cpu: &mut Cpu) {
                execute_imm!($reg, self, cpu)
            }
        }
    }
}

declare_load_imm!(LDAImm, a);
declare_load_imm!(LDXImm, x);
declare_load_imm!(LDYImm, y);

macro_rules! decode_load_zero_page {
    ($opcode:ident, $cpu:ident) =>
    {{
        if $opcode.state == 0 {
            // read offset from memory
            $opcode.addr = $cpu.read_from_pc();
            $opcode.state = 1;
            false
        } else {
            // read data from memory using offset in page 0
            $opcode.imm = $cpu.mem[$opcode.addr as usize];
            true
        }
    }};
}

macro_rules! declare_load_zero_page {
    ($name:ident, $reg:ident) => {
        pub struct $name {
            addr: u8,
            imm: u8,
            state: usize
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    addr: 0,
                    imm: 0,
                    state: 0
                }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                decode_load_zero_page!(self, cpu)
            }

            fn execute(&self, cpu: &mut Cpu) {
                execute_imm!($reg, self, cpu)
            }
        }
    }
}

declare_load_zero_page!(LDAZeroPage, a);
declare_load_zero_page!(LDXZeroPage, x);
declare_load_zero_page!(LDYZeroPage, y);

macro_rules! decode_load_zero_page_reg {
    ($opcode:ident, $cpu:ident, $base:ident) =>
    {{
        if $opcode.state == 0 {
            // read offset from memory
            $opcode.addr = $cpu.read_from_pc();
            $opcode.state = 1;
            false
        } else if $opcode.state == 1 {
            // compute final offset. Wrapping on page 0
            $opcode.addr = $opcode.addr.overflowing_add($cpu.$base).0;
            $opcode.state = 2;
            false
        } else {
            // read data from memory using offset
            $opcode.imm = $cpu.mem[$opcode.addr as usize];
            true
        }
    }};
}

macro_rules! declare_load_zero_page_reg {
    ($name:ident, $reg:ident, $base:ident) => {
        pub struct $name {
            addr: u8,
            imm: u8,
            state: usize
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    addr: 0,
                    imm: 0,
                    state: 0
                }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                decode_load_zero_page_reg!(self, cpu, $base)
            }

            fn execute(&self, cpu: &mut Cpu) {
                execute_imm!($reg, self, cpu)
            }
        }
    }
}

declare_load_zero_page_reg!(LDAZeroPageX, a, x);
declare_load_zero_page_reg!(LDYZeroPageX, y, x);
declare_load_zero_page_reg!(LDXZeroPageY, x, y);

macro_rules! decode_load_abs {
    ($opcode:ident, $cpu:ident) =>
    {{
        if $opcode.state == 0 {
            $opcode.low = $cpu.read_from_pc();
            $opcode.state = 1;
            false
        } else if $opcode.state == 1 {
            $opcode.high = $cpu.read_from_pc();
            $opcode.state = 2;
            false
        } else {
            let addr : u16 = (($opcode.high as u16) << 8) | ($opcode.low as u16);
            $opcode.imm = $cpu.mem[addr as usize];
            true
        }
    }};
}

macro_rules! declare_load_abs {
    ($name:ident, $reg:ident) => {
        pub struct $name {
            low: u8,
            high: u8,
            imm: u8,
            state: usize
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    low: 0,
                    high: 0,
                    imm: 0,
                    state: 0
                }
            }

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                decode_load_abs!(self, cpu)
            }

            fn execute(&self, cpu: &mut Cpu) {
                execute_imm!($reg, self, cpu)
            }
        }
    }
}

declare_load_abs!(LDAAbs, a);
declare_load_abs!(LDXAbs, x);
declare_load_abs!(LDYAbs, y);