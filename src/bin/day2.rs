// https://adventofcode.com/2019/day/2

use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Intcode {
    memory: Vec<i32>,
    iptr: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let (program, test) = match std::env::args().nth(1) {
        None => (fs::read_to_string("input/day2.txt")?, false),
        Some(s) => (s, true),
    };
    let memory: Vec<i32> = program.trim_end().split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut intcode = Intcode {
        memory,
        iptr: 0,
    };

    if !test {
        intcode.memory[1] = 12;
        intcode.memory[2] = 2;
    }

    println!("{:?}", &intcode);

    loop {
        match intcode.memory.get(intcode.iptr) {
            None | Some(99) => break,
            Some(1) => {
                let src0 = intcode.memory[intcode.iptr + 1] as usize;
                let src1 = intcode.memory[intcode.iptr + 2] as usize;
                let dst = intcode.memory[intcode.iptr + 3] as usize;
                intcode.memory[dst] = intcode.memory[src0] + intcode.memory[src1];
            }
            Some(2) => {
                let src0 = intcode.memory[intcode.iptr + 1] as usize;
                let src1 = intcode.memory[intcode.iptr + 2] as usize;
                let dst = intcode.memory[intcode.iptr + 3] as usize;
                intcode.memory[dst] = intcode.memory[src0] * intcode.memory[src1];
            }
            Some(opcode) => panic!("Unknown opcode: {}!", opcode),
        }

        // Increment the instruction pointer.
        intcode.iptr += 4;
    }

    println!("{:?}", &intcode);

    Ok(())
}
