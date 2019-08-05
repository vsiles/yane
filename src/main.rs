use std::fs;
use std::env;

mod cpu;

use cpu::*;

enum State {
    FetchOpcode,
    Processing,
    Done,
}

// GENERAL TODO: deal with cycle for page stuff

fn cycle(cpu: &mut Cpu, opcode: &mut Box<OpCode>, state: State) -> State {
    match state {
        State::FetchOpcode => {
            let op = cpu.read_from_pc();
            println!("> Fetching Opcode {:02x}", op);
            match op {
                0xA9 => {
                    *opcode = Box::new(LDAImm::new());
                    State::Processing
                },
                0xA5 => {
                    *opcode = Box::new(LDAZeroPage::new());
                    State::Processing
                },
                _ => {
                    /*TODO deal with errors */
                    State::Done
                }
            }
        },
        State::Processing => {
            println!("> Processing");
            if opcode.decode(cpu) {
                opcode.execute(cpu);
                println!("< Done: PC {:#04x} A {:02x} Flags {}",
                    cpu.pc, cpu.a, cpu.flags);
                *opcode = Box::new(Nop::new());
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
    loop {
        println!("DEBUG: PC = {:#04x}", cpu.pc);
        state = cycle(&mut cpu, &mut opcode, state);
        match state {
            State::Done => break,
            _ => {}
        }
    }
    println!("Exiting...")
}