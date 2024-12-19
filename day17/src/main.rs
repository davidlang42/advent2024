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

    fn output_matches_program(&self) -> bool {
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

    fn number_of_outputs_matching_program_from_end(&self) -> usize {
        let mut count = 0;
        for i in (0..self.output.len()).rev() {
            if self.output[i] == self.instructions[i] as usize {
                count += 1;
            } else {
                break;
            }
        }
        count
    }

    fn simulate_seed(&self, initial_register_a: usize) -> Computer {
        let mut pc = self.clone();
        pc.register_a = initial_register_a;
        while pc.run_next() { }
        pc
    }

    fn simulate_fast_output_matches_program(&mut self, initial_register_a: usize) -> bool {
        self.register_a = initial_register_a;
        self.register_b = 0;
        self.register_c = 0;
        self.pointer = 0;
        self.output.clear();
        while self.run_next() {
            if self.output.len() > self.instructions.len() {
                return false;
            }
            for i in 0..self.output.len() {
                if self.output[i] != self.instructions[i] as usize {
                    return false;
                }
            }
        }
        self.output_matches_program()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut original: Computer = text.parse().unwrap();
        // part1
        let mut pc = original.clone();
        println!("{:?}", pc);
        while pc.run_next() {
            println!("{:?}", pc);
        }
        println!("Output: {:?}", pc.output);
        // part2
        let mut seed = 1;
        // find any seed which gets the correct output length
        loop {
            let pc = original.simulate_seed(seed);
            if pc.output.len() < original.instructions.len() {
                seed *= 10;
            } else {
                break;
            }
        }
        println!("Seed {} achieves output length {}", seed, original.instructions.len());
        // find the min seed which gets correct output length
        let mut min = bisect(&original, seed/10, seed, &|pc: &Computer| pc.output.len() == original.instructions.len());
        println!("Min seed {} achieves output length {}", min, original.instructions.len());
        // find any seed which gets too much output
        loop {
            let pc = original.simulate_seed(seed);
            if pc.output.len() <= original.instructions.len() {
                seed *= 10;
            } else {
                break;
            }
        }
        println!("Seed {} gets output greater than {}", seed, original.instructions.len());
        // find the max seed which gets correct output length
        let mut max = bisect(&original, seed/10, seed, &|pc: &Computer| pc.output.len() > original.instructions.len()) - 1;
        println!("Max seed {} achieves output length {}", max, original.instructions.len());
        // try all
        for seed in min..(max+1) {
            if original.simulate_fast_output_matches_program(seed) {
                println!("Answer: {}", seed);
                break;
            }
            if seed.rem_euclid(10000000) == 0 {
                println!("{}", seed);
            }
        }

        // // iterate over the output numbers in reverse getting each place right in order
        // for i in 1..(original.instructions.len() + 1) {
        //     println!("Before #{}: MIN {:?}", i, original.simulate_seed(min));
        //     min = bisect(&original, min, max, &|pc: &Computer| pc.number_of_outputs_matching_program_from_end() >= i);
        //     println!("After  #{}: MIN {:?}", i, original.simulate_seed(min));
        //     println!("Before #{}: MAX {:?}", i, original.simulate_seed(max));
        //     max = bisect(&original, min + 1, max + 1, &|pc: &Computer| pc.output[i] != original.instructions[i] as usize) - 1;
        //     println!("After  #{}: MAX {:?}", i, original.simulate_seed(max));
        //     if min == max - 1 {
        //         println!("Answer: {}", max);
        //     }
        //     if min >= max {
        //         panic!();
        //     }
        // }
        // println!("Answer: {}-{}", min, max);
    } else if args.len() == 3 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let original: Computer = text.parse().unwrap();
        let seed = args[2].parse().unwrap();
        let pc = original.simulate_seed(seed);
        println!("Seed {}:", seed);
        println!("  output {:?}", pc.output);
        println!("expected {:?}", pc.instructions);
    } else {
        println!("Please provide 1/2 argument(s): Filename Seed");
    }
}

fn bisect(original: &Computer, mut min: usize, mut max: usize, condition: &dyn Fn(&Computer) -> bool) -> usize {
    let min_pc = original.simulate_seed(min);
    if condition(&min_pc) {
        panic!("min already passed condition before we started");
    }
    let max_pc = original.simulate_seed(max);
    if !condition(&max_pc) {
        panic!("max didnt pass condition before we started");
    }
    loop {
        let mid = (min + max) / 2;
        let pc = original.simulate_seed(mid);
        if condition(&pc) {
            max = mid;
        } else {
            min = mid;
        }
        if min == max - 1 {
            let min_pc = original.simulate_seed(min);
            if condition(&min_pc) {
                panic!("result failed");
            }
            let max_pc = original.simulate_seed(max);
            if !condition(&max_pc) {
                panic!("result failed");
            }
            return max;
        }
        if min >= max {
            panic!();
        }
    }
}