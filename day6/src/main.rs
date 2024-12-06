use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Clone)]
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

#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct State(Pos, Direction);

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
                self.move_guard()
            } else {
                // empty in front
                self.guard = in_front;
                true
            }
        } else {
            // left the area
            false
        }
    }

    pub fn ends_in_loop(&mut self) -> bool {
        let mut states = HashSet::new();
        states.insert(State(self.guard.clone(), self.facing.clone()));
        while self.move_guard() {
            if !states.insert(State(self.guard.clone(), self.facing.clone())) {
                // we were already in this state, therefore loop
                return true;
            }
        }
        false
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let original: Map = text.parse().unwrap();
        let mut map = original.clone();
        let mut positions = HashSet::new();
        positions.insert(map.guard.clone());
        while map.move_guard() {
            positions.insert(map.guard.clone());
        }
        println!("Discreet positions: {}", positions.len());
        positions.remove(&original.guard);
        let mut positions_causing_loop = 0;
        for p in positions.iter() {
            let mut cloned = original.clone();
            cloned.obstacles[p.row][p.col] = true;
            if cloned.ends_in_loop() {
                positions_causing_loop += 1;
            }
        }
        println!("Looping positions: {}", positions_causing_loop);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}