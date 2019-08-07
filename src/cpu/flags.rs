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
    pub fn new() -> CpuFlags {
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
