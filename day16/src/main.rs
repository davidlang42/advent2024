use std::fs;
use std::env;
use std::str::FromStr;
use pathfinding::prelude::astar;
use std::collections::HashSet;

struct Maze {
    start: (Pos, Direction),
    end: Pos,
    walls: HashSet<Pos>
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Pos {
    row: usize,
    col: usize
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West
}

impl FromStr for Maze {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut end = None;
        let mut walls = HashSet::new();
        let mut row = 0;
        for line in text.lines() {
            let mut col = 0;
            for ch in line.chars() {
                if ch == '#' {
                    walls.insert(Pos { row, col });
                } else if ch == 'S' {
                    start = Some((Pos { row, col }, Direction::East));
                } else if ch == 'E' {
                    end = Some(Pos { row, col });
                }
                col += 1;
            }
            row += 1;
        }
        Ok(Self {
            walls,
            start: start.expect("Missing start"),
            end: end.expect("Missing end")
        })
    }
}

impl Pos {
    fn minimum_cost(&self, _facing: &Direction, end: &Pos) -> u32 {
        // could add minimum turn cost aswell
        (self.row.abs_diff(end.row) + self.col.abs_diff(end.col)) as u32
    }
  
    fn successors(&self, facing: &Direction, walls: &HashSet<Pos>) -> Vec<((Pos, Direction), u32)> {
        let mut v: Vec<((Pos, Direction), u32)> = Vec::new();
        match facing {
            Direction::North => {
                if self.row > 0 {
                    let new_pos = Pos {
                        row: self.row - 1,
                        col: self.col
                    };
                    if !walls.contains(&new_pos) {
                        v.push(((new_pos, *facing), 1));
                    }
                }
                v.push(((*self, Direction::East), 1000));
                v.push(((*self, Direction::West), 1000));
            },
            Direction::West => {
                if self.col > 0 {
                    let new_pos = Pos {
                        row: self.row,
                        col: self.col - 1
                    };
                    if !walls.contains(&new_pos) {
                        v.push(((new_pos, *facing), 1));
                    }
                }
                v.push(((*self, Direction::North), 1000));
                v.push(((*self, Direction::South), 1000));
            },
            Direction::South => {
                let new_pos = Pos {
                    row: self.row + 1,
                    col: self.col
                };
                if !walls.contains(&new_pos) {
                    v.push(((new_pos, *facing), 1));
                }
                v.push(((*self, Direction::East), 1000));
                v.push(((*self, Direction::West), 1000));
            },
            Direction::East => {
                let new_pos = Pos {
                    row: self.row,
                    col: self.col + 1
                };
                if !walls.contains(&new_pos) {
                    v.push(((new_pos, *facing), 1));
                }
                v.push(((*self, Direction::North), 1000));
                v.push(((*self, Direction::South), 1000));
            }
        }
        v
    }
  }

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let maze: Maze = text.parse().unwrap();
        if let Some((_path, cost)) = astar(&maze.start, |(p,d)| p.successors(d, &maze.walls), |(p,d)| p.minimum_cost(&d, &maze.end), |(p,_)| *p == maze.end) {
            println!("Minimum cost: {}", cost);
        } else {
            println!("No solution");
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}