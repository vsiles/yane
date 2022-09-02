use std::collections::HashMap;

mod status;
use status::StatusFlag::*;
use status::*;

mod opcodes;

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xfd;

pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub ps: Status,
    pub pc: u16,
    pub sp: u8,
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
            sp: STACK_RESET,
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

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.x = value;
        self.update_zero_and_negative(self.x);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.y = value;
        self.update_zero_and_negative(self.y);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.a);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.x);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.y);
    }

    fn tax(&mut self) {
        self.x = self.a;
        self.update_zero_and_negative(self.x);
    }

    fn tay(&mut self) {
        self.y = self.a;
        self.update_zero_and_negative(self.y);
    }

    fn tsx(&mut self) {
        self.x = self.sp;
        self.update_zero_and_negative(self.x);
    }

    fn txa(&mut self) {
        self.a = self.x;
        self.update_zero_and_negative(self.a);
    }

    fn txs(&mut self) {
        self.sp = self.x;
    }

    fn tya(&mut self) {
        self.a = self.y;
        self.update_zero_and_negative(self.a);
    }

    fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.update_zero_and_negative(self.x);
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.a = self.a & value;
        self.update_zero_and_negative(self.a);
    }

    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.a = self.a ^ value;
        self.update_zero_and_negative(self.a);
    }
    
    fn ora(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.a = self.a | value;
        self.update_zero_and_negative(self.a);
    }

    fn asl_a(&mut self) {
        let mut data = self.a;
        let bit7 = (data >> 7) & 0x1;
        self.ps.set(Carry, bit7 != 0);
        data = data << 1;
        self.a = data;
        self.update_zero_and_negative(self.a);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        let bit7 = (data >> 7) & 0x1;
        self.ps.set(Carry, bit7 != 0);
        data = data << 1;
        self.mem_write(addr, data);
        self.update_zero_and_negative(data);
    }

    fn lsr_a(&mut self) {
        let mut data = self.a;
        let bit0 = data & 0x1;
        self.ps.set(Carry, bit0 != 0);
        data = data >> 1;
        self.a = data;
        self.update_zero_and_negative(self.a);
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        let bit0 = data & 0x1;
        self.ps.set(Carry, bit0 != 0);
        data = data >> 1;
        self.mem_write(addr, data);
        self.update_zero_and_negative(data);
    }

    fn rol_a(&mut self) {
        let mut data = self.a;
        let bit7 = (data >> 7) & 0x1;
        let carry  = if self.ps.carry { 1 } else { 0 };
        self.ps.set(Carry, bit7 != 0);
        data = (data << 1) | (carry as u8);
        self.a = data;
        self.update_zero_and_negative(self.a);
    }

    fn rol(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        let bit7 = (data >> 7) & 0x1;
        let carry  = if self.ps.carry { 1 } else { 0 };
        self.ps.set(Carry, bit7 != 0);
        data = (data << 1) | (carry as u8);
        self.mem_write(addr, data);
        self.update_zero_and_negative(data);
    }

    fn ror_a(&mut self) {
        let mut data = self.a;
        let bit0 = data & 0x1;
        let carry  = if self.ps.carry { 1 } else { 0 };
        let carry = (carry as u8) << 7;
        self.ps.set(Carry, bit0 != 0);
        data = (data >> 1) | carry;
        self.a = data;
        self.update_zero_and_negative(self.a);
    }

    fn ror(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        let bit0 = data & 0x1;
        let carry  = if self.ps.carry { 1 } else { 0 };
        let carry = (carry as u8) << 7;
        self.ps.set(Carry, bit0 != 0);
        data = (data >> 1) | carry;
        self.mem_write(addr, data);
        self.update_zero_and_negative(data);
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        let bit6 = (data >> 6) & 0x1;
        let bit7 = (data >> 7) & 0x1;

        let res = self.a & data;
        self.ps.set(Zero, res == 0);
        self.ps.set(Overflow, bit6 != 0);
        self.ps.set(Negative, bit7 != 0);
    }

    fn set_flag(&mut self, flag: StatusFlag, state: bool) {
        self.ps.set(flag, state)
    }

    fn nop(&self) {}

    // Add to A with Carry
    // Ignoring decimal mode
    /// http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
    fn add_to_a(&mut self, data: u8) {
        let sum = self.a as u16
            + data as u16
            + (if self.ps.carry {
                1
            } else {
                0
            }) as u16;

        let carry = sum > 0xff;
        self.ps.set(Carry, carry);

        let result = sum as u8;

        let overflow : bool = (data ^ result) & (result ^ self.a) & 0x80 != 0;
        self.ps.set(Overflow, overflow);

        self.a = result;
        self.update_zero_and_negative(self.a);
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let data = self.mem_read(addr);
        // A - B = A + (-B) and -B = !B + 1
        self.add_to_a(((data as i8).wrapping_neg().wrapping_sub(1)) as u8);
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.add_to_a(value);
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
                // LDX
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(&opcode.mode),
                // LDY
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(&opcode.mode),
                // STA
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(&opcode.mode);
                }
                // STX
                0x86 | 0x96 | 0x8E => self.stx(&opcode.mode),
                // STY
                0x84 | 0x94 | 0x8C => self.sty(&opcode.mode),
                // AND
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    self.and(&opcode.mode);
                }
                // EOR
                0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
                    self.eor(&opcode.mode);
                }
                // ORA
                0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                    self.ora(&opcode.mode);
                }
                // ASL
                0x0A => self.asl_a(),
                0x06 | 0x16 | 0x0E | 0x1E => self.asl(&opcode.mode),
                // ROL
                0x2A => self.rol_a(),
                0x26 | 0x36 | 0x2E | 0x3E => self.rol(&opcode.mode),
                // LSR
                0x4A => self.lsr_a(),
                0x46 | 0x56 | 0x4E | 0x5E => self.lsr(&opcode.mode),
                // ROR
                0x6A => self.ror_a(),
                0x66 | 0x76 | 0x6E | 0x7E => self.ror(&opcode.mode),
                // BIT
                0x24 | 0x2C => self.bit(&opcode.mode),
                // CLC
                0x18 => self.set_flag(Carry, false), 
                // CLD
                0xD8 => self.set_flag(Decimal, false), 
                // CLI
                0x58 => self.set_flag(Interrupt, false), 
                // CLV
                0xB8 => self.set_flag(Overflow, false), 
                // SEC
                0x38 => self.set_flag(Carry, true),
                // SED
                0xF8 => self.set_flag(Decimal, true),
                // SEI
                0x78 => self.set_flag(Interrupt, true),
                // TAX
                0xAA => self.tax(),
                // TAY
                0xA8 => self.tay(),
                // TSX
                0xBA => self.tsx(),
                // TXA
                0x8A => self.txa(),
                // TXS
                0x9A => self.txs(),
                // TYA
                0x98 => self.tya(),
                // INX
                0xE8 => self.inx(),
                // NOP
                0xEA => self.nop(),
                // ADC
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    self.adc(&opcode.mode)
                }
                // SBC
                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                    self.sbc(&opcode.mode)
                }
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
