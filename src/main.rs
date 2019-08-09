use std::fs;
use std::env;
use std::time::SystemTime;

mod cpu;

use cpu::*;
use cpu::lda_imm::LdaImm;
use cpu::ldx_imm::LdxImm;
use cpu::ldy_imm::LdyImm;
use cpu::lda_zero_page::LdaZeroPage;
use cpu::ldx_zero_page::LdxZeroPage;
use cpu::ldy_zero_page::LdyZeroPage;
use cpu::lda_zero_page_x::LdaZeroPageX;
use cpu::ldy_zero_page_x::LdyZeroPageX;
use cpu::ldx_zero_page_y::LdxZeroPageY;
use cpu::lda_abs::LdaAbs;
use cpu::ldx_abs::LdxAbs;
use cpu::ldy_abs::LdyAbs;
use cpu::lda_abs_x::LdaAbsX;
use cpu::lda_abs_y::LdaAbsY;
use cpu::ldx_abs_y::LdxAbsY;
use cpu::ldy_abs_x::LdyAbsX;
use cpu::lda_ndx_ind::LdaNdxInd;
use cpu::lda_ind_ndx::LdaIndNdx;

use cpu::sta_zero_page::StaZeroPage;
use cpu::stx_zero_page::StxZeroPage;
use cpu::sty_zero_page::StyZeroPage;
use cpu::sta_zero_page_x::StaZeroPageX;
use cpu::stx_zero_page_y::StxZeroPageY;
use cpu::sty_zero_page_x::StyZeroPageX;
use cpu::sta_abs::StaAbs;
use cpu::stx_abs::StxAbs;
use cpu::sty_abs::StyAbs;
use cpu::sta_abs_x::StaAbsX;
use cpu::sta_abs_y::StaAbsY;
use cpu::sta_ndx_ind::StaNdxInd;
use cpu::sta_ind_ndx::StaIndNdx;

use cpu::jmp::Jmp;
use cpu::jsr::Jsr;

enum State {
    FetchOpcode,
    Processing,
    Done,
}

macro_rules! add_opcode {
    ($name: ident,$opcode: ident) => {
        {
            *$opcode = Box::new($name::new());
            State::Processing
        }
    };
}

fn cycle(cpu: &mut Cpu, opcode: &mut Box<dyn OpCode>, state: State, nr: &mut usize) -> State {
    // print!("> [CYCLE {:04} PC {:#04x}]", *nr, cpu.pc);
    *nr = *nr + 1;
    match state {
        State::FetchOpcode => {
            let op = cpu.read_from_pc();
            // println!("Fetching Opcode {:02x}", op);
            match op {
                0x81 => add_opcode!(StaNdxInd, opcode),
                0x84 => add_opcode!(StyZeroPage, opcode),
                0x85 => add_opcode!(StaZeroPage, opcode),
                0x86 => add_opcode!(StxZeroPage, opcode),
                0x8C => add_opcode!(StyAbs, opcode),
                0x8D => add_opcode!(StaAbs, opcode),
                0x8E => add_opcode!(StxAbs, opcode),
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
                0xB1 => add_opcode!(LdaIndNdx, opcode),
                0xB4 => add_opcode!(LdyZeroPageX, opcode),
                0xB5 => add_opcode!(LdaZeroPageX, opcode),
                0xB6 => add_opcode!(LdxZeroPageY, opcode),
                0xB9 => add_opcode!(LdaAbsY, opcode),
                0xBC => add_opcode!(LdyAbsX, opcode),
                0xBD => add_opcode!(LdaAbsX, opcode),
                0xBE => add_opcode!(LdxAbsY, opcode),
                0x4C => add_opcode!(Jmp, opcode),
                0x20 => add_opcode!(Jsr, opcode),
                _ => {
                    /*TODO deal with errors */
                    State::Done
                }
            }
        },
        State::Processing => {
            // println!("Processing");
            if opcode.decode(cpu) {
                opcode.log(cpu);
                State::FetchOpcode
            } else {
                State::Processing
            }
        },
        State::Done => state
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

    let rom = fs::read(filename).expect("Can't read input file");

    let mut cpu = cpu::new(rom);
    let mut opcode : Box<dyn OpCode> = Box::new(Nop::new());
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
