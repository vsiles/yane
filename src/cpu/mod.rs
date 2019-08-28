pub mod cpu;
#[macro_use]
pub mod flags;
pub mod opcode;
#[macro_use]
mod macros;
pub mod jmp;
pub mod jsr;
pub mod rts;
#[macro_use]
pub mod branch;
#[macro_use]
pub mod incr;
#[macro_use]
pub mod decr;
#[macro_use]
pub mod trs;
pub mod rti;
#[macro_use]
pub mod addr_mod_nop;
#[macro_use]
pub mod addr_mod_imm;
#[macro_use]
pub mod addr_mod_zp;
#[macro_use]
pub mod addr_mod_zp_reg;
#[macro_use]
pub mod addr_mod_abs;
#[macro_use]
pub mod addr_mod_abs_reg;
#[macro_use]
pub mod addr_mod_indirect_y;
#[macro_use]
pub mod addr_mod_indirect_x;

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

declare_addr_imm!(AdcImm, ADC, adc_imm);
declare_addr_zero_page!(AdcZp, ADC, adc_addr);
declare_addr_abs!(AdcAbs, ADC, adc_addr);
declare_addr_ind_x!(AdcIndX, ADC, adc_addr);
declare_addr_ind_y!(AdcIndY, ADC, adc_addr);
declare_addr_abs_reg!(AdcAbsX, ADC, X, adc_addr);
declare_addr_abs_reg!(AdcAbsY, ADC, Y, adc_addr);
declare_addr_zero_page_reg!(AdcZpX, ADC, X, adc_addr);

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

declare_addr_imm!(SbcImm, SBC, sbc_imm);
declare_addr_zero_page!(SbcZp, SBC, sbc_addr);
declare_addr_abs!(SbcAbs, SBC, sbc_addr);
declare_addr_ind_x!(SbcIndX, SBC, sbc_addr);
declare_addr_ind_y!(SbcIndY, SBC, sbc_addr);
declare_addr_abs_reg!(SbcAbsX, SBC, X, sbc_addr);
declare_addr_abs_reg!(SbcAbsY, SBC, Y, sbc_addr);
declare_addr_zero_page_reg!(SbcZpX, SBC, X, sbc_addr);

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

declare_addr_imm!(OraImm, ORA, ora_imm);
declare_addr_imm!(AndImm, AND, and_imm);
declare_addr_imm!(EorImm, EOR, eor_imm);

declare_addr_zero_page!(OraZp, ORA, ora_addr);
declare_addr_zero_page!(AndZp, AND, and_addr);
declare_addr_zero_page!(EorZp, EOR, eor_addr);

declare_addr_abs!(OraAbs, ORA, ora_addr);
declare_addr_abs!(AndAbs, AND, and_addr);
declare_addr_abs!(EorAbs, EOR, eor_addr);

declare_addr_ind_x!(OraIndX, ORA, ora_addr);
declare_addr_ind_x!(AndIndX, AND, and_addr);
declare_addr_ind_x!(EorIndX, EOR, eor_addr);

declare_addr_ind_y!(OraIndY, ORA, ora_addr);
declare_addr_ind_y!(AndIndY, AND, and_addr);
declare_addr_ind_y!(EorIndY, EOR, eor_addr);

declare_addr_abs_reg!(OraAbsX, ORA, X, ora_addr);
declare_addr_abs_reg!(OraAbsY, ORA, Y, ora_addr);
declare_addr_abs_reg!(AndAbsX, AND, X, and_addr);
declare_addr_abs_reg!(AndAbsY, AND, Y, and_addr);
declare_addr_abs_reg!(EorAbsX, EOR, X, eor_addr);
declare_addr_abs_reg!(EorAbsY, EOR, Y, eor_addr);

declare_addr_zero_page_reg!(OraZpX, ORA, X, ora_addr);
declare_addr_zero_page_reg!(AndZpX, AND, X, and_addr);
declare_addr_zero_page_reg!(EorZpX, EOR, X, eor_addr);

// TAX, TAY, TSX, TXA, TXS, TYA
declare_transfert!(TAX, A, X);
declare_transfert!(TAY, A, Y);
declare_transfert!(TXA, X, A);
declare_transfert!(TYA, Y, A);

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

// INX, INY, DEX, DEY
declare_incr!(InX, X);
declare_incr!(InY, Y);
declare_decr!(DeX, X);
declare_decr!(DeY, Y);

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

declare_addr_imm!(CmpImm, CMP, cmp_imm_a);
declare_addr_imm!(CpxImm, CPX, cmp_imm_x);
declare_addr_imm!(CpyImm, CPY, cmp_imm_y);

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

