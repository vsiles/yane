pub mod cpu;
#[macro_use]
pub mod flags;
pub mod nop;
pub mod opcode;
#[macro_use]
mod macros;
#[macro_use]
pub mod load;
#[macro_use]
pub mod store;
pub mod jmp;
pub mod jsr;
pub mod rts;
#[macro_use]
pub mod branch;
#[macro_use]
pub mod cmp;
pub mod adc;
pub mod sbc;
#[macro_use]
pub mod incr;
#[macro_use]
pub mod decr;
#[macro_use]
pub mod trs;
pub mod rti;
#[macro_use]
pub mod bin_ndx_ind;
#[macro_use]
pub mod addr_mod_imm;
#[macro_use]
pub mod addr_mod_zp;
#[macro_use]
pub mod addr_mod_abs;

pub use cpu::*;
pub use opcode::OpCode;

// ADC
// https://stackoverflow.com/questions/29193303/6502-emulation-proper-way-to-implement-adc-and-sbc
fn adc_core(cpu: &mut Cpu, imm: usize) {
    let a: usize = cpu.A as usize;
    let val: usize = a + imm + (if cpu.flags.carry { 1 } else { 0 });
    cpu.flags.overflow = ((a ^ val) & (imm ^ val) & 0x80) == 0x80;
    cpu.flags.carry = (val & 0x100) == 0x100;
    cpu.flags.zero = (val & 0xFF) == 0;
    cpu.flags.negative = (val & 0x80) == 0x80;
    cpu.A = (val & 0xFF) as u8
}

fn adc_imm(cpu: &mut Cpu, val: usize) {
    adc_core(cpu, val)
}

fn adc_addr(cpu: &mut Cpu, val: usize) {
    let addr = (val & 0xFFFF) as u16;
    let imm = cpu.mem.get(addr) as usize;
    adc_core(cpu, imm)
}

declare_addr_imm!(adc_imm, AdcImm, ADC, super::adc_imm);
declare_addr_zero_page!(adc_zp, AdcZp, ADC, super::adc_addr);
declare_addr_abs!(adc_abs, AdcAbs, ADC, super::adc_addr);

// SBC
// https://stackoverflow.com/questions/29193303/6502-emulation-proper-way-to-implement-adc-and-sbc
// TODO: Could be implemented using ADC(~input) as ~input is -input -1
fn sbc_core(cpu: &mut Cpu, imm: u8) {
    let a: u8 = cpu.A;
    let (val0, _) = a.overflowing_sub(imm);
    let (val, _) = val0.overflowing_sub(if cpu.flags.carry { 0 } else { 1 });
    let res = (a as i32) - (imm as i32) - (if cpu.flags.carry { 0 } else { 1 });
    cpu.flags.overflow = ((a ^ val) & (a ^ imm) & 0x80) == 0x80;
    cpu.flags.carry = res >= 0;
    cpu.flags.zero = val == 0;
    cpu.flags.negative = (val & 0x80) == 0x80;
    cpu.A = val
}

fn sbc_imm(cpu: &mut Cpu, val: usize) {
    let imm = (val & 0xFF) as u8;
    sbc_core(cpu, imm)
}

fn sbc_addr(cpu: &mut Cpu, val: usize) {
    let addr = (val & 0xFFFF) as u16;
    let imm = cpu.mem.get(addr);
    sbc_core(cpu, imm)
}

declare_addr_imm!(sbc_imm, SbcImm, SBC, super::sbc_imm);
declare_addr_zero_page!(sbc_zp, SbcZp, SBC, super::sbc_addr);
declare_addr_abs!(sbc_abs, SbcAbs, SBC, super::sbc_addr);

// ORA, AND, EOR
macro_rules! bool_imm_impl {
    ($name:ident, $op:expr) => {
        fn $name(cpu: &mut Cpu, val: usize) { 
            let imm = $op((val & 0xFF) as u8, cpu.A);
            execute_load!(A, imm, cpu);
        }
    };
}

