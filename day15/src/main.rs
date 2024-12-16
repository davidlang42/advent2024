use std::fs;
use std::env;
use std::str::FromStr;

struct Map {
    tiles: Vec<Vec<Tile>>,
    robot: Pos
}

#[derive(Debug, Clone)]
struct Pos {
    row: usize,
    col: usize
}

#[derive(PartialEq, Copy, Clone)]
enum Tile {
    Empty,
    Box,
    Wall
}

impl Tile {
    fn from_char(ch: char) -> Self {
        match ch {
            '#' => Self::Wall,
            'O' => Self::Box,
            '.' => Self::Empty,
            _ => panic!("Invalid tile: {}", ch)
        }
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(section: &str) -> Result<Self, Self::Err> {
        let mut tiles = Vec::new();
        let mut robot = None;
        for line in section.lines() {
            let mut row = Vec::new();
            for ch in line.chars() {
                if ch == '@' {
                    robot = Some(Pos {
                        row: tiles.len(),
                        col: row.len()
                    });
                    row.push(Tile::Empty);
                } else {
                    row.push(Tile::from_char(ch));
                }
            }
            tiles.push(row);
        }
        Ok(Self {
            tiles,
            robot: robot.expect("Robot to be found")
        })
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn from_char(ch: char) -> Self {
        match ch {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => panic!("Invalid tile: {}", ch)
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Right => (0, 1),
            Self::Down => (1, 0),
            Self::Left => (0, -1)
        }
    }
}

impl Map {
    fn get(&self, pos: &Pos) -> &Tile {
        &self.tiles[pos.row][pos.col]
    }

    fn set(&mut self, pos: &Pos, tile: Tile) {
        self.tiles[pos.row][pos.col] = tile;
    }

    fn move_all(&mut self, directions: Vec<Direction>) {
        for direction in directions {
            self.move_one(direction);
        }
    }

    fn move_one(&mut self, direction: Direction) {
        let (dr, dc) = direction.delta();
        let existing = self.robot.clone();
        if self.maybe_move(&existing, dr, dc) {
            self.robot.row = (self.robot.row as isize + dr) as usize;
            self.robot.col = (self.robot.col as isize + dc) as usize;
        }
    }

    fn maybe_move(&mut self, pos: &Pos, dr: isize, dc: isize) -> bool {
        let new_pos = Pos {
            row: (pos.row as isize + dr) as usize,
            col: (pos.col as isize + dc) as usize
        };
        match self.get(&new_pos) {
            Tile::Wall => false,
            Tile::Empty => {
                self.set(&new_pos, *self.get(&pos));
                self.set(&pos, Tile::Empty);
                true
            },
            Tile::Box => {
                if self.maybe_move(&new_pos, dr, dc) {
                    self.set(&new_pos, *self.get(&pos));
                    self.set(&pos, Tile::Empty);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn sum_gps(&self) -> usize {
        let mut sum = 0;
        for row in 0..self.tiles.len() {
            for col in 0..self.tiles[0].len() {
                if self.tiles[row][col] == Tile::Box {
                    sum += row * 100 + col;
                }
            }
        }
        sum
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let sections: Vec<&str> = text.split("\r\n\r\n").collect();
        if sections.len() != 2 {
            panic!("Invalid input sections");
        }
        let mut map: Map = sections[0].parse().unwrap();
        let mut directions = Vec::new();
        for line in sections[1].lines() {
            for ch in line.chars() {
                directions.push(Direction::from_char(ch));
            }
        }
        map.move_all(directions);
        println!("GPS Sum: {}", map.sum_gps());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}