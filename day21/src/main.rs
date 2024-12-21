use std::fs;
use std::env;
use std::str::FromStr;
use std::fmt::Display;
use std::fmt::Formatter;
use pathfinding::prelude::bfs;
use crate::keypad::{Key, Keypad};
use crate::numeric::{NumericKeypad, NumericKey};
use crate::directional::DirectionalKeypad;

mod keypad;
mod numeric;
mod directional;

struct Code {
    keys: Vec<NumericKey>
}

impl FromStr for Code {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let keys = line.chars().map(|c| NumericKey::from_char(c)).collect();
        Ok(Self {
            keys
        })
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for k in &self.keys {
            write!(f, "{}", k.to_char())?
        }
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let codes: Vec<Code> = text.lines().map(|s| s.parse().unwrap()).collect();
        let start = DirectionalKeypad::new(DirectionalKeypad::new(NumericKeypad::new()));
        for code in codes {
            println!("Code: {}", code);
            let result = bfs(&start, |dk| dk.available_options(&code.keys), |dk| *dk.underlying_code() == code.keys).expect("No solution");
            println!("Result ({}): {}", result.len() - 1, result.last().unwrap().press_string());
            panic!();
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}