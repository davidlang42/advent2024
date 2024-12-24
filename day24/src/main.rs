use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;

type Wire = String;

#[derive(Debug)]
struct Logic {
    calculations: HashMap<Wire, Gate>,
    values: HashMap<Wire, bool>
}

struct Expression {
    operation: Operation,
    input_a: Input,
    input_b: Input
}

enum Input {
    Exp(Box<Expression>),
    X(usize),
    Y(usize)
}

#[derive(Debug, Clone)]
struct Gate {
    operation: Operation,
    input_a: Wire,
    input_b: Wire
}

#[derive(Debug, Clone)]
enum Operation {
    And,
    Or,
    Xor
}

impl FromStr for Logic {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let sections: Vec<_> = text.split("\n\n").collect();
        if sections.len() != 2 {
            panic!();
        }
        let mut values = HashMap::new();
        for line in sections[0].lines() {
            let sub: Vec<_> = line.split(": ").collect();
            if sub.len() != 2 {
                panic!();
            }
            let b = sub[1] == "1";
            values.insert(sub[0].to_string(), b);
        }
        let mut calculations = HashMap::new();
        for line in sections[1].lines() {
            let sub: Vec<_> = line.split(" ").collect(); //x02 OR y02 -> z02
            if sub.len() != 5 {
                panic!();
            }
            let input_a = sub[0].to_string();
            let input_b = sub[2].to_string();
            let output = sub[4].to_string();
            let operation = match sub[1] {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => panic!()
            };
            calculations.insert(output, Gate {
                input_a,
                input_b,
                operation
            });
        }
        Ok(Self {
            values,
            calculations
        })
    }
}

impl Logic {
    fn calculate(&mut self) {
        while self.calculations.len() > 0 {
            let (output, calc) = self.next_ready_calculation();
            let value = calc.calculate(&self.values);
            self.calculations.remove(&output);
            self.values.insert(output, value);
        }
    }

    fn next_ready_calculation(&self) -> (Wire, Gate) {
        for (wire, gate) in self.calculations.iter() {
            if gate.all_inputs_ready(&self.values) {
                return (wire.clone(), gate.clone());
            }
        }
        panic!();
    }

    fn binary(&self, starts_with: &str) -> String {
        let mut s = String::new();
        let mut keys: Vec<_> = self.values.keys().filter(|k| k.starts_with(starts_with)).collect();
        keys.sort_by(|a,b| b.cmp(&a));
        for k in keys {
            s.push(if *self.values.get(k).unwrap() { '1' } else { '0' });
        }
        s
    }

    fn simplify(&self) -> Vec<Expression> {
        let mut exp = Vec::new();
        let mut keys: Vec<_> = self.calculations.keys().filter(|k| k.starts_with("z")).collect();
        keys.sort_by(|a,b| b.cmp(&a));
        for k in keys {
            if let Input::Exp(e) = self.expression_for(k) {
                exp.push(*e);
            } else {
                panic!();
            }
        }
        exp
    }

    fn expression_for(&self, key: &Wire) -> Input {
        if key.starts_with("x") {
            Input::X(key[1..].parse().unwrap())
        } else if key.starts_with("y") {
            Input::Y(key[1..].parse().unwrap())
        } else {
            let calc = self.calculations.get(key).unwrap();
            let exp = Expression {
                operation: calc.operation.clone(),
                input_a: self.expression_for(&calc.input_a),
                input_b: self.expression_for(&calc.input_b)
            };
            Input::Exp(Box::new(exp))
        }
    }
}

impl Gate {
    fn all_inputs_ready(&self, inputs: &HashMap<Wire,bool>) -> bool {
        inputs.contains_key(&self.input_a) && inputs.contains_key(&self.input_b)
    }

    fn calculate(&self, inputs: &HashMap<Wire, bool>) -> bool {
        let a = *inputs.get(&self.input_a).unwrap();
        let b = *inputs.get(&self.input_b).unwrap();
        match self.operation {
            Operation::And => a && b,
            Operation::Or => a || b,
            Operation::Xor => a ^ b
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} {}", self.input_a, self.operation, self.input_b)
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            Self::And => "&&",
            Self::Or => "||",
            Self::Xor => "=="
        })
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::X(x) => write!(f, "x{}", x),
            Self::Y(y) => write!(f, "y{}", y),
            Self::Exp(e) => write!(f, "({})", e)
        }
    }
}

impl Expression {
    fn depth(&self) -> usize {
        self.input_a.depth().max(self.input_b.depth())
    }
}

impl Input {
    fn depth(&self) -> usize {
        if let Self::Exp(e) = &self {
            e.depth() + 1
        } else {
            1
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut logic: Logic = text.parse().unwrap();
        let exp = logic.simplify();
        logic.calculate();
        println!("Part1: {}", logic.binary("z"));
        for e in 0..exp.len() {
            println!("[{}] z{} = {}", exp[e].depth(), e, exp[e]);
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}