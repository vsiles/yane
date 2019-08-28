use std::env;
use std::fs;
use std::time::SystemTime;

mod cpu;
mod format;
mod memory;
use memory::Memory;

use cpu::*;
use cpu::jsr::*;
use cpu::jmp::*;
use cpu::nop::*;
use cpu::rts::*;
use cpu::rti::*;
use cpu::opcode::OpCode;

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

fn cycle(
    cpu: &mut Cpu,
    opcode: &mut Box<dyn OpCode>,
    state: State,
    start_cycle: &mut usize,
    nr: &mut usize,
) -> State {
    // print!("> [CYCLE {:04} PC {:#04x}]", *nr, cpu.pc);
    *nr = *nr + 1;
    match state {
        State::FetchOpcode => {
            *start_cycle = *nr - 1;
            // TODO: use array ?
            let op = cpu.read_from_pc();
            // println!("Fetching Opcode {:02x}", op);
            match op {
                0x01 => add_opcode!(OraIndX, opcode, cpu),
                0x04 => add_opcode!(NopZp, opcode, cpu),
                0x05 => add_opcode!(OraZp, opcode, cpu),
                0x06 => add_opcode!(AslZp, opcode, cpu),
                0x08 => add_opcode!(Php, opcode, cpu),
                0x09 => add_opcode!(OraImm, opcode, cpu),
                0x11 => add_opcode!(OraIndY, opcode, cpu),
                0x0A => add_opcode!(AslA, opcode, cpu),
                0x0D => add_opcode!(OraAbs, opcode, cpu),
                0x0E => add_opcode!(AslAbs, opcode, cpu),
                0x10 => add_opcode!(Bpl, opcode, cpu),
                0x15 => add_opcode!(OraZpX, opcode, cpu),
                0x16 => add_opcode!(AslZpX, opcode, cpu),
                0x18 => add_opcode!(Clc, opcode, cpu),
                0x19 => add_opcode!(OraAbsY, opcode, cpu),
                0x1D => add_opcode!(OraAbsX, opcode, cpu),
                0x1E => add_opcode!(AslAbsX, opcode, cpu),
                0x20 => add_opcode!(Jsr, opcode, cpu),
                0x21 => add_opcode!(AndIndX, opcode, cpu),
                0x24 => add_opcode!(BitZp, opcode, cpu),
                0x25 => add_opcode!(AndZp, opcode, cpu),
                0x26 => add_opcode!(RolZp, opcode, cpu),
                0x28 => add_opcode!(Plp, opcode, cpu),
                0x29 => add_opcode!(AndImm, opcode, cpu),
                0x2A => add_opcode!(RolA, opcode, cpu),
                0x2C => add_opcode!(BitAbs, opcode, cpu),
                0x2D => add_opcode!(AndAbs, opcode, cpu),
                0x2E => add_opcode!(RolAbs, opcode, cpu),
                0x30 => add_opcode!(Bmi, opcode, cpu),
                0x31 => add_opcode!(AndIndY, opcode, cpu),
                0x35 => add_opcode!(AndZpX, opcode, cpu),
                0x36 => add_opcode!(RolZpX, opcode, cpu),
                0x38 => add_opcode!(Sec, opcode, cpu),
                0x39 => add_opcode!(AndAbsY, opcode, cpu),
                0x3D => add_opcode!(AndAbsX, opcode, cpu),
                0x3E => add_opcode!(RolAbsX, opcode, cpu),
                0x40 => add_opcode!(Rti, opcode, cpu),
                0x41 => add_opcode!(EorIndX, opcode, cpu),
                0x44 => add_opcode!(NopZp, opcode, cpu),
                0x45 => add_opcode!(EorZp, opcode, cpu),
                0x46 => add_opcode!(LsrZp, opcode, cpu),
                0x48 => add_opcode!(Pha, opcode, cpu),
                0x49 => add_opcode!(EorImm, opcode, cpu),
                0x4A => add_opcode!(LsrA, opcode, cpu),
                0x4C => add_opcode!(Jmp, opcode, cpu),
                0x4D => add_opcode!(EorAbs, opcode, cpu),
                0x4E => add_opcode!(LsrAbs, opcode, cpu),
                0x50 => add_opcode!(Bvc, opcode, cpu),
                0x51 => add_opcode!(EorIndY, opcode, cpu),
                0x55 => add_opcode!(EorZpX, opcode, cpu),
                0x56 => add_opcode!(LsrZpX, opcode, cpu),
                0x58 => add_opcode!(Cli, opcode, cpu),
                0x59 => add_opcode!(EorAbsY, opcode, cpu),
                0x5D => add_opcode!(EorAbsX, opcode, cpu),
                0x5E => add_opcode!(LsrAbsX, opcode, cpu),
                0x60 => add_opcode!(Rts, opcode, cpu),
                0x61 => add_opcode!(AdcIndX, opcode, cpu),
                0x64 => add_opcode!(NopZp, opcode, cpu),
                0x65 => add_opcode!(AdcZp, opcode, cpu),
                0x66 => add_opcode!(RorZp, opcode, cpu),
                0x68 => add_opcode!(Pla, opcode, cpu),
                0x69 => add_opcode!(AdcImm, opcode, cpu),
                0x6A => add_opcode!(RorA, opcode, cpu),
                0x6C => add_opcode!(JmpInd, opcode, cpu),
                0x6D => add_opcode!(AdcAbs, opcode, cpu),
                0x6E => add_opcode!(RorAbs, opcode, cpu),
                0x70 => add_opcode!(Bvs, opcode, cpu),
                0x71 => add_opcode!(AdcIndY, opcode, cpu),
                0x75 => add_opcode!(AdcZpX, opcode, cpu),
                0x76 => add_opcode!(RorZpX, opcode, cpu),
                0x78 => add_opcode!(Sei, opcode, cpu),
                0x79 => add_opcode!(AdcAbsY, opcode, cpu),
                0x7D => add_opcode!(AdcAbsX, opcode, cpu),
                0x7E => add_opcode!(RorAbsX, opcode, cpu),
                0x81 => add_opcode!(StaIndX, opcode, cpu),
                0x84 => add_opcode!(StyZeroPage, opcode, cpu),
                0x85 => add_opcode!(StaZeroPage, opcode, cpu),
                0x86 => add_opcode!(StxZeroPage, opcode, cpu),
                0x88 => add_opcode!(DeY, opcode, cpu),
                0x8A => add_opcode!(TXA, opcode, cpu),
                0x8C => add_opcode!(StyAbs, opcode, cpu),
                0x8D => add_opcode!(StaAbs, opcode, cpu),
                0x8E => add_opcode!(StxAbs, opcode, cpu),
                0x90 => add_opcode!(Bcc, opcode, cpu),
                0x91 => add_opcode!(StaIndY, opcode, cpu),
                0x94 => add_opcode!(StyZeroPageX, opcode, cpu),
                0x95 => add_opcode!(StaZeroPageX, opcode, cpu),
                0x96 => add_opcode!(StxZeroPageY, opcode, cpu),
                0x98 => add_opcode!(TYA, opcode, cpu),
                0x99 => add_opcode!(StaAbsY, opcode, cpu),
                0x9A => add_opcode!(TXS, opcode, cpu),
                0x9D => add_opcode!(StaAbsX, opcode, cpu),
                0xA0 => add_opcode!(LdyImm, opcode, cpu),
                0xA1 => add_opcode!(LdaIndX, opcode, cpu),
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
                0xB1 => add_opcode!(LdaIndY, opcode, cpu),
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
                0xC1 => add_opcode!(CmpIndX, opcode, cpu),
                0xC4 => add_opcode!(CpyZp, opcode, cpu),
                0xC5 => add_opcode!(CmpZp, opcode, cpu),
                0xC6 => add_opcode!(DecZp, opcode, cpu),
                0xC8 => add_opcode!(InY, opcode, cpu),
                0xC9 => add_opcode!(CmpImm, opcode, cpu),
                0xCA => add_opcode!(DeX, opcode, cpu),
                0xCC => add_opcode!(CpyAbs, opcode, cpu),
                0xCD => add_opcode!(CmpAbs, opcode, cpu),
                0xCE => add_opcode!(DecAbs, opcode, cpu),
                0xD0 => add_opcode!(Bne, opcode, cpu),
                0xD1 => add_opcode!(CmpIndY, opcode, cpu),
                0xD5 => add_opcode!(CmpZpX, opcode, cpu),
                0xD6 => add_opcode!(DecZpX, opcode, cpu),
                0xD8 => add_opcode!(Cld, opcode, cpu),
                0xD9 => add_opcode!(CmpAbsY, opcode,  cpu),
                0xDD => add_opcode!(CmpAbsX, opcode,  cpu),
                0xDE => add_opcode!(DecAbsX, opcode, cpu),
                0xE0 => add_opcode!(CpxImm, opcode, cpu),
                0xE1 => add_opcode!(SbcIndX, opcode, cpu),
                0xE4 => add_opcode!(CpxZp, opcode, cpu),
                0xE5 => add_opcode!(SbcZp, opcode, cpu),
                0xE6 => add_opcode!(IncZp, opcode, cpu),
                0xE8 => add_opcode!(InX, opcode, cpu),
                0xE9 => add_opcode!(SbcImm, opcode, cpu),
                0xEA => add_opcode!(Nop, opcode, cpu),
                0xEC => add_opcode!(CpxAbs, opcode, cpu),
                0xED => add_opcode!(SbcAbs, opcode, cpu),
                0xEE => add_opcode!(IncAbs, opcode, cpu),
                0xF0 => add_opcode!(Beq, opcode, cpu),
                0xF1 => add_opcode!(SbcIndY, opcode, cpu),
                0xF5 => add_opcode!(SbcZpX, opcode, cpu),
                0xF6 => add_opcode!(IncZpX, opcode, cpu),
                0xF8 => add_opcode!(Sed, opcode, cpu),
                0xF9 => add_opcode!(SbcAbsY, opcode, cpu),
                0xFD => add_opcode!(SbcAbsX, opcode, cpu),
                0xFE => add_opcode!(IncAbsX, opcode, cpu),
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
        }
        None => {
            println!("Invalid header");
            std::process::exit(1)
        }
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
