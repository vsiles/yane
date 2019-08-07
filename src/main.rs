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
    ($name: ident,$opcode: ident, $size: expr) => {
        {
            *$opcode = Box::new($name::new($size));
            State::Processing
        }
    };
}

fn cycle(cpu: &mut Cpu, opcode: &mut Box<OpCode>, state: State, nr: &mut usize) -> State {
    // print!("> [CYCLE {:04} PC {:#04x}]", *nr, cpu.pc);
    *nr = *nr + 1;
    match state {
        State::FetchOpcode => {
            let op = cpu.read_from_pc();
            // println!("Fetching Opcode {:02x}", op);
            match op {
                0x81 => add_opcode!(STANdxInd,    opcode, 2),
                0x84 => add_opcode!(STYZeroPage,  opcode, 2),
                0x85 => add_opcode!(STAZeroPage,  opcode, 2),
                0x86 => add_opcode!(STXZeroPage,  opcode, 2),
                0x8C => add_opcode!(STYAbs,       opcode, 3),
                0x8D => add_opcode!(STAAbs,       opcode, 3),
                0x8E => add_opcode!(STXAbs,       opcode, 3),
                0x91 => add_opcode!(STAIndNdx,    opcode, 2),
                0x94 => add_opcode!(STYZeroPageX, opcode, 2),
                0x95 => add_opcode!(STAZeroPageX, opcode, 2),
                0x96 => add_opcode!(STXZeroPageY, opcode, 2),
                0x99 => add_opcode!(STAAbsY,      opcode, 3),
                0x9D => add_opcode!(STAAbsX,      opcode, 3),
                0xA0 => add_opcode!(LDYImm,       opcode, 2),
                0xA1 => add_opcode!(LDANdxInd,    opcode, 2),
                0xA2 => add_opcode!(LDXImm,       opcode, 2),
                0xA4 => add_opcode!(LDYZeroPage,  opcode, 2),
                0xA5 => add_opcode!(LDAZeroPage,  opcode, 2),
                0xA6 => add_opcode!(LDXZeroPage,  opcode, 2),
                0xA9 => add_opcode!(LDAImm,       opcode, 2),
                0xAC => add_opcode!(LDYAbs,       opcode, 3),
                0xAD => add_opcode!(LDAAbs,       opcode, 3),
                0xAE => add_opcode!(LDXAbs,       opcode, 3),
                0xB1 => add_opcode!(LDAIndNdx,    opcode, 2),
                0xB4 => add_opcode!(LDYZeroPageX, opcode, 2),
                0xB5 => add_opcode!(LDAZeroPageX, opcode, 2),
                0xB6 => add_opcode!(LDXZeroPageY, opcode, 2),
                0xB9 => add_opcode!(LDAAbsY,      opcode, 3),
                0xBC => add_opcode!(LDYAbsX,      opcode, 3),
                0xBD => add_opcode!(LDAAbsX,      opcode, 3),
                0xBE => add_opcode!(LDXAbsY,      opcode, 3),
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
    let mut opcode : Box<dyn OpCode> = Box::new(Nop::new(0));
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