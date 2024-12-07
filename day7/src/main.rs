use std::fs;
use std::env;
use std::str::FromStr;

#[derive(Clone)]
struct Equation {
    answer: usize,
    operands: Vec<usize>
}

impl FromStr for Equation {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let sections: Vec<_> = line.split(": ").collect();
        if sections.len() != 2 {
            panic!("Incorect number of sections")
        }
        let answer = sections[0].parse().unwrap();
        let operands = sections[1].split(" ").map(|n| n.parse().unwrap()).collect();
        Ok(Self {
            answer,
            operands
        })
    }
}

#[derive(Clone)]
enum Operation {
    Plus,
    Multiply,
    Concat
}

impl Equation {
    pub fn solveable(&self) -> bool {
        self.solve(self.operands[0], Vec::new()).is_some()
    }

    fn solve(&self, so_far: usize, operations: Vec<Operation>) -> Option<Vec<Operation>> {
        if operations.len() == self.operands.len() - 1 {
            if so_far == self.answer {
                // solved
                Some(operations)
            } else {
                None
            }
        } else {
            let next_number = self.operands[operations.len() + 1];
            // try plus
            let mut operations_with_plus = operations.clone();
            operations_with_plus.push(Operation::Plus);
            if let Some(solution) = self.solve(so_far + next_number, operations_with_plus) {
                return Some(solution);
            }
            // try multiply
            let mut operations_with_multiply = operations.clone();
            operations_with_multiply.push(Operation::Multiply);
            if let Some(solution) = self.solve(so_far * next_number, operations_with_multiply) {
                return Some(solution);
            }
            // try concat
            let mut operations_with_concat = operations;
            operations_with_concat.push(Operation::Concat);
            if let Some(solution) = self.solve(Self::concat(so_far, next_number), operations_with_concat) {
                return Some(solution);
            }
            None
        }
    }

    fn concat(a: usize, b: usize) -> usize {
        let a_s: String = a.to_string();
        let b_s: String = b.to_string();
        let ab_s: String = a_s + &b_s;
        let ab: usize = ab_s.parse().unwrap();
        ab
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let equations: Vec<Equation> = text.lines().map(|s| s.parse().unwrap()).collect();
        let mut answer_sum = 0;
        for e in equations {
            if e.solveable() {
                answer_sum += e.answer;
            }
        }
        println!("Answer Sum: {}", answer_sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}