use std::fs;
use std::env;
use std::str::FromStr;
use pathfinding::prelude::bfs;
use std::fmt::Display;
use std::fmt::Formatter;
use crate::numeric::{NumericKeypad, NumericKey};

mod numeric;

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

#[derive(Clone, Hash, Eq, PartialEq)]
struct FirstRobot {
    keypad: NumericKeypad
}

impl FirstRobot {
    fn new() -> Self {
        Self {
            keypad: NumericKeypad::new()
        }
    }
}

impl FirstRobot {
    fn next_options(&self) -> Vec<FirstRobot> {
        let mut v = vec![Self {
            keypad: self.keypad.press_current()
        }];
        if let Some(keypad) = self.keypad.move_up() {
            v.push(Self { keypad });
        }
        if let Some(keypad) = self.keypad.move_down() {
            v.push(Self { keypad });
        }
        if let Some(keypad) = self.keypad.move_left() {
            v.push(Self { keypad });
        }
        if let Some(keypad) = self.keypad.move_right() {
            v.push(Self { keypad });
        }
        v
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let codes: Vec<Code> = text.lines().map(|s| s.parse().unwrap()).collect();
        let start = FirstRobot::new();
        for code in codes {
            println!("Code: {}", code);
            let result = bfs(&start, |r| r.next_options(), |r| r.keypad.presses == code.keys).expect("No solution");
            println!("Shortest path: {}", result.len() - 1);
            panic!();
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}