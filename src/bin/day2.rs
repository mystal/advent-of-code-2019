// https://adventofcode.com/2019/day/2

use std::error::Error;
use std::fs;

#[derive(Clone, Debug)]
struct Intcode {
    memory: Vec<i32>,
    iptr: usize,
}

impl Intcode {
    fn from_program(program: &str) -> Self {
        let memory: Vec<i32> = program.trim_end().split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        Self {
            memory,
            iptr: 0,
        }
    }

    fn run(&mut self) {
        loop {
            match self.memory.get(self.iptr) {
                None | Some(99) => break,
                Some(1) => {
                    let src0 = self.memory[self.iptr + 1] as usize;
                    let src1 = self.memory[self.iptr + 2] as usize;
                    let dst = self.memory[self.iptr + 3] as usize;
                    self.memory[dst] = self.memory[src0] + self.memory[src1];
                }
                Some(2) => {
                    let src0 = self.memory[self.iptr + 1] as usize;
                    let src1 = self.memory[self.iptr + 2] as usize;
                    let dst = self.memory[self.iptr + 3] as usize;
                    self.memory[dst] = self.memory[src0] * self.memory[src1];
                }
                Some(opcode) => panic!("Unknown opcode: {}!", opcode),
            }

            // Increment the instruction pointer.
            self.iptr += 4;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Make a separate binary to run intcode on arbitrary input.
    let (program, test) = match std::env::args().nth(1) {
        None => (fs::read_to_string("input/day2.txt")?, false),
        Some(s) => (s, true),
    };

    let init_intcode = Intcode::from_program(&program);
    let mut intcode = init_intcode.clone();

    println!("Initial state: {:?}", &intcode);

    // Part 1
    if !test {
        intcode.memory[1] = 12;
        intcode.memory[2] = 2;
    }

    intcode.run();

    println!("For noun 12, verbe 2: {:?}", &intcode);

    // Part 2
    for noun in 0..100 {
        for verb in 0..100 {
            intcode = init_intcode.clone();
            intcode.memory[1] = noun;
            intcode.memory[2] = verb;
            intcode.run();
            if intcode.memory[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                break;
            }
        }
    }

    Ok(())
}
