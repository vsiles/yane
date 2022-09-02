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
    memory: [u8; 0xFFFF],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            x: 0,
            ps: Status::new(),
            pc: 0,
            memory: [0; 0xFFFF],
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let low = self.mem_read(pos) as u16;
        // TODO: check if this +1 can oveflow
        let high = self.mem_read(pos + 1) as u16;
        (high << 8) | (low as u16)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let high = (data >> 8) as u8;
        let low = (data & 0xff) as u8;
        self.mem_write(pos, low);
        // TODO: check if this +1 can oveflow
        self.mem_write(pos + 1, high);
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.ps = Status::from(0);

        self.pc = self.mem_read_u16(0xFFFC);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run()
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000)
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

    pub fn run(&mut self) {
        loop {
            let opcode = self.mem_read(self.pc);
            self.pc += 1;

            match opcode {
                // BRK
                0x00 => return,
                // LDA
                0xA9 => {
                    // 2 Cycles
                    let a = self.mem_read(self.pc);
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
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.a, 0x05);
        assert!(cpu.ps.zero == false);
        assert!(cpu.ps.negative == false);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert_eq!(cpu.a, 0x00);
        assert!(cpu.ps.zero == true);
        assert!(cpu.ps.negative == false);
    }

    #[test]
    fn test_0xa9_lda_neg_flag() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(vec![0xa9, 0xf0, 0x00]);
        assert_eq!(cpu.a, 0xf0);
        assert!(cpu.ps.zero == false);
        assert!(cpu.ps.negative == true);
    }

    #[test]
    fn test_0xaa_tax() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(vec![0xa9, 0x0b, 0xaa, 0x00]);
        assert_eq!(cpu.a, 0x0b);
        assert_eq!(cpu.x, 0x0b);
        assert!(cpu.ps.zero == false);
        assert!(cpu.ps.negative == false);
    }

    #[test]
    fn test_0xaa_tax_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.x = 0x0b;
        cpu.load_and_run(vec![0xaa, 0x00]);
        assert_eq!(cpu.x, 0x00);
        assert!(cpu.ps.zero == true);
        assert!(cpu.ps.negative == false);
    }

    #[test]
    fn test_0xaa_tax_negative_flag() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(vec![0xa9, 0xf0, 0xaa, 0x00]);
        assert_eq!(cpu.x, 0xf0);
        assert!(cpu.ps.zero == false);
        assert!(cpu.ps.negative == true);
    }

    #[test]
    fn test_0xe8_inx() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(vec![0xa9, 0x0a, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.x, 0x0b);
        assert!(cpu.ps.zero == false);
        assert!(cpu.ps.negative == false);
    }

    #[test]
    fn test_0xe8_inx_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.x, 0x00);
        assert!(cpu.ps.zero == true);
        assert!(cpu.ps.negative == false);
    }

    #[test]
    fn test_0xe8_inx_negative_flag() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(vec![0xa9, 0xf0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.x, 0xf1);
        assert!(cpu.ps.zero == false);
        assert!(cpu.ps.negative == true);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.x, 1)
    }
}
