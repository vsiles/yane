use std::fs;
use std::env;
use std::time::SystemTime;

mod cpu;

use cpu::*;

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

// GENERAL TODO: deal with cycle for page stuff

fn cycle(cpu: &mut Cpu, opcode: &mut Box<OpCode>, state: State, nr: &mut usize) -> State {
    print!("> [CYCLE {:04} PC {:#04x}]", *nr, cpu.pc);
    *nr = *nr + 1;
    match state {
        State::FetchOpcode => {
            let op = cpu.read_from_pc();
            println!("Fetching Opcode {:02x}", op);
            match op {
                0xA0 => add_opcode!(LDYImm, opcode),
                0xA2 => add_opcode!(LDXImm, opcode),
                0xA4 => add_opcode!(LDYZeroPage, opcode),
                0xA5 => add_opcode!(LDAZeroPage, opcode),
                0xA6 => add_opcode!(LDXZeroPage, opcode),
                0xA9 => add_opcode!(LDAImm, opcode),
                0xAC => add_opcode!(LDYAbs, opcode),
                0xAD => add_opcode!(LDAAbs, opcode),
                0xAE => add_opcode!(LDXAbs, opcode),
                0xB4 => add_opcode!(LDYZeroPageX, opcode),
                0xB5 => add_opcode!(LDAZeroPageX, opcode),
                0xB6 => add_opcode!(LDXZeroPageY, opcode),
                0xB9 => add_opcode!(LDAAbsY, opcode),
                0xBC => add_opcode!(LDYAbsX, opcode),
                0xBD => add_opcode!(LDAAbsX, opcode),
                0xBE => add_opcode!(LDXAbsY, opcode),
                _ => {
                    /*TODO deal with errors */
                    State::Done
                }
            }
        },
        State::Processing => {
            println!("Processing");
            if opcode.decode(cpu) {
                opcode.execute(cpu);
                println!("< DUMP: A {:02x} X {:02x} Y {:02x} Flags {}",
                    cpu.a, cpu.x, cpu.y, cpu.flags);
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
        if ms > 500 {
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
