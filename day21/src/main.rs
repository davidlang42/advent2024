use std::fs;
use std::env;
use crate::keypad::Keypad;
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
        let start = Keypad::<NumericKey>::new();
        for code in codes {
            println!("Code: {}", code);
            let result = start.shortest_path_to_code(&code);
            let presses = result.presses_string();
            println!("Shortest ({}): {}", presses.len(), presses);
            println!("Final: {:?}", result);
            panic!();
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}