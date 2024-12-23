use std::collections::HashMap;
use std::fs;
use std::env;
use directional::DirectionalKey;

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
            let results = robot_indirection(&code, 2);
            let shortest: usize = results.iter().map(|r| r.len()).min().unwrap();
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

fn robot_indirection<K: Key>(code: &Code<K>, layers_of_indirection: usize) -> Vec<Vec<DirectionalKey>> {
    if layers_of_indirection == 0 {
        solve_for(&code)
    } else {
        let mut final_results = Vec::new();
        let results = solve_for(code);
        for r in results {
            let inner_code = Code { keys: r };
            final_results.append(&mut robot_indirection(&inner_code, layers_of_indirection - 1));
        }
        final_results
    }
}

fn solve_for<K: Key>(code: &Code<K>) -> Vec<Vec<DirectionalKey>> {
    let start = Keypad::<K>::new();
    let mut cache = HashMap::new();//TODO persist cache
    let results = start.shortest_paths_to_code(&code, &mut cache);
    // filter out results which are no longer the shortest (due to combining with upstream results)
    let shortest = results.iter().map(|r| r.movements.len()).min().unwrap();
    results.into_iter().filter(|r| r.movements.len() == shortest).map(|r| r.movements).collect::<Vec<Vec<DirectionalKey>>>()
}