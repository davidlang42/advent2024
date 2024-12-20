use std::fs;
use std::env;
use std::str::FromStr;
use pathfinding::prelude::bfs;
use std::collections::HashSet;
use std::collections::HashMap;

struct Race {
    start: Pos,
    end: Pos,
    walls: HashSet<Pos>
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Pos {
    row: isize,
    col: isize
}

impl FromStr for Race {
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
                    start = Some(Pos { row, col });
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

#[derive(Clone, Hash, Eq, PartialEq)]
struct CheatState {
    start: Option<Pos>,
    moves: usize
}

impl CheatState {
    fn is_active(&self) -> bool {
        !self.start.is_none() && self.moves > 0
    }

    fn next_state(&self) -> Self {
        if !self.is_active() {
           panic!("Called next_state() when not active");
        }
        Self {
            start: self.start,
            moves: self.moves - 1
        }
    }

    fn is_available(&self) -> bool {
        self.start.is_none()
    }

    fn start_cheat(&self, start: &Pos) -> Self {
        if !self.is_available() {
           panic!("Called start_cheat() when not available");
        }
        Self {
            start: Some(*start),
            moves: 1
        }
    }
}

impl Pos {
    // fn minimum_cost(&self, end: &Pos) -> u32 {
    //     (self.row.abs_diff(end.row) + self.col.abs_diff(end.col)) as u32
    // }

    fn adjacent(&self) -> [Self; 4] {
        [
            Self {
                row: self.row + 1,
                col: self.col
            },
            Self {
                row: self.row - 1,
                col: self.col
            },
            Self {
                row: self.row,
                col: self.col + 1
            },
            Self {
                row: self.row,
                col: self.col - 1
            }
        ]
    }
  
    fn successors(&self, cheat: &CheatState, walls: &HashSet<Pos>) -> Vec<(Pos, CheatState)> {
        let mut v: Vec<(Pos, CheatState)> = Vec::new();
        for adj in self.adjacent() {
            if cheat.is_active() {
                v.push((adj, cheat.next_state()));
            } else {
                if cheat.is_available() {
                    let cheat_started = cheat.start_cheat(&adj);
                    v.push((adj, cheat_started));
                }
                if !walls.contains(&adj) {
                    v.push((adj, cheat.clone()));
                }
            }
        }
        v
    }
}

impl Race {
    fn no_cheat_path(&self) -> usize {
        let no_cheat = CheatState {
            start: Some(Pos {
                row: 0, col: 0
            }),
            moves: 0
        };
        let result = bfs(&(self.start, no_cheat), |(p,c): &(Pos, CheatState)| p.successors(c, &self.walls), |(p, _c)| *p == self.end);
        result.expect("No solution").len() - 1
    }

    fn cheat_paths(&self) -> HashMap<Pos, usize> { // start cheat : picoseconds saved
        let no_cheat = self.no_cheat_path();
        let not_yet_cheated = CheatState {
            start: None,
            moves: 0
        };
        let result = bfs(&(self.start, not_yet_cheated), |(p,c): &(Pos, CheatState)| p.successors(c, &self.walls), |(p, _c)| *p == self.end).expect("No solution");
        let (_, final_cheat) = result.last().unwrap();


        let mut h = HashMap::new();
        h.insert(final_cheat.start.unwrap(), no_cheat - (result.len() - 1));
        h
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let race: Race = text.parse().unwrap();
        println!("No cheat path: {}", race.no_cheat_path());
        let mut count = 0;
        for (start_cheat, pico) in race.cheat_paths().iter() {
            println!("Start at {:?}, saves {} picoseconds", start_cheat, pico);
            if *pico >= 100 {
                count += 1;
            }
        }
        println!("Count > 100: {}", count);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}