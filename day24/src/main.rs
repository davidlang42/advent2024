use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::HashMap;

type Wire = String;

#[derive(Debug)]
struct Logic {
    calculations: HashMap<Wire, Gate>,
    values: HashMap<Wire, bool>
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut logic: Logic = text.parse().unwrap();
        logic.calculate();
        println!("Logic: {}", logic.binary("z"));
    } else {
        println!("Please provide 1 argument: Filename");
    }
}