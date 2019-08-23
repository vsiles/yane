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
pub mod and;
pub mod eor;
pub mod ora;
pub mod sbc;
#[macro_use]
pub mod incr;
#[macro_use]
pub mod decr;
#[macro_use]
pub mod trs;
pub mod asl;
pub mod lsr;
pub mod ror;
pub mod rti;

pub use cpu::*;
pub use opcode::OpCode;

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
declare_cmp_imm!(cmp_imm, CmpImm, CMP, A);
declare_cmp_imm!(cpx_imm, CpxImm, CPX, X);
declare_cmp_imm!(cpy_imm, CpyImm, CPY, Y);

// LDA, LDX, LDY
declare_load_imm!(lda_imm, LdaImm, A);
declare_load_imm!(ldx_imm, LdxImm, X);
declare_load_imm!(ldy_imm, LdyImm, Y);

declare_load_zero_page!(lda_zero_page, LdaZeroPage, A);
declare_load_zero_page!(ldx_zero_page, LdxZeroPage, X);
declare_load_zero_page!(ldy_zero_page, LdyZeroPage, Y);

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

declare_load_ndx_ind!(lda_ndx_ind, LdaNdxInd, A);
declare_load_ind_ndx!(lda_ind_ndx, LdaIndNdx, A);

// STA, STX, STY
declare_store_zero_page!(sta_zero_page, StaZeroPage, A);
declare_store_zero_page!(stx_zero_page, StxZeroPage, X);
declare_store_zero_page!(sty_zero_page, StyZeroPage, Y);

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
    use super::super::flags::CpuFlags;
    use super::super::Cpu;
    use super::super::OpCode;

    const SIZE: u16 = 3;

    pub struct BitAbs {
        low: u8,
        high: u8,
        imm: u8,
        state: usize,
        saved: CpuFlags,
    }

    impl OpCode for BitAbs {
        fn new() -> BitAbs {
            BitAbs {
                low: 0,
                high: 0,
                imm: 0,
                state: 0,
                saved: CpuFlags::new(),
            }
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            if self.state == 0 {
                self.saved = cpu.flags.clone();
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
            let pc = cpu.pc - SIZE;
            let code = cpu.mem.get(pc);
            let addr = mk_addr!(self.low, self.high);
            print!(
                "{:04X}  {:02X} {:02X} {:02X}  BIT ${:04X}",
                pc, code, self.low, self.high, addr
            );
            let mut old_cpu = cpu.debug_clone();
            old_cpu.flags = self.saved.clone();
            print!(" = {:02X} {: >17}{}", self.imm, "", old_cpu);
        }
    }
}

pub mod bit_zp {
    use super::super::Cpu;
    use super::super::OpCode;

    pub struct BitZp {
        addr: u8,
        state: usize,
    }

    impl OpCode for BitZp {
        fn new() -> BitZp {
            BitZp { addr: 0, state: 0 }
        }

        fn decode(&mut self, cpu: &mut Cpu) -> bool {
            if self.state == 0 {
                // read offset from memory
                self.addr = cpu.read_from_pc();
                self.state = 1;
                false
            } else {
                // read data from memory using offset in page 0
                let imm = cpu.mem.get(self.addr as u16);
                let val = cpu.A & imm;
                cpu.flags.zero = val == 0;
                cpu.flags.overflow = (imm & 0x40) != 0;
                cpu.flags.negative = (imm & 0x80) != 0;
                true
            }
        }

        fn log(&self, cpu: &Cpu) {
            let pc = cpu.pc - 1;
            let code = cpu.mem.get(pc);
            let imm = cpu.mem.get(pc + 1);
            let old = cpu.mem.get(imm as u16);
            print!("{:04X}  {:02X} {:02X}     BIT ${:02X}", pc, code, imm, imm);
            print!(" = {:02X}{: >20}{}", old, "", cpu)
        }
    }
}

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
