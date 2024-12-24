use std::collections::HashMap;
use std::fs;
use std::env;
use directional::DirectionalKey;
use keypad::FinalKeypad;
use keypad::RobotKeypad;

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
                RobotKeypad::controlling(
                    FinalKeypad::new()
                )
            );
            let shortest = keypad.shortest_path_to_code(&code);
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

fn robot_indirection_numeric(code: &Code<NumericKey>, layers_of_indirection: usize) -> Vec<Vec<DirectionalKey>> {
    if layers_of_indirection == 0 {
        solve_for(&code, &mut HashMap::new())
    } else {
        let mut final_results = Vec::new();
        let results = solve_for(code, &mut HashMap::new());
        let mut cache = HashMap::new();
        for r in results {
            let inner_code = Code { keys: r };
            final_results.append(&mut robot_indirection_directional(&inner_code, layers_of_indirection - 1, &mut cache));
        }
        final_results
    }
}

fn robot_indirection_directional(code: &Code<DirectionalKey>, layers_of_indirection: usize, cache: &mut HashMap<(DirectionalKey, Vec<DirectionalKey>), Vec<Keypad<DirectionalKey>>>) -> Vec<Vec<DirectionalKey>> {
    //println!("Starting layers remaining: {}", layers_of_indirection);
    if layers_of_indirection == 0 {
        solve_for(&code, cache)
    } else {
        let mut final_results = Vec::new();
        let results = solve_for(code, cache);
        for r in results {
            let inner_code = Code { keys: r };
            final_results.append(&mut robot_indirection_directional(&inner_code, layers_of_indirection - 1, cache));
        }
        final_results
    }
}

fn solve_for<K: Key>(code: &Code<K>, cache: &mut HashMap<(K, Vec<K>), Vec<Keypad<K>>>) -> Vec<Vec<DirectionalKey>> {
    let start = Keypad::<K>::new();
    let results = start.shortest_paths_to_code(&code, cache);
    results.into_iter().map(|r| r.movements).collect::<Vec<Vec<DirectionalKey>>>()
}