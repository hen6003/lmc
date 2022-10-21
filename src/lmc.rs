pub trait InputProvider {
    fn input(&mut self) -> i16;
    fn output(&mut self, data: i16);
}

pub struct LMC<T: InputProvider> {
    pc: u8,
    accumulator: i16,
    memory: [i16; 100],
    halted: bool,
    input_provider: T,
}

impl<T: InputProvider> LMC<T> {
    pub fn new(input_provider: T, memory: [i16; 100]) -> Self {
        Self {
            pc: 0,
            accumulator: 0,
            memory,
            halted: false,
            input_provider,
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            let instruction = self.memory[self.pc as usize];
            self.pc += 1;

            let opcode = instruction / 100;
            let operand = instruction % 100;

            match opcode {
                // Halt
                0 => self.halted = true,
                // Add
                1 => self.accumulator += self.memory[operand as usize],
                // Sub
                2 => self.accumulator -= self.memory[operand as usize],
                // Store
                3 => self.memory[operand as usize] = self.accumulator,
                4 => panic!("Unused opcode"),
                // Load
                5 => self.accumulator = self.memory[operand as usize],
                // Branch
                6 => self.pc = operand as u8,
                // Branch if zero
                7 => {
                    if self.accumulator == 0 {
                        self.pc = operand as u8
                    }
                }
                // Branch if positive
                8 => {
                    if self.accumulator >= 0 {
                        self.pc = operand as u8
                    }
                }
                9 => match operand {
                    1 => self.accumulator = self.input_provider.input(),
                    2 => self.input_provider.output(self.accumulator),
                    _ => panic!("Invalid instruction"),
                },
                _ => unreachable!(),
            }
        }
    }
}

use std::io::{stdin, stdout, Stdin, Stdout, Write};

pub struct IO {
    stdin: Stdin,
    stdout: Stdout,
}

impl IO {
    pub fn new() -> Self {
        Self {
            stdin: stdin(),
            stdout: stdout(),
        }
    }
}

impl InputProvider for IO {
    fn input(&mut self) -> i16 {
        let mut s = String::new();

        write!(self.stdout, "IN: ").unwrap();
        self.stdout.flush().unwrap();

        self.stdin
            .read_line(&mut s)
            .expect("Did not enter a correct string");

        let s = s.strip_suffix('\n').unwrap();

        s.parse().expect("Expected a number")
    }

    fn output(&mut self, data: i16) {
        write!(self.stdout, "OUT: {}\n", data).unwrap();
    }
}