macro_rules! bool_addr_impl {
    ($name:ident, $op:expr) => {
        fn $name(cpu: &mut Cpu, val: usize) { 
            let addr = (val & 0xFFFF) as u16;
            let imm = $op(cpu.mem.get(addr), cpu.A);
            execute_load!(A, imm, cpu)
        }
    };
}

bool_imm_impl!(ora_imm, |x, y| { x | y });
bool_imm_impl!(and_imm, |x, y| { x & y });
bool_imm_impl!(eor_imm, |x, y| { x ^ y });
bool_addr_impl!(ora_addr, |x, y| { x | y });
bool_addr_impl!(and_addr, |x, y| { x & y });
bool_addr_impl!(eor_addr, |x, y| { x ^ y });

declare_addr_imm!(ora_imm, OraImm, ORA, super::ora_imm);
declare_addr_imm!(and_imm, AndImm, AND, super::and_imm);
declare_addr_imm!(eor_imm, EorImm, EOR, super::eor_imm);

declare_bin_ndx_ind!(ora_ndx_ind, OraNdxInd, A, ORA, |x, y| { x | y });
declare_bin_ndx_ind!(and_ndx_ind, AndNdxInd, A, AND, |x, y| { x & y });
declare_bin_ndx_ind!(eor_ndx_ind, EorNdxInd, A, EOR, |x, y| { x ^ y });

declare_addr_zero_page!(ora_zp, OraZp, ORA, super::ora_addr);
declare_addr_zero_page!(and_zp, AndZp, AND, super::and_addr);
declare_addr_zero_page!(eor_zp, EorZp, EOR, super::eor_addr);

declare_addr_abs!(ora_abs, OraAbs, ORA, super::ora_addr);
declare_addr_abs!(and_abs, AndAbs, AND, super::and_addr);
declare_addr_abs!(eor_abs, EorAbs, EOR, super::eor_addr);

// TAX, TAY, TSX, TXA, TXS, TYA
declare_transfert!(tax, TAX, A, X);
declare_transfert!(tay, TAY, A, Y);
declare_transfert!(txa, TXA, X, A);
declare_transfert!(tya, TYA, Y, A);

pub mod tsx {
    use super::Cpu;
    use super::OpCode;

    pub struct TSX {}

    impl OpCode for TSX {
        fn new() -> TSX {
            TSX {}
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            let imm = cpu.sp;
            execute_load!(X, imm, cpu);
            true
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            print!("{:04X}  {:02X}        TSX", pc, code);
            print!("{: <29}{}", "", cpu)
        }
    }
}

pub mod txs {
    use super::Cpu;
    use super::OpCode;

    pub struct TXS {}

    impl OpCode for TXS {
        fn new() -> TXS {
            TXS {}
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            cpu.sp = cpu.X;
            true
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            print!("{:04X}  {:02X}        TXS", pc, code);
            print!("{: <29}{}", "", cpu)
        }
    }
}

// INX, INY, DEX, DEY
declare_incr!(inx, InX, X);
declare_incr!(iny, InY, Y);
declare_decr!(dex, DeX, X);
declare_decr!(dey, DeY, Y);

// CMP, CMX, CMY
macro_rules! cmp_core {
    ($name:ident, $reg:ident) => {
        fn $name(cpu: &mut Cpu, imm: u8) {
            let (res, _) = cpu.$reg.overflowing_sub(imm);
            cpu.flags.carry = cpu.$reg >= imm;
            cpu.flags.zero = cpu.$reg == imm;
            cpu.flags.negative = (res & 0x80) != 0;
        }
    };
}
cmp_core!(cmp_a, A);
cmp_core!(cmp_x, X);
cmp_core!(cmp_y, Y);

macro_rules! cmp_imm_impl {
    ($name:ident, $core:ident) => {
        fn $name(cpu: &mut Cpu, val: usize) {
            let imm = (val & 0xFF) as u8;
            $core(cpu, imm)
        }
    };
}

