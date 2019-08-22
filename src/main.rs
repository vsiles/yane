use std::env;
use std::fs;
use std::time::SystemTime;

mod cpu;
mod format;
mod memory;
use memory::Memory;

use cpu::tax::TAX;
use cpu::tay::TAY;
use cpu::txa::TXA;
use cpu::tya::TYA;
use cpu::txs::TXS;
use cpu::tsx::TSX;

use cpu::inx::InX;
use cpu::iny::InY;
use cpu::dex::DeX;
use cpu::dey::DeY;

use cpu::cmp_imm::CmpImm;
use cpu::cpx_imm::CpxImm;
use cpu::cpy_imm::CpyImm;

use cpu::lda_abs::LdaAbs;
use cpu::lda_abs_x::LdaAbsX;
use cpu::lda_abs_y::LdaAbsY;
use cpu::lda_imm::LdaImm;
use cpu::lda_ind_ndx::LdaIndNdx;
use cpu::lda_ndx_ind::LdaNdxInd;
use cpu::lda_zero_page::LdaZeroPage;
use cpu::lda_zero_page_x::LdaZeroPageX;
use cpu::ldx_abs::LdxAbs;
use cpu::ldx_abs_y::LdxAbsY;
use cpu::ldx_imm::LdxImm;
use cpu::ldx_zero_page::LdxZeroPage;
use cpu::ldx_zero_page_y::LdxZeroPageY;
use cpu::ldy_abs::LdyAbs;
use cpu::ldy_abs_x::LdyAbsX;
use cpu::ldy_imm::LdyImm;
use cpu::ldy_zero_page::LdyZeroPage;
use cpu::ldy_zero_page_x::LdyZeroPageX;
use cpu::*;

use cpu::sta_abs::StaAbs;
use cpu::sta_abs_x::StaAbsX;
use cpu::sta_abs_y::StaAbsY;
use cpu::sta_ind_ndx::StaIndNdx;
use cpu::sta_ndx_ind::StaNdxInd;
use cpu::sta_zero_page::StaZeroPage;
use cpu::sta_zero_page_x::StaZeroPageX;
use cpu::stx_abs::StxAbs;
use cpu::stx_zero_page::StxZeroPage;
use cpu::stx_zero_page_y::StxZeroPageY;
use cpu::sty_abs::StyAbs;
use cpu::sty_zero_page::StyZeroPage;
use cpu::sty_zero_page_x::StyZeroPageX;

use cpu::jmp::Jmp;
use cpu::jsr::Jsr;
use cpu::rts::Rts;
use cpu::nop::Nop;
use cpu::sec::Sec;
use cpu::sed::Sed;
use cpu::sei::Sei;
use cpu::clc::Clc;
use cpu::cld::Cld;
use cpu::cli::Cli;
use cpu::clv::Clv;

use cpu::bcs::Bcs;
use cpu::bcc::Bcc;
use cpu::beq::Beq;
use cpu::bne::Bne;
use cpu::bvs::Bvs;
use cpu::bvc::Bvc;
use cpu::bpl::Bpl;
use cpu::bmi::Bmi;

use cpu::bit_abs::BitAbs;
use cpu::bit_zp::BitZp;

use cpu::and::and_imm::AndImm;
use cpu::ora::ora_imm::OraImm;
use cpu::eor::eor_imm::EorImm;
use cpu::adc::adc_imm::AdcImm;
use cpu::sbc::sbc_imm::SbcImm;

use cpu::php::Php;
use cpu::pha::Pha;
use cpu::pla::Pla;
use cpu::plp::Plp;

enum State {
    FetchOpcode,
    Processing,
    Done,
}

macro_rules! add_opcode {
    ($name: ident, $opcode: ident, $cpu: ident) => {{
        *$opcode = Box::new($name::new());
        $opcode.log($cpu);
        println!("");
        // let ppu_cycle = *start_cycle * 3;
        // let frame_nr = ppu_cycle / 334;
        // println!(" PPU:{: >3}, {: >2} CYC:{}", ppu_cycle, frame_nr, *start_cycle + 7);
        State::Processing
    }};
}

