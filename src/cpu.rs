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
    fn new() -> Self {
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

    fn set(&mut self, flag: StatusFlag, val: bool) {
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

pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub ps: Status,
    pub pc: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            x: 0,
            ps: Status::new(),
            pc: 0,
        }
    }

    fn update_zero_and_negative(&mut self, result: u8) {
        self.ps.set(Zero, result == 0);
        self.ps.set(Negative, (result & (1 << 7)) != 0)
    }

    fn lda(&mut self, value: u8) {
        // Cycles 2
        self.a = value;
        self.update_zero_and_negative(self.a);
    }

    fn tax(&mut self) {
        self.x = self.a;
        self.update_zero_and_negative(self.x);
    }

    fn inx(&mut self) {
        let (x, _) = self.x.overflowing_add(1);
        self.x = x;
        self.update_zero_and_negative(self.x);
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.pc = 0;

        loop {
            let opcode = program[self.pc as usize];
            self.pc += 1;

            match opcode {
                // BRK
                0x00 => return,
                // LDA
                0xA9 => {
                    // 2 Cycles
                    let a = program[self.pc as usize];
                    self.pc += 1;
                    self.lda(a);
                }
                // TAX : 2 Cycles
                0xAA => self.tax(),
                // INX : 2 Cycles
                0xE8 => self.inx(),
                _ => todo!(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_imm() {
        let mut cpu = Cpu::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.a, 0x05);
        assert!(cpu.ps.zero == false);
        assert!(cpu.ps.negative == false);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert_eq!(cpu.a, 0x00);
        assert!(cpu.ps.zero == true);
        assert!(cpu.ps.negative == false);
    }

    #[test]
    fn test_0xa9_lda_neg_flag() {
        let mut cpu = Cpu::new();
        cpu.interpret(vec![0xa9, 0xf0, 0x00]);
        assert_eq!(cpu.a, 0xf0);
        assert!(cpu.ps.zero == false);
        assert!(cpu.ps.negative == true);
    }

    #[test]
    fn test_0xaa_tax() {
        let mut cpu = Cpu::new();
        cpu.a = 0x0b;
        cpu.interpret(vec![0xaa, 0x00]);
        assert_eq!(cpu.a, 0x0b);
        assert_eq!(cpu.x, 0x0b);
        assert!(cpu.ps.zero == false);
        assert!(cpu.ps.negative == false);
    }

    #[test]
    fn test_0xaa_tax_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.x = 0x0b;
        cpu.interpret(vec![0xaa, 0x00]);
        assert_eq!(cpu.x, 0x00);
        assert!(cpu.ps.zero == true);
        assert!(cpu.ps.negative == false);
    }

    #[test]
    fn test_0xaa_tax_negative_flag() {
        let mut cpu = Cpu::new();
        cpu.x = 0x0b;
        cpu.a = 0xf0;
        cpu.interpret(vec![0xaa, 0x00]);
        assert_eq!(cpu.x, 0xf0);
        assert!(cpu.ps.zero == false);
        assert!(cpu.ps.negative == true);
    }

    #[test]
    fn test_0xe8_inx() {
        let mut cpu = Cpu::new();
        cpu.x = 0x0a;
        cpu.interpret(vec![0xe8, 0x00]);
        assert_eq!(cpu.x, 0x0b);
        assert!(cpu.ps.zero == false);
        assert!(cpu.ps.negative == false);
    }

    #[test]
    fn test_0xe8_inx_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.x = 0xff;
        cpu.interpret(vec![0xe8, 0x00]);
        assert_eq!(cpu.x, 0x00);
        assert!(cpu.ps.zero == true);
        assert!(cpu.ps.negative == false);
    }

    #[test]
    fn test_0xe8_inx_negative_flag() {
        let mut cpu = Cpu::new();
        cpu.x = 0xf0;
        cpu.interpret(vec![0xe8, 0x00]);
        assert_eq!(cpu.x, 0xf1);
        assert!(cpu.ps.zero == false);
        assert!(cpu.ps.negative == true);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = Cpu::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = Cpu::new();
        cpu.x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.x, 1)
    }
}
