use std::fs;
use std::env;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Computer {
    instructions: Vec<u8>,
    pointer: usize,
    register_a: usize,
    register_b: usize,
    register_c: usize,
    output: Vec<usize>
}

impl FromStr for Computer {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let sections:Vec<_> = text.split("\r\n\r\n").collect();
        if sections.len() != 2 {
            panic!("Invalid number of sections");
        }
        let registers: Vec<usize> = sections[0].lines().map(|l| l.split(": ").nth(1).unwrap().parse().unwrap()).collect();
        if registers.len() != 3 {
            panic!("Invalid number of registers");
        }
        let instructions = sections[1].split(": ").nth(1).unwrap().split(",").map(|n| n.parse().unwrap()).collect();
        Ok(Self {
            register_a: registers[0],
            register_b: registers[1],
            register_c: registers[2],
            instructions,
            pointer: 0,
            output: Vec::new()
        })
    }
}

enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv
}

impl From<u8> for Instruction {
    fn from(n: u8) -> Self {
        match n {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid instruction")
        }
    }
}

impl Computer {
    fn run_next(&mut self) -> bool {
        if self.pointer >= self.instructions.len() {
            // halted
            return false;
        }
        let instruction: Instruction = self.instructions[self.pointer].into();
        self.pointer += 1;
        let operand = self.instructions[self.pointer];
        self.pointer += 1;
        match instruction {
            Instruction::Adv => self.register_a = self.register_a / 2_usize.pow(self.read_combo(operand).try_into().unwrap()),
            Instruction::Bxl => self.register_b = self.register_b ^ operand as usize,
            Instruction::Bst => self.register_b = self.read_combo(operand).rem_euclid(8),
            Instruction::Jnz => {
                if self.register_a != 0 {
                    self.pointer = operand as usize;
                }
            },
            Instruction::Bxc => self.register_b = self.register_b ^ self.register_c, // ignores operand
            Instruction::Out => self.output.push(self.read_combo(operand).rem_euclid(8)),
            Instruction::Bdv => self.register_b = self.register_a / 2_usize.pow(self.read_combo(operand).try_into().unwrap()),
            Instruction::Cdv => self.register_c = self.register_a / 2_usize.pow(self.read_combo(operand).try_into().unwrap())
        }
        self.pointer < self.instructions.len()
    }

    fn read_combo(&self, operand: u8) -> usize {
        if operand <= 3 {
            operand as usize
        } else if operand == 4 {
            self.register_a
        } else if operand == 5 {
            self.register_b
        } else if operand == 6 {
            self.register_c
        } else {
            panic!("Invalid combo operand")
        }
    }

    fn output_matches_program_start(&self) -> bool {
        if self.output.len() > self.instructions.len() {
            return false;
        }
        for i in 0..self.output.len() {
            if self.output[i] != self.instructions[i] as usize {
                return false;
            }
        }
        true
    }

    fn output_matches_program_full(&self) -> bool {
        if self.output.len() != self.instructions.len() {
            return false;
        }
        for i in 0..self.output.len() {
            if self.output[i] != self.instructions[i] as usize {
                return false;
            }
        }
        true
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let original: Computer = text.parse().unwrap();
        // part1
        let mut pc = original.clone();
        println!("{:?}", pc);
        while pc.run_next() {
            println!("{:?}", pc);
        }
        println!("Output: {:?}", pc.output);
        // part2
        let mut seed = 100000000000000; // to match number of digits of output required
        loop {
            pc = original.clone();
            pc.register_a = seed;
            while pc.run_next() {
                if !pc.output_matches_program_start() {
                    break;
                }
            }
            // if pc.output.len() < pc.instructions.len() {
            //     println!("Seed {} generated {} out, needed {}, try starting with seed: {}", seed, pc.output.len(), pc.instructions.len(), seed* 10);
            //     return;
            // }
            if pc.output_matches_program_full() {
                break;
            }
            if seed.rem_euclid(10000000) == 0 {
                println!("{}", seed);
            }
            seed += 1;
        }
        println!("Seed: {}", seed);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}