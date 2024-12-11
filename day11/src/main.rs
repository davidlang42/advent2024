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
    fn blink(&self, blinks: usize) -> usize {
        let mut count = 0;
        for stone in &self.stones {
            count += Self::make_stones(*stone, blinks).len();
        }
        count
    }

    fn make_stones(stone: usize, blinks: usize) -> Vec<usize> {
        if blinks == 0 {
            // finished
            vec![stone]
        } else {
            // do a blink
            if stone == 0 {
                return Self::make_stones(1, blinks - 1);
            }
            let stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                let middle = stone_str.len() / 2;
                let left_stone = stone_str[0..middle].parse().unwrap();
                let right_stone = stone_str[middle..stone_str.len()].parse().unwrap();
                let mut result = Self::make_stones(left_stone, blinks - 1);
                result.append(&mut Self::make_stones(right_stone, blinks - 1));
                return result;
            }
            Self::make_stones(stone * 2024, blinks - 1)
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
        let stones: Stones = text.parse().unwrap();
        println!("Count after {} blinks: {}", blinks, stones.blink(blinks));
    } else {
        println!("Please provide 2 arguments: Filename, Blinks");
    }
}