cmp_imm_impl!(cmp_imm_a, cmp_a);
cmp_imm_impl!(cmp_imm_x, cmp_x);
cmp_imm_impl!(cmp_imm_y, cmp_y);

declare_addr_imm!(cmp_imm, CmpImm, CMP, super::cmp_imm_a);
declare_addr_imm!(cpx_imm, CpxImm, CPX, super::cmp_imm_x);
declare_addr_imm!(cpy_imm, CpyImm, CPY, super::cmp_imm_y);

macro_rules! cmp_addr_impl {
    ($name:ident, $core:ident) => {
        fn $name(cpu: &mut Cpu, val: usize) {
            let addr = (val & 0xFFFF) as u16;
            let imm = cpu.mem.get(addr);
            $core(cpu, imm)
        }
    };
}

cmp_addr_impl!(cmp_addr_a, cmp_a);
cmp_addr_impl!(cmp_addr_x, cmp_x);
cmp_addr_impl!(cmp_addr_y, cmp_y);

declare_addr_zero_page!(cmp_zp, CmpZp, CMP, super::cmp_addr_a);
declare_addr_zero_page!(cpx_zp, CpxZp, CPX, super::cmp_addr_x);
declare_addr_zero_page!(cpy_zp, CpyZp, CPY, super::cmp_addr_y);

// LDA, LDX, LDY
macro_rules! load_imm_impl {
    ($name:ident, $reg:ident) => {
        fn $name(cpu: &mut Cpu, val: usize) { 
            let imm = (val & 0xFF) as u8;
            execute_load!($reg, imm, cpu);
        }
    };
}

macro_rules! load_addr_impl {
    ($name:ident, $reg:ident) => {
        fn $name(cpu: &mut Cpu, val: usize) { 
            let addr = (val & 0xFFFF) as u16;
            let imm = cpu.mem.get(addr);
            execute_load!($reg, imm, cpu)
        }
    };
}

load_imm_impl!(load_imm_a, A);
load_imm_impl!(load_imm_x, X);
load_imm_impl!(load_imm_y, Y);
load_addr_impl!(load_addr_a, A);
load_addr_impl!(load_addr_x, X);
load_addr_impl!(load_addr_y, Y);

declare_addr_imm!(lda_imm, LdaImm, LDA, super::load_imm_a);
declare_addr_imm!(ldx_imm, LdxImm, LDX, super::load_imm_x);
declare_addr_imm!(ldy_imm, LdyImm, LDY, super::load_imm_y);

declare_addr_zero_page!(lda_zero_page, LdaZeroPage, LDA, super::load_addr_a);
declare_addr_zero_page!(ldx_zero_page, LdxZeroPage, LDX, super::load_addr_x);
declare_addr_zero_page!(ldy_zero_page, LdyZeroPage, LDY, super::load_addr_y);

declare_load_zero_page_reg!(lda_zero_page_x, LdaZeroPageX, A, X);
declare_load_zero_page_reg!(ldy_zero_page_x, LdyZeroPageX, Y, X);
declare_load_zero_page_reg!(ldx_zero_page_y, LdxZeroPageY, X, Y);

declare_load_abs!(lda_abs, LdaAbs, A);
declare_load_abs!(ldx_abs, LdxAbs, X);
declare_load_abs!(ldy_abs, LdyAbs, Y);

declare_load_abs_reg!(lda_abs_x, LdaAbsX, A, X);
declare_load_abs_reg!(lda_abs_y, LdaAbsY, A, Y);
declare_load_abs_reg!(ldx_abs_y, LdxAbsY, X, Y);
declare_load_abs_reg!(ldy_abs_x, LdyAbsX, Y, X);

declare_load_ind_ndx!(lda_ind_ndx, LdaIndNdx, A);

// STA, STX, STY
macro_rules! store_impl {
    ($name:ident, $reg:ident) => {
        fn $name(cpu: &mut Cpu, val: usize) { 
            let addr = (val & 0xFFFF) as u16;
            cpu.mem.set(addr, cpu.$reg)
        }
    };
}
store_impl!(store_a, A);
store_impl!(store_x, X);
store_impl!(store_y, Y);

