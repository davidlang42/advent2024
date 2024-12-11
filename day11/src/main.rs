use std::collections::HashMap;
use std::fs;
use std::env;
use std::str::FromStr;

#[derive(Debug)]
struct Stones {
    stones: Vec<usize>
}

impl FromStr for Stones {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let stones = line.split(" ").map(|s| s.parse().unwrap()).collect();
        Ok(Self {
            stones
        })
    }
}

impl Stones {
    fn blink(&mut self, blinks: usize) -> usize {
        let mut count = 0;
        let mut cache = HashMap::new();
        for stone in &self.stones {
            count += Self::count_stones(&mut cache, *stone, blinks);
        }
        count
    }

    fn count_stones(cache: &mut HashMap<(usize, usize), usize>, stone: usize, blinks: usize) -> usize {
        if blinks == 0 {
            // finished
            1
        } else if let Some(cached_result) = cache.get(&(stone, blinks)) {
            // cached
            *cached_result
        } else {
            // calculate the result of blinks
            let result = if stone == 0 {
                Self::count_stones(cache, 1, blinks - 1)
            } else {
                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    let middle = stone_str.len() / 2;
                    let left_stone = stone_str[0..middle].parse().unwrap();
                    let right_stone = stone_str[middle..stone_str.len()].parse().unwrap();
                    Self::count_stones(cache, left_stone, blinks - 1) + Self::count_stones(cache, right_stone, blinks - 1)
                } else {
                    Self::count_stones(cache, stone * 2024, blinks - 1)
                }
            };
            // cache this result
            cache.insert((stone, blinks), result);
            result
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let blinks = args[2].parse().unwrap();
        let mut stones: Stones = text.parse().unwrap();
        println!("Count after {} blinks: {}", blinks, &mut stones.blink(blinks));
    } else {
        println!("Please provide 2 arguments: Filename, Blinks");
    }
}