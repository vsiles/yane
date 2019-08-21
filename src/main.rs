use std::env;
use std::fs;
use std::time::SystemTime;

mod cpu;
mod format;
mod memory;
use memory::Memory;

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

use cpu::bit::bit_abs::BitAbs;
use cpu::bit::bit_zp::BitZp;

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
    ($name: ident, $opcode: ident) => {{
        *$opcode = Box::new($name::new());
        State::Processing
    }};
}

fn cycle(cpu: &mut Cpu, opcode: &mut Box<dyn OpCode>, state: State, nr: &mut usize) -> State {
    // print!("> [CYCLE {:04} PC {:#04x}]", *nr, cpu.pc);
    *nr = *nr + 1;
    match state {
        State::FetchOpcode => {
            let op = cpu.read_from_pc();
            // println!("Fetching Opcode {:02x}", op);
            match op {
                0x08 => add_opcode!(Php, opcode),
                0x09 => add_opcode!(OraImm, opcode),
                0x10 => add_opcode!(Bpl, opcode),
                0x18 => add_opcode!(Clc, opcode),
                0x20 => add_opcode!(Jsr, opcode),
                0x24 => add_opcode!(BitZp, opcode),
                0x28 => add_opcode!(Plp, opcode),
                0x29 => add_opcode!(AndImm, opcode),
                0x2c => add_opcode!(BitAbs, opcode),
                0x30 => add_opcode!(Bmi, opcode),
                0x38 => add_opcode!(Sec, opcode),
                0x48 => add_opcode!(Pha, opcode),
                0x49 => add_opcode!(EorImm, opcode),
                0x4C => add_opcode!(Jmp, opcode),
                0x50 => add_opcode!(Bvc, opcode),
                0x58 => add_opcode!(Cli, opcode),
                0x60 => add_opcode!(Rts, opcode),
                0x68 => add_opcode!(Pla, opcode),
                0x69 => add_opcode!(AdcImm, opcode),
                0x70 => add_opcode!(Bvs, opcode),
                0x78 => add_opcode!(Sei, opcode),
                0x81 => add_opcode!(StaNdxInd, opcode),
                0x84 => add_opcode!(StyZeroPage, opcode),
                0x85 => add_opcode!(StaZeroPage, opcode),
                0x86 => add_opcode!(StxZeroPage, opcode),
                0x8C => add_opcode!(StyAbs, opcode),
                0x8D => add_opcode!(StaAbs, opcode),
                0x8E => add_opcode!(StxAbs, opcode),
                0x90 => add_opcode!(Bcc, opcode),
                0x91 => add_opcode!(StaIndNdx, opcode),
                0x94 => add_opcode!(StyZeroPageX, opcode),
                0x95 => add_opcode!(StaZeroPageX, opcode),
                0x96 => add_opcode!(StxZeroPageY, opcode),
                0x99 => add_opcode!(StaAbsY, opcode),
                0x9D => add_opcode!(StaAbsX, opcode),
                0xA0 => add_opcode!(LdyImm, opcode),
                0xA1 => add_opcode!(LdaNdxInd, opcode),
                0xA2 => add_opcode!(LdxImm, opcode),
                0xA4 => add_opcode!(LdyZeroPage, opcode),
                0xA5 => add_opcode!(LdaZeroPage, opcode),
                0xA6 => add_opcode!(LdxZeroPage, opcode),
                0xA9 => add_opcode!(LdaImm, opcode),
                0xAC => add_opcode!(LdyAbs, opcode),
                0xAD => add_opcode!(LdaAbs, opcode),
                0xAE => add_opcode!(LdxAbs, opcode),
                0xB0 => add_opcode!(Bcs, opcode),
                0xB1 => add_opcode!(LdaIndNdx, opcode),
                0xB4 => add_opcode!(LdyZeroPageX, opcode),
                0xB5 => add_opcode!(LdaZeroPageX, opcode),
                0xB6 => add_opcode!(LdxZeroPageY, opcode),
                0xB8 => add_opcode!(Clv, opcode),
                0xB9 => add_opcode!(LdaAbsY, opcode),
                0xBC => add_opcode!(LdyAbsX, opcode),
                0xBD => add_opcode!(LdaAbsX, opcode),
                0xBE => add_opcode!(LdxAbsY, opcode),
                0xC0 => add_opcode!(CpyImm, opcode),
                0xC9 => add_opcode!(CmpImm, opcode),
                0xD0 => add_opcode!(Bne, opcode),
                0xD8 => add_opcode!(Cld, opcode),
                0xE0 => add_opcode!(CpxImm, opcode),
                0xE9 => add_opcode!(SbcImm, opcode),
                0xEA => add_opcode!(Nop, opcode),
                0xF0 => add_opcode!(Beq, opcode),
                0xF8 => add_opcode!(Sed, opcode),
                _ => {
                    /*TODO deal with errors */
                    State::Done
                }
            }
        }
        State::Processing => {
            // println!("Processing");
            if opcode.decode(cpu) {
                opcode.log(cpu);
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

    let start = SystemTime::now();

    loop {
        state = cycle(&mut cpu, &mut opcode, state, &mut nr);
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