declare_addr_zero_page!(sta_zero_page, StaZeroPage, STA, super::store_a);
declare_addr_zero_page!(stx_zero_page, StxZeroPage, STX, super::store_x);
declare_addr_zero_page!(sty_zero_page, StyZeroPage, STY, super::store_y);

declare_store_zero_page_reg!(sta_zero_page_x, StaZeroPageX, A, X);
declare_store_zero_page_reg!(stx_zero_page_y, StxZeroPageY, X, Y);
declare_store_zero_page_reg!(sty_zero_page_x, StyZeroPageX, Y, X);

declare_store_abs!(sta_abs, StaAbs, A);
declare_store_abs!(stx_abs, StxAbs, X);
declare_store_abs!(sty_abs, StyAbs, Y);

declare_store_abs_reg!(sta_abs_x, StaAbsX, A, X);
declare_store_abs_reg!(sta_abs_y, StaAbsY, A, Y);

declare_store_ndx_ind!(sta_ndx_ind, StaNdxInd, A);

declare_store_ind_ndx!(sta_ind_ndx, StaIndNdx, A);

// SEC, SED, SEI, CLC, CLD, CLI
declare_flags_opcode!(sec, Sec, SEC, carry, true);
declare_flags_opcode!(sed, Sed, SED, decimal_mode, true);
declare_flags_opcode!(sei, Sei, SEI, int_disable, true);
declare_flags_opcode!(clc, Clc, CLC, carry, false);
declare_flags_opcode!(cld, Cld, CLD, decimal_mode, false);
declare_flags_opcode!(cli, Cli, CLI, int_disable, false);
declare_flags_opcode!(clv, Clv, CLV, overflow, false);

// Branching
declare_branch!(bcs, Bcs, carry, true, BCS);
declare_branch!(bcc, Bcc, carry, false, BCC);
declare_branch!(beq, Beq, zero, true, BEQ);
declare_branch!(bne, Bne, zero, false, BNE);
declare_branch!(bvs, Bvs, overflow, true, BVS);
declare_branch!(bvc, Bvc, overflow, false, BVC);
declare_branch!(bpl, Bpl, negative, false, BPL);
declare_branch!(bmi, Bmi, negative, true, BMI);

// BIT
pub mod bit_abs {
    use super::super::Cpu;
    use super::super::OpCode;

    pub struct BitAbs {
        low: u8,
        high: u8,
        imm: u8,
        state: usize,
    }

    impl OpCode for BitAbs {
        fn new() -> BitAbs {
            BitAbs {
                low: 0,
                high: 0,
                imm: 0,
                state: 0,
            }
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            if self.state == 0 {
                self.low = cpu.read_from_pc();
                self.state = 1;
                false
            } else if self.state == 1 {
                self.high = cpu.read_from_pc();
                self.state = 2;
                false
            } else {
                let addr: u16 = mk_addr!(self.low, self.high);
                self.imm = cpu.mem.get(addr);
                let val = cpu.A & self.imm;
                cpu.flags.zero = val == 0;
                cpu.flags.overflow = (self.imm & 0x40) != 0;
                cpu.flags.negative = (self.imm & 0x80) != 0;
                true
            }
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            let low = cpu.mem.get(pc + 1);
            let high = cpu.mem.get(pc + 2);
            let addr = mk_addr!(low, high);
            let imm = cpu.mem.get(addr);
            print!(
                "{:04X}  {:02X} {:02X} {:02X}  BIT ${:04X}",
                pc, code, low, high, addr
            );
            print!(" = {:02X} {: >17}{}", imm, "", cpu);
        }
    }
}

fn bit(cpu: &mut Cpu, val: usize) {
    let addr = (val & 0xFFFF) as u16;
    let imm = cpu.mem.get(addr);
    let fval = cpu.A & imm;
    cpu.flags.zero = fval == 0;
    cpu.flags.overflow = (imm & 0x40) != 0;
    cpu.flags.negative = (imm & 0x80) != 0
}

