use std::collections::HashMap;
use std::fs;
use std::env;
use pathfinding::prelude::astar;

use crate::keypad::{Keypad, Key, FinalKeypad, RobotKeypad};
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
            // //part1
            // let keypad = RobotKeypad::controlling(
            //     RobotKeypad::<FinalKeypad, NumericKey>::controlling(
            //         FinalKeypad::new()
            //     )
            // );
            //part2
            let keypad = RobotKeypad::controlling(
                RobotKeypad::controlling(
                    RobotKeypad::controlling(
                        RobotKeypad::controlling(
                            RobotKeypad::controlling(
                                RobotKeypad::controlling(
                                    RobotKeypad::controlling(
                                        RobotKeypad::controlling(
                                            RobotKeypad::controlling(
                                                RobotKeypad::controlling(
                                                    RobotKeypad::controlling(
                                                        RobotKeypad::controlling(
                                                            RobotKeypad::controlling(
                                                                RobotKeypad::controlling(
                                                                    RobotKeypad::controlling(
                                                                        RobotKeypad::controlling(
                                                                            RobotKeypad::controlling(
                                                                                RobotKeypad::controlling(
                                                                                    RobotKeypad::controlling(
                                                                                        RobotKeypad::controlling(
                                                                                            RobotKeypad::controlling(
                                                                                                RobotKeypad::controlling(
                                                                                                    RobotKeypad::controlling(
                                                                                                        RobotKeypad::controlling(
                                                                                                            RobotKeypad::<FinalKeypad, NumericKey>::controlling(
                                                                                                                FinalKeypad::new()
                                                                                                            )
                                                                                                        )
                                                                                                    )
                                                                                                )
                                                                                            )
                                                                                        )
                                                                                    )
                                                                                )
                                                                            )
                                                                        )
                                                                    )
                                                                )
                                                            )
                                                        )
                                                    )
                                                )
                                            )
                                        )
                                    )
                                )
                            )
                        )
                    )
                )
            );
            let shortest = shortest_path_to_code(keypad, &code);
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

fn shortest_path_to_code<KP: Keypad<K>, K: Key>(start: RobotKeypad<KP, K>, code: &Code<NumericKey>) -> usize {
    let mut shortest = 0;
    let mut state = start;
    let mut cache = HashMap::new();
    for nk in &code.keys {
        println!("Finding path to {:?}", nk);
        let (final_state, length) = shortest_path_to_key(&state, nk, &mut cache);
        println!("Shortest: {}", length);
        state = final_state;
        shortest += length;
        shortest += 1; // press activate
    }
    shortest
}

fn shortest_path_to_key<KP: Keypad<K>, K: Key>(start: &RobotKeypad<KP, K>, key: &NumericKey, cache: &mut HashMap<(RobotKeypad<KP, K>, NumericKey),(RobotKeypad<KP, K>, usize)>) -> (RobotKeypad<KP, K>, usize) {
    let cache_key = (start.clone(), *key);
    if let Some(cached) = cache.get(&cache_key) {
        cached.clone()
    } else {
        let (result, length) = astar(start, |kp| kp.successors(), |kp| kp.minimum_moves_to_final_key(key), |kp| kp.ready_for_final_key(key)).expect("No solution");
        let final_state = result.into_iter().last().unwrap();
        cache.insert(cache_key, (final_state.clone(), length));
        (final_state, length)
    }
}