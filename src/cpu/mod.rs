use std::collections::HashMap;

mod status;
use status::StatusFlag::*;
use status::*;

mod opcodes;

pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub ps: Status,
    pub pc: u16,
    memory: [u8; 0xFFFF],
}

#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    NoneAddressing,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            ps: Status::new(),
            pc: 0,
            memory: [0; 0xFFFF],
        }
    }

    // Memory related actions
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let low = self.mem_read(pos) as u16;
        let high = self.mem_read(pos.wrapping_add(1)) as u16;
        (high << 8) | (low as u16)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let high = (data >> 8) as u8;
        let low = (data & 0xff) as u8;
        self.mem_write(pos, low);
        self.mem_write(pos.wrapping_add(1), high);
    }

    // Global actions & entry points
    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
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

    // Addressing Modes
    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.pc,
            AddressingMode::ZeroPage => self.mem_read(self.pc) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.pc),
            AddressingMode::ZeroPageX => {
                let pos = self.mem_read(self.pc);
                let addr = pos.wrapping_add(self.x) as u16;
                addr
            }
            AddressingMode::ZeroPageY => {
                let pos = self.mem_read(self.pc);
                let addr = pos.wrapping_add(self.y) as u16;
                addr
            }
            AddressingMode::AbsoluteX => {
                let base = self.mem_read_u16(self.pc);
                let addr = base.wrapping_add(self.x as u16);
                addr
            }
            AddressingMode::AbsoluteY => {
                let base = self.mem_read_u16(self.pc);
                let addr = base.wrapping_add(self.y as u16);
                addr
            }
            AddressingMode::IndirectX => {
                let base = self.mem_read(self.pc);

                let ptr: u8 = (base as u8).wrapping_add(self.x);
                let low = self.mem_read(ptr as u16);
                let high = self.mem_read(ptr.wrapping_add(1) as u16);
                (high as u16) << 8 | (low as u16)
            }
            AddressingMode::IndirectY => {
                let base = self.mem_read(self.pc);

                let low = self.mem_read(base as u16);
                let high = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (high as u16) << 8 | (low as u16);
                let deref = deref_base.wrapping_add(self.y as u16);
                deref
            }
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    // Instructions
    fn update_zero_and_negative(&mut self, result: u8) {
        self.ps.set(Zero, result == 0);
        self.ps.set(Negative, (result & (1 << 7)) != 0)
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.a = value;
        self.update_zero_and_negative(self.a);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.a);
    }

    fn tax(&mut self) {
        self.x = self.a;
        self.update_zero_and_negative(self.x);
    }

    fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.update_zero_and_negative(self.x);
    }

    pub fn run(&mut self) {
        let ref opcodes: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;
        loop {
            let code = self.mem_read(self.pc);
            self.pc += 1;

            let pc = self.pc;

            let opcode = opcodes
                .get(&code)
                .expect(&format!("OpCode {:x} is not supported", code));

            match code {
                // BRK
                0x00 => return,
                // LDA
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&opcode.mode);
                }
                // STA
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(&opcode.mode);
                }
                // TAX : 2 Cycles
                0xAA => self.tax(),
                // INX : 2 Cycles
                0xE8 => self.inx(),
                _ => todo!(),
            }

            if pc == self.pc {
                self.pc += (opcode.len - 1) as u16;
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
    fn test_lda_from_memory() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
        assert_eq!(cpu.a, 0x55);
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