declare_addr_zero_page!(bit_zp, BitZp, BIT, super::bit);

pub mod php {
    use super::super::Cpu;
    use super::super::OpCode;

    pub struct Php {
        state: usize,
    }

    impl OpCode for Php {
        fn new() -> Php {
            Php { state: 0 }
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            if self.state == 0 {
                let _ = cpu.mem.get(cpu.pc);
                self.state = 1;
                false
            } else {
                // see https://wiki.nesdev.com/w/index.php/Status_flags
                // with PHP, bit 4 and 5 are always set to one
                let val = cpu.flags.to_p() | 0x30;
                push!(cpu, val);
                true
            }
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            print!("{:04X}  {:02X}        PHP", pc, code);
            print!("{: >29}{}", "", cpu)
        }
    }
}

pub mod pha {
    use super::super::Cpu;
    use super::super::OpCode;

    pub struct Pha {
        state: usize,
    }

    impl OpCode for Pha {
        fn new() -> Pha {
            Pha { state: 0 }
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            if self.state == 0 {
                let _ = cpu.mem.get(cpu.pc);
                self.state = 1;
                false
            } else {
                push!(cpu, cpu.A);
                true
            }
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            print!("{:04X}  {:02X}        PHA", pc, code);
            print!("{: >29}{}", "", cpu)
        }
    }
}

pub mod pla {
    use super::super::Cpu;
    use super::super::OpCode;

    pub struct Pla {
        state: usize,
    }

    impl OpCode for Pla {
        fn new() -> Pla {
            Pla { state: 0 }
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            if self.state == 0 {
                let _ = cpu.mem.get(cpu.pc);
                self.state = 1;
                false
            } else if self.state == 1 {
                let (sp, _) = cpu.sp.overflowing_add(1);
                cpu.sp = sp;
                self.state = 2;
                false
            } else {
                let sp = mk_addr!(cpu.sp, 0x01 as usize);
                // see https://wiki.nesdev.com/w/index.php/Status_flags
                // with PHP, bit 4 and 5 are always set to one
                cpu.A = cpu.mem.get(sp);
                cpu.flags.zero = cpu.A == 0;
                cpu.flags.negative = (cpu.A & 0x80) != 0;
                true
            }
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            print!("{:04X}  {:02X}        PLA", pc, code);
            print!("{: >29}{}", "", cpu)
        }
    }
}

pub mod plp {
    use super::super::Cpu;
    use super::super::OpCode;

    pub struct Plp {
        state: usize,
    }

    impl OpCode for Plp {
        fn new() -> Plp {
            Plp { state: 0 }
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            if self.state == 0 {
                let _ = cpu.mem.get(cpu.pc);
                self.state = 1;
                false
            } else if self.state == 1 {
                let (sp, _) = cpu.sp.overflowing_add(1);
                cpu.sp = sp;
                self.state = 2;
                false
            } else {
                let sp = mk_addr!(cpu.sp, 0x01 as usize);
                // see https://wiki.nesdev.com/w/index.php/Status_flags
                let imm = cpu.mem.get(sp);
                cpu.flags.update(imm);
                true
            }
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            print!("{:04X}  {:02X}        PLP", pc, code);
            print!("{: >29}{}", "", cpu)
        }
    }
}

pub mod lsr_a {
    use super::Cpu;
    use super::OpCode;

    pub struct LsrA {}

    impl OpCode for LsrA {
        fn new() -> LsrA {
            LsrA {}
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            cpu.flags.carry = (cpu.A & 0x01) != 0;
            let imm = cpu.A >> 1;
            execute_load!(A, imm, cpu);
            true
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            print!("{:04X}  {:02X}        LSR A", pc, code);
            print!("{: <27}{}", "", cpu);
        }
    }
}