fn cycle(cpu: &mut Cpu, opcode: &mut Box<dyn OpCode>, state: State, 
         start_cycle: &mut usize, nr: &mut usize) -> State {
    // print!("> [CYCLE {:04} PC {:#04x}]", *nr, cpu.pc);
    *nr = *nr + 1;
    match state {
        State::FetchOpcode => {
            *start_cycle = *nr - 1;
            let op = cpu.read_from_pc();
            // println!("Fetching Opcode {:02x}", op);
            match op {
                0x08 => add_opcode!(Php, opcode, cpu),
                0x09 => add_opcode!(OraImm, opcode, cpu),
                0x10 => add_opcode!(Bpl, opcode, cpu),
                0x18 => add_opcode!(Clc, opcode, cpu),
                0x20 => add_opcode!(Jsr, opcode, cpu),
                0x24 => add_opcode!(BitZp, opcode, cpu),
                0x28 => add_opcode!(Plp, opcode, cpu),
                0x29 => add_opcode!(AndImm, opcode, cpu),
                0x2c => add_opcode!(BitAbs, opcode, cpu),
                0x30 => add_opcode!(Bmi, opcode, cpu),
                0x38 => add_opcode!(Sec, opcode, cpu),
                0x48 => add_opcode!(Pha, opcode, cpu),
                0x49 => add_opcode!(EorImm, opcode, cpu),
                0x4C => add_opcode!(Jmp, opcode, cpu),
                0x50 => add_opcode!(Bvc, opcode, cpu),
                0x58 => add_opcode!(Cli, opcode, cpu),
                0x60 => add_opcode!(Rts, opcode, cpu),
                0x68 => add_opcode!(Pla, opcode, cpu),
                0x69 => add_opcode!(AdcImm, opcode, cpu),
                0x70 => add_opcode!(Bvs, opcode, cpu),
                0x78 => add_opcode!(Sei, opcode, cpu),
                0x81 => add_opcode!(StaNdxInd, opcode, cpu),
                0x84 => add_opcode!(StyZeroPage, opcode, cpu),
                0x85 => add_opcode!(StaZeroPage, opcode, cpu),
                0x86 => add_opcode!(StxZeroPage, opcode, cpu),
                0x88 => add_opcode!(DeY, opcode, cpu),
                0x8A => add_opcode!(TXA, opcode, cpu),
                0x8C => add_opcode!(StyAbs, opcode, cpu),
                0x8D => add_opcode!(StaAbs, opcode, cpu),
                0x8E => add_opcode!(StxAbs, opcode, cpu),
                0x90 => add_opcode!(Bcc, opcode, cpu),
                0x91 => add_opcode!(StaIndNdx, opcode, cpu),
                0x94 => add_opcode!(StyZeroPageX, opcode, cpu),
                0x95 => add_opcode!(StaZeroPageX, opcode, cpu),
                0x96 => add_opcode!(StxZeroPageY, opcode, cpu),
                0x98 => add_opcode!(TYA, opcode, cpu),
                0x99 => add_opcode!(StaAbsY, opcode, cpu),
                0x9A => add_opcode!(TXS, opcode, cpu),
                0x9D => add_opcode!(StaAbsX, opcode, cpu),
                0xA0 => add_opcode!(LdyImm, opcode, cpu),
                0xA1 => add_opcode!(LdaNdxInd, opcode, cpu),
                0xA2 => add_opcode!(LdxImm, opcode, cpu),
                0xA4 => add_opcode!(LdyZeroPage, opcode, cpu),
                0xA5 => add_opcode!(LdaZeroPage, opcode, cpu),
                0xA6 => add_opcode!(LdxZeroPage, opcode, cpu),
                0xA8 => add_opcode!(TAY, opcode, cpu),
                0xA9 => add_opcode!(LdaImm, opcode, cpu),
                0xAA => add_opcode!(TAX, opcode, cpu),
                0xAC => add_opcode!(LdyAbs, opcode, cpu),
                0xAD => add_opcode!(LdaAbs, opcode, cpu),
                0xAE => add_opcode!(LdxAbs, opcode, cpu),
                0xB0 => add_opcode!(Bcs, opcode, cpu),
                0xB1 => add_opcode!(LdaIndNdx, opcode, cpu),
                0xB4 => add_opcode!(LdyZeroPageX, opcode, cpu),
                0xB5 => add_opcode!(LdaZeroPageX, opcode, cpu),
                0xB6 => add_opcode!(LdxZeroPageY, opcode, cpu),
                0xB8 => add_opcode!(Clv, opcode, cpu),
                0xB9 => add_opcode!(LdaAbsY, opcode, cpu),
                0xBA => add_opcode!(TSX, opcode, cpu),
                0xBC => add_opcode!(LdyAbsX, opcode, cpu),
                0xBD => add_opcode!(LdaAbsX, opcode, cpu),
                0xBE => add_opcode!(LdxAbsY, opcode, cpu),
                0xC0 => add_opcode!(CpyImm, opcode, cpu),
                0xC8 => add_opcode!(InY, opcode, cpu),
                0xC9 => add_opcode!(CmpImm, opcode, cpu),
                0xCA => add_opcode!(DeX, opcode, cpu),
                0xD0 => add_opcode!(Bne, opcode, cpu),
                0xD8 => add_opcode!(Cld, opcode, cpu),
                0xE0 => add_opcode!(CpxImm, opcode, cpu),
                0xE8 => add_opcode!(InX, opcode, cpu),
                0xE9 => add_opcode!(SbcImm, opcode, cpu),
                0xEA => add_opcode!(Nop, opcode, cpu),
                0xF0 => add_opcode!(Beq, opcode, cpu),
                0xF8 => add_opcode!(Sed, opcode, cpu),
                _ => {
                    /*TODO deal with errors */
                    State::Done
                }
            }
        }
        State::Processing => {
            // println!("Processing");
            if opcode.decode(cpu) {
                State::FetchOpcode
            } else {
                State::Processing
            }
        }
        State::Done => state,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Command line arguments: {:?}\n", args);

    if args.len() < 2 {
        println!("Not enough arguments.");
        println!("Usage: {} filename", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    let mut rom = fs::read(filename).expect("Can't read input file");
    if rom.len() < 16 {
        println!("Invalid iNes file, no header can be found");
        std::process::exit(1);
    }
    let rom_data: Vec<_> = rom.drain(16..).collect();
    let opt_header = format::ines::new(&rom);
    let header = match opt_header {
        Some(header) => {
            println!("Dumping header info:\n{}\n", header);
            header
        },
        None => { println!("Invalid header"); std::process::exit(1) }
    };

    if header.mapper != 0 {
        println!("Sorry, only mapper 0 is supported at the moment");
        std::process::exit(1)
    }

    if header.prg_rom_size != 0x4000 {
        println!("Sorry, only NROM-128 is supported");
        std::process::exit(1);
    }

    let memory = Memory::new(rom_data);

    let mut cpu = cpu::new(memory);
    let mut opcode: Box<dyn OpCode> = Box::new(Nop::new());
    let mut state = State::FetchOpcode;
    let mut nr = 0;
    let mut start_cycle = 0;

    let start = SystemTime::now();

    loop {
        state = cycle(&mut cpu, &mut opcode, state, &mut start_cycle, &mut nr);
        match state {
            // State::Done => cpu.pc = 0,
            State::Done => break,
            _ => {}
        }

        let now = SystemTime::now();
        let ms = now.duration_since(start).unwrap().as_millis();
        if ms > 3000 {
            break;
        }
    }
    let now = SystemTime::now();
    let ms = now.duration_since(start).unwrap().as_millis();
    println!("Exiting...");
    println!("Total time: {}", (ms as f64) / 1000.0);
    println!("Total instructions: {}", nr);
    println!("IPS: {}", (nr as f64) * 1000.0 / (ms as f64))
}