declare_addr_zero_page!(CmpZp, CMP, cmp_addr_a);
declare_addr_zero_page!(CpxZp, CPX, cmp_addr_x);
declare_addr_zero_page!(CpyZp, CPY, cmp_addr_y);

declare_addr_abs!(CmpAbs, CMP, cmp_addr_a);
declare_addr_abs!(CpxAbs, CPX, cmp_addr_x);
declare_addr_abs!(CpyAbs, CPY, cmp_addr_y);

declare_addr_ind_x!(CmpIndX, CMP, cmp_addr_a);
declare_addr_ind_y!(CmpIndY, CMP, cmp_addr_a);

declare_addr_abs_reg!(CmpAbsX, CMP, X, cmp_addr_a);
declare_addr_abs_reg!(CmpAbsY, CMP, Y, cmp_addr_a);

declare_addr_zero_page_reg!(CmpZpX, CMP, X, cmp_addr_a);

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

declare_addr_imm!(LdaImm, LDA, load_imm_a);
declare_addr_imm!(LdxImm, LDX, load_imm_x);
declare_addr_imm!(LdyImm, LDY, load_imm_y);

declare_addr_zero_page!(LdaZeroPage, LDA, load_addr_a);
declare_addr_zero_page!(LdxZeroPage, LDX, load_addr_x);
declare_addr_zero_page!(LdyZeroPage, LDY, load_addr_y);

declare_addr_zero_page_reg!(LdaZeroPageX, LDA, X, load_addr_a);
declare_addr_zero_page_reg!(LdyZeroPageX, LDY, X, load_addr_y);
declare_addr_zero_page_reg!(LdxZeroPageY, LDX, Y, load_addr_x);

declare_addr_abs!(LdaAbs, LDA, load_addr_a);
declare_addr_abs!(LdxAbs, LDX, load_addr_x);
declare_addr_abs!(LdyAbs, LDY, load_addr_y);

declare_addr_abs_reg!(LdaAbsX, LDA, X, load_addr_a);
declare_addr_abs_reg!(LdaAbsY, LDA, Y, load_addr_a);
declare_addr_abs_reg!(LdxAbsY, LDX, Y, load_addr_x);
declare_addr_abs_reg!(LdyAbsX, LDY, X, load_addr_y);

declare_addr_ind_x!(LdaIndX, LDA, load_addr_a);
declare_addr_ind_y!(LdaIndY, LDA, load_addr_a);

// STA, STX, STY
macro_rules! store_impl {
    ($name:ident, $reg:ident) => {
        fn $name(cpu: &mut Cpu, val: usize) { 
            // println!("Storing register {} = {:#X} at {:#X}",
            //          stringify!($reg), cpu.$reg, val);
            let addr = (val & 0xFFFF) as u16;
            cpu.mem.set(addr, cpu.$reg)
        }
    };
}
store_impl!(store_a, A);
store_impl!(store_x, X);
store_impl!(store_y, Y);

declare_addr_zero_page!(StaZeroPage, STA, store_a);
declare_addr_zero_page!(StxZeroPage, STX, store_x);
declare_addr_zero_page!(StyZeroPage, STY, store_y);

declare_addr_zero_page_reg!(StaZeroPageX, STA, X, store_a);
declare_addr_zero_page_reg!(StxZeroPageY, STX, Y, store_x);
declare_addr_zero_page_reg!(StyZeroPageX, STY, X, store_y);

declare_addr_abs!(StaAbs, STA, store_a);
declare_addr_abs!(StxAbs, STX, store_x);
declare_addr_abs!(StyAbs, STY, store_y);

declare_addr_abs_reg!(StaAbsX, STA, X, store_a);
declare_addr_abs_reg!(StaAbsY, STA, Y, store_a);

declare_addr_ind_x!(StaIndX, STA, store_a);
declare_addr_ind_y!(StaIndY, STA, store_a);

// SEC, SED, SEI, CLC, CLD, CLI
declare_flags_opcode!(Sec, SEC, carry, true);
declare_flags_opcode!(Sed, SED, decimal_mode, true);
declare_flags_opcode!(Sei, SEI, int_disable, true);
declare_flags_opcode!(Clc, CLC, carry, false);
declare_flags_opcode!(Cld, CLD, decimal_mode, false);
declare_flags_opcode!(Cli, CLI, int_disable, false);
declare_flags_opcode!(Clv, CLV, overflow, false);

// Branching
declare_branch!(Bcs, carry, true, BCS);
declare_branch!(Bcc, carry, false, BCC);
declare_branch!(Beq, zero, true, BEQ);
declare_branch!(Bne, zero, false, BNE);
declare_branch!(Bvs, overflow, true, BVS);
declare_branch!(Bvc, overflow, false, BVC);
declare_branch!(Bpl, negative, false, BPL);
declare_branch!(Bmi, negative, true, BMI);

