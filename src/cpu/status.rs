pub struct Status {
    pub carry: bool,
    pub zero: bool,
    pub interrupt: bool,
    pub decimal: bool,
    pub break_: bool,
    pub overflow: bool,
    pub negative: bool,
}

pub enum StatusFlag {
    Carry,
    Zero,
    Interrupt,
    Decimal,
    Break,
    Overflow,
    Negative,
}

use StatusFlag::*;

impl Status {
    pub fn new() -> Self {
        Status {
            carry: false,
            zero: false,
            interrupt: false,
            decimal: false,
            break_: false,
            overflow: false,
            negative: false,
        }
    }

    pub fn set(&mut self, flag: StatusFlag, val: bool) {
        match flag {
            StatusFlag::Carry => self.carry = val,
            StatusFlag::Zero => self.zero = val,
            StatusFlag::Interrupt => self.interrupt = val,
            StatusFlag::Decimal => self.decimal = val,
            StatusFlag::Break => self.break_ = val,
            StatusFlag::Overflow => self.overflow = val,
            StatusFlag::Negative => self.negative = val,
        }
    }
}

impl From<u8> for Status {
    fn from(status: u8) -> Self {
        let mut res = Status::new();
        if (status & (1 << 0)) != 0 {
            res.set(Carry, true)
        }
        if (status & (1 << 1)) != 0 {
            res.set(Zero, true)
        }
        if (status & (1 << 2)) != 0 {
            res.set(Interrupt, true)
        }
        if (status & (1 << 3)) != 0 {
            res.set(Decimal, true)
        }
        if (status & (1 << 4)) != 0 {
            res.set(Break, true)
        }
        if (status & (1 << 6)) != 0 {
            res.set(Overflow, true)
        }
        if (status & (1 << 7)) != 0 {
            res.set(Negative, true)
        }
        res
    }
}
