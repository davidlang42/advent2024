use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fmt::Formatter;

type Wire = String;

#[derive(Debug, Clone)]
struct Logic {
    calculations: HashMap<Wire, Gate>,
    values: HashMap<Wire, bool>
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Expression {
    operation: Operation,
    input_a: Input,
    input_b: Input
}

#[derive(Clone, Hash, Eq, PartialEq)]
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
        keys.sort();
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

    fn dependants_of(&self, wire: &Wire) -> Vec<Wire> {
        let mut v = Vec::new();
        if let Some(gate) = &self.calculations.get(wire) {
            v.push(gate.input_a.clone());
            for a in self.dependants_of(&gate.input_a) {
                v.push(a);
            }
            v.push(gate.input_b.clone());
            for b in self.dependants_of(&gate.input_b) {
                v.push(b);
            }
        }
        v
    }

    fn swap(&mut self, a: &Wire, b: &Wire) {
        let a_gate = self.calculations.remove(a).unwrap();
        let b_gate = self.calculations.remove(b).unwrap();
        self.calculations.insert(a.clone(), b_gate);
        self.calculations.insert(b.clone(), a_gate);
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

    fn depends_on(&self) -> HashSet<Input> {
        let mut set = self.input_a.depends_on();
        for input in self.input_b.depends_on() {
            set.insert(input);
        }
        set
    }

    fn valid_for_addition(&self, digit: usize) -> bool {
        if !self.depends_on().iter().all(|input| match input {
            Input::X(x) => *x <= digit,
            Input::Y(y) => *y <= digit,
            _ => panic!()
        }) {
            return false; //invalid dependants
        }
        let expected_depth = if digit == 0 {
            1
        } else {
            digit * 2
        };
        if self.depth() != expected_depth {
            return false; //invalid depth
        }
        true
        //TODO further: could derive each one
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

    fn depends_on(&self) -> HashSet<Input> {
        if let Self::Exp(e) = &self {
            e.depends_on()
        } else {
            let mut set = HashSet::new();
            set.insert(self.clone());
            set
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let logic: Logic = text.parse().unwrap();
        let mut part1 = logic.clone();
        part1.calculate();
        println!("Part1: {}", part1.binary("z"));
        let exp = logic.simplify();
        for e in 0..exp.len() {
            //println!("[{}] z{} = {}", exp[e].depth(), e, exp[e]);

            // let depends: Vec<Input> = exp[e].depends_on().into_iter().collect();
            // print!("z{} depends on [ ", e);
            // for d in depends {
            //     print!("{} ", d)
            // }
            // println!("]");

            if exp[e].valid_for_addition(e) {
                println!("z{} is valid", e);
            } else {
                println!("z{} is NOT valid", e);
                let key = format!("z0{}", e);
                for d in logic.dependants_of(&key) {
                    for swap_with in logic.calculations.keys() {
                        if d == *swap_with {
                            continue; // dont swap with yourself
                        }
                        if !logic.calculations.contains_key(&d) {
                            continue; // dont swap with things that already have values (rather than calcs)
                        }
                        let mut clone = logic.clone();
                        //println!("Trying to swap {} with {}", d, swap_with);
                        clone.swap(&d, swap_with);
                        let exp2 = logic.simplify();
                        for e2 in 0..exp.len() {
                            if exp2[e2].valid_for_addition(e2) {
                                if e2 >= e {
                                    println!("With {}<->{}, z{} is valid", d,swap_with,e2);
                                }
                            } else {
                                if e2 > e {
                                    println!("With {}<->{}, z{} is NOT valid", d, swap_with,e2);
                                }
                                break;
                            }
                        }
                    }
                }
                panic!();
            }
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}