#[macro_use]
mod opcode;

use std::fmt;

#[derive(Clone)]
pub struct CpuFlags {
    pub carry: bool,
    pub zero: bool,
    pub int_disable: bool,
    pub decimal_mode: bool,
    pub b4: bool,
    pub b5: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl CpuFlags {
    pub fn new() -> CpuFlags {
        CpuFlags {
            carry: false,
            zero: false,
            int_disable: true,
            decimal_mode: false,
            // b4: true, // nestest says this one is false at startup
            b4: false,
            b5: true,
            overflow: false,
            negative: false,
        }
    }

    pub fn update(&mut self, val: u8) {
        self.carry = (val & 0x01) != 0;
        self.zero = (val & 0x02) != 0;
        self.int_disable = (val & 0x04) != 0;
        self.decimal_mode = (val & 0x08) != 0;
        // see https://wiki.nesdev.com/w/index.php/Status_flags
        // for why we ignore bit 4 and 5
        self.overflow = (val & 0x40) != 0;
        self.negative = (val & 0x80) != 0;
    }

    pub fn to_p(&self) -> u8 {
        let mut ret: u8 = 0;
        if self.carry {
            ret = ret | (1 << 0)
        }
        if self.zero {
            ret = ret | (1 << 1)
        }
        if self.int_disable {
            ret = ret | (1 << 2)
        }
        if self.decimal_mode {
            ret = ret | (1 << 3)
        }
        if self.b4 {
            ret = ret | (1 << 4)
        }
        if self.b5 {
            ret = ret | (1 << 5)
        }
        if self.overflow {
            ret = ret | (1 << 6)
        }
        if self.negative {
            ret = ret | (1 << 7)
        }
        ret
    }
}

impl fmt::Display for CpuFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(C{} Z{} I{} D{} B{}{} O{} N{})",
            if self.carry { 1 } else { 0 },
            if self.zero { 1 } else { 0 },
            if self.int_disable { 1 } else { 0 },
            if self.decimal_mode { 1 } else { 0 },
            if self.b4 { 1 } else { 0 },
            if self.b5 { 1 } else { 0 },
            if self.overflow { 1 } else { 0 },
            if self.negative { 1 } else { 0 }
        )
    }
}
