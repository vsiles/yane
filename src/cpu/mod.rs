use std::collections::HashMap;

mod status;
use status::StatusFlag::*;
use status::*;

mod opcodes;

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xfd;
const BRK_IRQ_BASE: u16 = 0xFFFE;

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

    #[allow(dead_code)]
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

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.a &= value;
        self.update_zero_and_negative(self.a);
    }

    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.a ^= value;
        self.update_zero_and_negative(self.a);
    }

    fn ora(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.a |= value;
        self.update_zero_and_negative(self.a);
    }

    fn asl_a(&mut self) {
        let mut data = self.a;
        let bit7 = (data >> 7) & 0x1;
        self.ps.set(Carry, bit7 != 0);
        data <<= 1;
        self.a = data;
        self.update_zero_and_negative(self.a);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        let bit7 = (data >> 7) & 0x1;
        self.ps.set(Carry, bit7 != 0);
        data <<= 1;
        self.mem_write(addr, data);
        self.update_zero_and_negative(data);
    }

    fn lsr_a(&mut self) {
        let mut data = self.a;
        let bit0 = data & 0x1;
        self.ps.set(Carry, bit0 != 0);
        data >>= 1;
        self.a = data;
        self.update_zero_and_negative(self.a);
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        let bit0 = data & 0x1;
        self.ps.set(Carry, bit0 != 0);
        data >>= 1;
        self.mem_write(addr, data);
        self.update_zero_and_negative(data);
    }

    fn rol_a(&mut self) {
        let mut data = self.a;
        let bit7 = (data >> 7) & 0x1;
        let carry = if self.ps.carry { 1 } else { 0 };
        self.ps.set(Carry, bit7 != 0);
        data = (data << 1) | (carry as u8);
        self.a = data;
        self.update_zero_and_negative(self.a);
    }

    fn rol(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        let bit7 = (data >> 7) & 0x1;
        let carry = if self.ps.carry { 1 } else { 0 };
        self.ps.set(Carry, bit7 != 0);
        data = (data << 1) | (carry as u8);
        self.mem_write(addr, data);
        self.update_zero_and_negative(data);
    }

    fn ror_a(&mut self) {
        let mut data = self.a;
        let bit0 = data & 0x1;
        let carry = if self.ps.carry { 1 } else { 0 };
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
        let carry = if self.ps.carry { 1 } else { 0 };
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
        let sum = self.a as u16 + data as u16 + (if self.ps.carry { 1 } else { 0 }) as u16;

        let carry = sum > 0xff;
        self.ps.set(Carry, carry);

        let result = sum as u8;

        let overflow: bool = (data ^ result) & (result ^ self.a) & 0x80 != 0;
        self.ps.set(Overflow, overflow);

        self.a = result;
        self.update_zero_and_negative(self.a);
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        // A - B = A + (-B) and -B = !B + 1
        self.add_to_a(((data as i8).wrapping_neg().wrapping_sub(1)) as u8);
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.add_to_a(value);
    }

    // Stack ops
    fn stack_pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.mem_read((STACK as u16) + self.sp as u16)
    }

    fn stack_push(&mut self, data: u8) {
        self.mem_write((STACK as u16) + self.sp as u16, data);
        self.sp = self.sp.wrapping_sub(1)
    }

    fn stack_push_u16(&mut self, data: u16) {
        let high = ((data >> 8) & 0xFF) as u8;
        let low = (data & 0xff) as u8;
        self.stack_push(high);
        self.stack_push(low);
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let low = self.stack_pop() as u16;
        let high = self.stack_pop() as u16;

        high << 8 | low
    }

    fn pha(&mut self) {
        self.stack_push(self.a)
    }

    fn php(&mut self) {
        let sps = &self.ps;
        //http://wiki.nesdev.com/w/index.php/CPU_status_flag_behavior
        let mut ps: u8 = sps.into();
        ps |= (0x3 << 4);
        self.stack_push(ps)
    }

    fn pla(&mut self) {
        self.a = self.stack_pop();
        self.update_zero_and_negative(self.a);
    }

    fn plp(&mut self) {
        let data = self.stack_pop();
        self.ps = data.into();
        self.ps.break0 = false;
        self.ps.break1 = true;
    }

    // Jump/Branching
    fn branch(&mut self, condition: bool) {
        if condition {
            let jump: i8 = self.mem_read(self.pc) as i8;
            let jump_addr = self.pc.wrapping_add(1).wrapping_add(jump as u16);
            self.pc = jump_addr;
        }
    }

    fn jmp_abs(&mut self) {
        let addr = self.mem_read_u16(self.pc);
        self.pc = addr;
    }

    fn jmp_indirect(&mut self) {
        let addr = self.mem_read_u16(self.pc);
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#JMP
        // let indirect_ref = self.mem_read_u16(addr);

        let indirect_ref = if addr & 0x00FF == 0x00FF {
            let low = self.mem_read(addr);
            let high = self.mem_read(addr & 0xFF00);
            (high as u16) << 8 | (low as u16)
        } else {
            self.mem_read_u16(addr)
        };

        self.pc = indirect_ref;
    }

    fn jsr(&mut self) {
        self.stack_push_u16(self.pc + 2 - 1); // push return addr - 1
        let addr = self.mem_read_u16(self.pc);
        self.pc = addr;
    }

    fn rti(&mut self) {
        self.ps = self.stack_pop().into();
        self.ps.set(Break0, false);
        self.ps.set(Break1, true);

        self.pc = self.stack_pop_u16();
    }

    fn rts(&mut self) {
        self.pc = self.stack_pop_u16() + 1;
    }

    #[allow(dead_code)]
    fn brk(&mut self) {
        // #  address R/W description
        // --- ------- --- -----------------------------------------------
        //  1    PC     R  fetch opcode, increment PC
        //  2    PC     R  read next instruction byte (and throw it away),
        //                 increment PC
        //  3  $0100,S  W  push PCH on stack, decrement S
        //  4  $0100,S  W  push PCL on stack, decrement S
        // *** At this point, the signal status determines which interrupt vector is used ***
        //  5  $0100,S  W  push P on stack (with B flag set), decrement S
        //  6   $FFFE   R  fetch PCL, set I flag
        //  7   $FFFF   R  fetch PCH
        //
        // 1 is done by the `run` method.
        // 2 we ignore
        self.stack_push_u16(self.pc);
        let mut ps = self.ps.clone();
        ps.break0 = true;
        let data: u8 = (&ps).into();
        self.stack_push(data);
        self.pc = self.mem_read_u16(BRK_IRQ_BASE)
    }

    fn cmp(&mut self, mode: &AddressingMode, reference: u8) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        if reference >= data {
            self.ps.set(Carry, true)
        }
        self.update_zero_and_negative(reference.wrapping_sub(data))
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        data = data.wrapping_sub(1);
        self.update_zero_and_negative(data);
        self.mem_write(addr, data);
    }

    fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.update_zero_and_negative(self.x);
    }

    fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.update_zero_and_negative(self.y);
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        data = data.wrapping_add(1);
        self.update_zero_and_negative(data);
        self.mem_write(addr, data);
    }

    fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.update_zero_and_negative(self.x);
    }

    fn iny(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.update_zero_and_negative(self.y);
    }

    #[allow(dead_code)]
    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut Cpu),
    {
        let opcodes: &HashMap<u8, &'static opcodes::OpCode> = &(*opcodes::OPCODES_MAP);
        loop {
            callback(self);
            let code = self.mem_read(self.pc);
            self.pc += 1;

            let pc = self.pc;

            let opcode = opcodes
                .get(&code)
                .unwrap_or_else(|| panic!("OpCode {:x} is not supported", code));

            match code {
                // BRK : TODO
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
                // NOP
                0xEA => self.nop(),
                // ADC
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => self.adc(&opcode.mode),
                // SBC
                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => self.sbc(&opcode.mode),
                // PHA
                0x48 => self.pha(),
                // PHP
                0x08 => self.php(),
                // PLA
                0x68 => self.pla(),
                // PLP
                0x28 => self.plp(),
                // BCC
                0x90 => self.branch(!self.ps.carry),
                // BCS
                0xB0 => self.branch(self.ps.carry),
                // BEQ
                0xF0 => self.branch(self.ps.zero),
                // BMI
                0x30 => self.branch(self.ps.negative),
                // BNE
                0xD0 => self.branch(!self.ps.zero),
                // BPL
                0x10 => self.branch(!self.ps.negative),
                // BVC
                0x50 => self.branch(!self.ps.overflow),
                // BVS
                0x70 => self.branch(self.ps.overflow),
                // CMP
                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                    self.cmp(&opcode.mode, self.a)
                }
                // CPX
                0xE0 | 0xE4 | 0xEC => self.cmp(&opcode.mode, self.x),
                // CPY
                0xC0 | 0xC4 | 0xCC => self.cmp(&opcode.mode, self.y),
                // DEC
                0xC6 | 0xD6 | 0xCE | 0xDE => self.dec(&opcode.mode),
                // DEX
                0xCA => self.dex(),
                // DEY
                0x88 => self.dey(),
                // INC
                0xE6 | 0xF6 | 0xEE | 0xFE => self.inc(&opcode.mode),
                // INX
                0xE8 => self.inx(),
                // INY
                0xC8 => self.iny(),
                // JMP
                0x4C => self.jmp_abs(),
                0x6C => self.jmp_indirect(),
                // JSR
                0x20 => self.jsr(),
                // RTI
                0x40 => self.rti(),
                // RTS
                0x60 => self.rts(),
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