// BIT
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

fn bit(cpu: &mut Cpu, val: usize) {
    let addr = (val & 0xFFFF) as u16;
    let imm = cpu.mem.get(addr);
    let fval = cpu.A & imm;
    cpu.flags.zero = fval == 0;
    cpu.flags.overflow = (imm & 0x40) != 0;
    cpu.flags.negative = (imm & 0x80) != 0
}

declare_addr_zero_page!(BitZp, BIT, bit);

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

fn lsr_core(cpu: &mut Cpu, data: u8) -> u8 {
    cpu.flags.carry = (data & 0x01) != 0;
    let res = data >> 1;
    cpu.flags.zero = res == 0;
    cpu.flags.negative = (res & (0x80 as u8)) != 0;
    res
}

declare_addr_zero_page2!(LsrZp, LSR, lsr_core);
declare_addr_abs2!(LsrAbs, LSR, lsr_core);
declare_addr_zero_page_reg2!(LsrZpX,  LSR, lsr_core);
declare_addr_abs_reg2!(LsrAbsX, LSR, lsr_core);

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

fn asl_core(cpu: &mut Cpu, data: u8) -> u8 {
    cpu.flags.carry = (data & 0x80) != 0;
    let res = data << 1;
    cpu.flags.zero = res == 0;
    cpu.flags.negative = (res & (0x80 as u8)) != 0;
    res
}

declare_addr_zero_page2!(AslZp, ASL, asl_core);
declare_addr_abs2!(AslAbs, ASL, asl_core);
declare_addr_zero_page_reg2!(AslZpX, ASL, asl_core);
declare_addr_abs_reg2!(AslAbsX, ASL, asl_core);

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

fn ror_core(cpu: &mut Cpu, data: u8) -> u8 {
    let res: u8 = (data >> 1) | (if cpu.flags.carry { 0x80 } else { 0 });
    cpu.flags.carry = (data & 0x01) != 0;
    cpu.flags.zero = res == 0;
    cpu.flags.negative = (res & (0x80 as u8)) != 0;
    res
}

declare_addr_zero_page2!(RorZp, ROR, ror_core);
declare_addr_abs2!(RorAbs, ROR, ror_core);
declare_addr_zero_page_reg2!(RorZpX, ROR, ror_core);
declare_addr_abs_reg2!(RorAbsX, ROR, ror_core);

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

fn rol_core(cpu: &mut Cpu, data: u8) -> u8 {
    let res: u8 = (data << 1) | (if cpu.flags.carry { 0x01 } else { 0 });
    cpu.flags.carry = (data & 0x80) != 0;
    cpu.flags.zero = res == 0;
    cpu.flags.negative = (res & (0x80 as u8)) != 0;
    res
}

declare_addr_zero_page2!(RolZp, ROL, rol_core);
declare_addr_abs2!(RolAbs, ROL, rol_core);
declare_addr_zero_page_reg2!(RolZpX, ROL, rol_core);
declare_addr_abs_reg2!(RolAbsX, ROL, rol_core);

fn inc_core(cpu: &mut Cpu, data: u8) -> u8 {
    let res: u8 = data.overflowing_add(1).0;
    cpu.flags.zero = res == 0;
    cpu.flags.negative = (res & (0x80 as u8)) != 0;
    res
}

declare_addr_zero_page2!(IncZp, INC, inc_core);
declare_addr_abs2!(IncAbs, INC, inc_core);
declare_addr_zero_page_reg2!(IncZpX, INC, inc_core);
declare_addr_abs_reg2!(IncAbsX, INC, inc_core);

fn dec_core(cpu: &mut Cpu, data: u8) -> u8 {
    let res: u8 = data.overflowing_sub(1).0;
    cpu.flags.zero = res == 0;
    cpu.flags.negative = (res & (0x80 as u8)) != 0;
    res
}

declare_addr_zero_page2!(DecZp, DEC, dec_core);
declare_addr_abs2!(DecAbs, DEC, dec_core);
declare_addr_zero_page_reg2!(DecZpX, DEC, dec_core);
declare_addr_abs_reg2!(DecAbsX, DEC, dec_core);

declare_addr_nop!(Nop, NOP);
/* unofficial opcodes */

fn nop_addr(_cpu: &mut Cpu, _val: usize) { }
declare_addr_zero_page!(NopZp, NOP, nop_addr, true);
declare_addr_abs!(NopAbs, NOP, nop_addr, true);
declare_addr_zero_page_reg!(NopZpX, NOP, X, nop_addr, true);
declare_addr_nop!(NopIllegal, NOP, true);
declare_addr_imm!(NopImm, NOP, nop_addr, true);
declare_addr_abs_reg!(NopAbsX, NOP, X, nop_addr, true);
