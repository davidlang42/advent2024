use std::fs;
use std::env;
use std::str::FromStr;
use pathfinding::prelude::bfs;
use std::fmt::Display;
use std::fmt::Formatter;

struct Memory {
    corrupted: Vec<Vec<bool>>,
    size: usize
}

impl Display for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in 0..self.size {
            for col in 0..self.size {
                if self.corrupted[row][col] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Memory {
    fn new(size: usize) -> Self {
        let mut corrupted = Vec::new();
        for _i in 0..size {
            let mut row = Vec::new();
            for _j in 0..size {
                row.push(false);
            }
            corrupted.push(row);
        }
        Self {
            corrupted, size
        }
    }

    fn corrupt(&mut self, pos: &Pos) {
        self.corrupted[pos.row][pos.col] = true;
    }

    fn is_available(&self, pos: &Pos) -> bool {
        if pos.row >= self.size || pos.col >= self.size {
            false
        } else {
            !self.corrupted[pos.row][pos.col]
        }
    }

    fn available_adjacent_to(&self, pos: &Pos) -> Vec<Pos> {
        let mut v = Vec::new();
        if pos.row > 0 {
            let p = Pos {
                row: pos.row - 1,
                col: pos.col
            };
            if self.is_available(&p) {
                v.push(p);
            }
        }
        if pos.col > 0 {
            let p = Pos {
                row: pos.row,
                col: pos.col - 1
            };
            if self.is_available(&p) {
                v.push(p);
            }
        }
        let p = Pos {
            row: pos.row + 1,
            col: pos.col
        };
        if self.is_available(&p) {
            v.push(p);
        }
        let p = Pos {
            row: pos.row,
            col: pos.col + 1
        };
        if self.is_available(&p) {
            v.push(p);
        }
        v
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    row: usize,
    col: usize
}

impl FromStr for Pos {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // 0,4 => row:4, col:0
        let numbers: Vec<usize> = line.split(",").map(|s| s.parse().unwrap()).collect();
        if numbers.len() != 2 {
            panic!("Invalid number count");
        }
        Ok(Self {
            row: numbers[1],
            col: numbers[0]
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let bytes: Vec<Pos> = text.lines().map(|s| s.parse().unwrap()).collect();
        let mut m = Memory::new(71); // 7 for test
        for i in 0..1024 { // 12 for test
            if i == bytes.len() {
                println!("Ran out of bytes");
                break;
            }
            m.corrupt(&bytes[i]);
        }
        let start = Pos {
            row: 0,
            col: 0
        };
        let end = Pos {
            row: m.size - 1,
            col: m.size - 1
        };
        let result = bfs(&start, |p| m.available_adjacent_to(p), |p| *p == end);
        println!("{}", m);
        println!("Answer: {}", result.expect("no path found").len() - 1);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}