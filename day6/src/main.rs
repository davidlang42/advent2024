use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::HashSet;

struct Map {
    obstacles: Vec<Vec<bool>>,
    max: Pos,
    guard: Pos,
    facing: Direction
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Pos {
    row: usize,
    col: usize
}

enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl FromStr for Map {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let obstacles: Vec<Vec<bool>> = text.lines().map(|l| l.chars().map(|c| c == '#').collect()).collect();
        let max = Pos {
            row: obstacles.len() - 1,
            col: obstacles[0].len() - 1
        };
        let row = text.lines().position(|l| l.chars().position(|c| c == '^').is_some()).unwrap();
        let col = text.lines().nth(row).unwrap().chars().position(|c| c == '^').unwrap();
        Ok(Self {
            obstacles,
            max,
            guard: Pos {
                row,
                col
            },
            facing: Direction::Up
        })
    }
}

impl Pos {
    fn move_forward(&self, direction: &Direction, max: &Self) -> Option<Self> {
        match direction {
            Direction::Up => {
                if self.row == 0 {
                    None
                } else {
                    Some(Self {
                        row: self.row - 1,
                        col: self.col
                    })
                }
            },
            Direction::Right => {
                if self.col == max.col {
                    None
                } else {
                    Some(Self {
                        row: self.row,
                        col: self.col + 1
                    })
                }
            },
            Direction::Down => {
                if self.row == max.row {
                    None
                } else {
                    Some(Self {
                        row: self.row + 1,
                        col: self.col
                    })
                }
            },
            Direction::Left => {
                if self.col == 0 {
                    None
                } else {
                    Some(Self {
                        row: self.row,
                        col: self.col - 1
                    })
                }
            },
        }
    }
}

impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

impl Map {
    pub fn move_guard(&mut self) -> bool {
        if let Some(in_front) = self.guard.move_forward(&self.facing, &self.max) {
            if self.obstacles[in_front.row][in_front.col] {
                // obstactle in front
                self.facing = self.facing.rotate();
            } else {
                // empty in front
                self.guard = in_front;
            }
            true
        } else {
            // left the area
            false
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut map: Map = text.parse().unwrap();
        let mut positions = HashSet::new();
        positions.insert(map.guard.clone());
        while map.move_guard() {
            positions.insert(map.guard.clone());
        }
        println!("Discreet positions: {}", positions.len());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}