fn lsr_core(cpu: &mut Cpu, data: u8) -> u8 {
    cpu.flags.carry = (data & 0x01) != 0;
    let res = data >> 1;
    cpu.flags.zero = res == 0;
    cpu.flags.negative = (res & (0x80 as u8)) != 0;
    res
}

declare_addr_zero_page2!(lsr_zp, LsrZp, LSR, super::lsr_core);

pub mod asl_a {
    use super::Cpu;
    use super::OpCode;

    pub struct AslA {}

    impl OpCode for AslA {
        fn new() -> AslA {
            AslA {}
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            cpu.flags.carry = (cpu.A & 0x80) != 0;
            let imm = cpu.A << 1;
            execute_load!(A, imm, cpu);
            true
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            print!("{:04X}  {:02X}        ASL A", pc, code,);
            print!("{: <27}{}", "", cpu);
        }
    }
}

fn asl_core(cpu: &mut Cpu, data: u8) -> u8 {
    cpu.flags.carry = (data & 0x80) != 0;
    let res = data << 1;
    cpu.flags.zero = res == 0;
    cpu.flags.negative = (res & (0x80 as u8)) != 0;
    res
}

declare_addr_zero_page2!(asl_zp, AslZp, ASL, super::asl_core);

pub mod ror_a {
    use super::Cpu;
    use super::OpCode;

    pub struct RorA {}

    impl OpCode for RorA {
        fn new() -> RorA {
            RorA {}
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            let imm: u8 = (cpu.A >> 1) | (if cpu.flags.carry { 0x80 } else { 0 });
            cpu.flags.carry = (cpu.A & 0x01) != 0;
            execute_load!(A, imm, cpu);
            true
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            print!("{:04X}  {:02X}        ROR A", pc, code,);
            print!("{: <27}{}", "", cpu);
        }
    }
}

fn ror_core(cpu: &mut Cpu, data: u8) -> u8 {
    let res: u8 = (data >> 1) | (if cpu.flags.carry { 0x80 } else { 0 });
    cpu.flags.carry = (data & 0x01) != 0;
    cpu.flags.zero = res == 0;
    cpu.flags.negative = (res & (0x80 as u8)) != 0;
    res
}

declare_addr_zero_page2!(ror_zp, RorZp, ROR, super::ror_core);

pub mod rol_a {
    use super::Cpu;
    use super::OpCode;

    pub struct RolA {}

    impl OpCode for RolA {
        fn new() -> RolA {
            RolA {}
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            let imm = (cpu.A << 1) | (if cpu.flags.carry { 0x01 } else { 0 });
            cpu.flags.carry = (cpu.A & 0x80) != 0;
            execute_load!(A, imm, cpu);
            true
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            print!("{:04X}  {:02X}        ROL A", pc, code,);
            print!("{: <27}{}", "", cpu);
        }
    }
}

fn rol_core(cpu: &mut Cpu, data: u8) -> u8 {
    let res: u8 = (data << 1) | (if cpu.flags.carry { 0x01 } else { 0 });
    cpu.flags.carry = (data & 0x80) != 0;
    cpu.flags.zero = res == 0;
    cpu.flags.negative = (res & (0x80 as u8)) != 0;
    res
}

declare_addr_zero_page2!(rol_zp, RolZp, ROL, super::rol_core);

fn inc_core(cpu: &mut Cpu, data: u8) -> u8 {
    let res: u8 = data.overflowing_add(1).0;
    cpu.flags.zero = res == 0;
    cpu.flags.negative = (res & (0x80 as u8)) != 0;
    res
}

declare_addr_zero_page2!(inc_zp, IncZp, INC, super::inc_core);

fn dec_core(cpu: &mut Cpu, data: u8) -> u8 {
    let res: u8 = data.overflowing_sub(1).0;
    cpu.flags.zero = res == 0;
    cpu.flags.negative = (res & (0x80 as u8)) != 0;
    res
}

declare_addr_zero_page2!(dec_zp, DecZp, DEC, super::dec_core);
