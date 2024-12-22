use std::fs;
use std::env;
use crate::keypad::{Keypad, Key};
use crate::code::Code;
use crate::numeric::NumericKey;
use crate::directional::DirectionalKey;

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
        for code1 in codes {
            let results1 = solve_for(&code1, "1");
            for r1 in results1 {
                let code2 = Code { keys: r1.movements.clone() };
                let results2 = solve_for(&code2, "2");
                for r2 in results2 {
                    let code3 = Code { keys: r2.movements.clone() };
                    solve_for(&code3, "3");
                }
            }
            panic!();
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn solve_for<K: Key>(code: &Code<K>, log_description: &str) -> Vec<Keypad<K>> {
    println!("[{}] Code: {}", log_description, code);
    let mut results = Vec::new();
    let start = Keypad::<K>::new();
    for result in start.shortest_paths_to_code(&code) {
        if result.movements_string() == "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A" {
            panic!("GOT IT");
        }
        results.push(result);
    }
    // filter out results which are no longer the shortest (due to combining with upstream results)
    let shortest = results.iter().map(|r| r.movements.len()).min().unwrap();
    results = results.into_iter().filter(|r| r.movements.len() == shortest).collect();
    // print shortest
    for result in &results {
        println!("[{}] Shortest ({}): {}", log_description, result.movements.len(), result.movements_string());
    }
    results
}