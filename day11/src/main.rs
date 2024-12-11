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
    fn blink(&mut self) {
        let mut i = 0;
        while i < self.stones.len() {
            if self.stones[i] == 0 {
                self.stones[i] = 1;
                i += 1;
                continue;
            }
            let stone_str = self.stones[i].to_string();
            if stone_str.len() % 2 == 0 {
                let middle = stone_str.len() / 2;
                self.stones[i] = stone_str[0..middle].parse().unwrap();
                self.stones.insert(i + 1, stone_str[middle..stone_str.len()].parse().unwrap());
                i += 2;
                continue;
            }
            self.stones[i] *= 2024;
            i += 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut stones: Stones = text.parse().unwrap();
        for _i in 0..25 {
            stones.blink();
        }
        println!("Count after 25: {}", stones.stones.len());
        for i in 0..50 {
            stones.blink();
            println!("Count after {}: {}", i + 26, stones.stones.len());
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}