use std::collections::HashMap;
use std::fs;
use std::env;
use directional::DirectionalKey;
use keypad::FinalKeypad;
use keypad::RobotKeypad;
use pathfinding::prelude::{astar_bag, bfs};

use crate::keypad::{Keypad, Key};
use crate::code::Code;
use crate::numeric::NumericKey;

mod keypad;
mod code;
mod numeric;
mod directional;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let codes: Vec<Code<NumericKey>> = text.lines().map(|s| s.parse().unwrap()).collect();
        let mut sum = 0;
        for code in codes {
            let keypad = RobotKeypad::controlling(
                RobotKeypad::<FinalKeypad, NumericKey>::controlling(
                    FinalKeypad::new()
                )
            );
            let shortest = shortest_path_to_code(&keypad, &code);
            let numeric_part = code.numeric_part();
            let complexity = numeric_part * shortest;
            println!("Code: {}, Shortest: {}, Complexity: {}", code, shortest, complexity);
            sum += complexity;
        }
        println!("Sum: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn shortest_path_to_code<KP: Keypad<K>, K: Key>(start: &RobotKeypad<KP, K>, code: &Code<NumericKey>) -> usize {
    let mut shortest = 0;
    for nk in &code.keys {
        shortest += shortest_path_to_key(start, nk);
        // activate?
    }
    shortest
}

fn shortest_path_to_key<KP: Keypad<K>, K: Key>(start: &RobotKeypad<KP, K>, key: &NumericKey) -> usize {
    let result = bfs(start, |kp| kp.successors(), |kp| kp.final_key() == *key).expect("No solution");
    result.len() - 1
}