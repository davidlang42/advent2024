use std::fs;
use std::env;
use std::str::FromStr;
use pathfinding::prelude::astar;
use std::collections::HashSet;
use std::collections::HashMap;

struct Race {
    start: Pos,
    end: Pos,
    walls: HashSet<Pos>,
    size: Pos
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
        let mut size = Pos {
            row: 0,
            col: 0
        };
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
                size.col = col;
            }
            row += 1;
            size.row = row;
        }
        Ok(Self {
            walls,
            start: start.expect("Missing start"),
            end: end.expect("Missing end"),
            size
        })
    }
}

impl Pos {
    fn minimum_distance(&self, end: &Pos) -> u32 {
        (self.row.abs_diff(end.row) + self.col.abs_diff(end.col)) as u32
    }

    fn adjacent(&self, size: &Pos) -> Vec<Self> {
        let mut v = Vec::new();
        if self.row < size.row - 1 {
            v.push(Self {
                row: self.row + 1,
                col: self.col
            });
        }
        if self.row > 0 {
            v.push(Self {
                row: self.row - 1,
                col: self.col
            });
        }
        if self.col < size.col - 1 {
            v.push(Self {
                row: self.row,
                col: self.col + 1
            });
        }
        if self.col > 0 {
            v.push(Self {
                row: self.row,
                col: self.col - 1
            });
        }
        v
    }
}


#[derive(Clone, Hash, Eq, PartialEq)]
struct CheatState {
    start: Option<Pos>,
    end: Option<Pos>,
    moves: usize
}

//TODO cheat state
impl CheatState {
    fn is_active(&self) -> bool {
        !self.start.is_none() && self.moves > 0
    }

    fn continue_cheat(&self) -> Option<Self> {
        if !self.is_active() {
           panic!("Called next_state() when not active");
        }
        if self.moves > 1 {
            Some(Self {
                start: self.start,
                moves: self.moves - 1,
                end: None
            })
        } else {
            None // ran out of moves, can't continue
        }
    }

    fn end_cheat(&self, end: &Pos) -> Self {
        if !self.is_active() {
            panic!("Called end_cheat() when not active");
         }
         Self {
            start: self.start,
            end: Some(*end),
            moves: 0
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
            end: None,
            moves: 1
        }
    }
    
    fn successors(&self, pos: &Pos, size: &Pos, walls: &HashSet<Pos>, existing_solutions: &HashMap<CheatState, usize>) -> Vec<((Pos, CheatState),u32)> {
        let mut v = Vec::new();
        for adj in pos.adjacent(size) {
            if self.is_active() {
                if walls.contains(&adj) {
                    if let Some(next_state) = self.continue_cheat() {
                        v.push(((adj, next_state), 1));
                    }
                } else {
                    let next_state = self.end_cheat(&adj);
                    if !existing_solutions.contains_key(&next_state) {
                        v.push(((adj, next_state), 1));
                    }
                }
            } else {
                //TODO up to here
                if self.is_available() && !existing_solutions.contains_key(&self) {
                    let cheat_started = self.start_cheat(&adj);
                    v.push(((adj, cheat_started), 1));
                }
                if !walls.contains(&adj) {
                    v.push(((adj, self.clone()), 1));
                }
            }
        }
        v
    }
}

impl Race {
    fn no_cheat_path(&self) -> usize {
        let (result, _) = astar(
            &self.start,
            |p| p.adjacent(&self.size).into_iter().filter(|p| !self.walls.contains(p)).map(|p| (p, 1)).collect::<Vec<(Pos, u32)>>(),
            |p| p.minimum_distance(&self.end),
            |p| *p == self.end
        ).expect("No solution");
        result.len() - 1
    }

    fn cheat_paths(&self, threshold: usize) -> HashMap<CheatState, usize> { // cheat : picoseconds saved
        let no_cheat = self.no_cheat_path();
        let not_yet_cheated = CheatState {
            start: None,
            end: None,
            moves: 0
        };
        let mut solutions_above_threshold = HashMap::new();
        while let Some(result) = astar(
            &(self.start, not_yet_cheated.clone()),
            |(p, c)| c.successors(p, &self.size, &self.walls, &solutions_above_threshold),
            |(p, _c)| p.minimum_distance(&self.end),
            |(p, _c)| *p == self.end
        ) {
            let pico_saved = no_cheat - (result.0.len() - 1);
            if pico_saved < threshold {
                break;
            }
            let (_, final_cheat) = result.0.into_iter().last().unwrap();
            let cheat_start = final_cheat.start.unwrap();
            println!("Start at {:?}, saves {} picoseconds", cheat_start, pico_saved);
            solutions_above_threshold.insert(final_cheat, pico_saved);
        }
        solutions_above_threshold
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
        // test threshold 2, result 44
        // input threshold 100, result ?
        let threshold = 2;
        let result = race.cheat_paths(threshold);
        println!("Count > {}: {}", threshold, result.len());
        let mut count_by_saved = HashMap::new();
        for (_, pico) in result {
            if let Some(existing) = count_by_saved.get(&pico) {
                count_by_saved.insert(pico, existing + 1);
            } else {
                count_by_saved.insert(pico, 1);
            }
        }
        let mut sum = 0;
        for (saved, count) in count_by_saved {
            sum += count;
            if count == 1 {
                println!("There is one cheat that saves {} picoseconds.", saved);
            } else {
                println!("There are {} cheats that save {} picoseconds.", count, saved);
            }
        }
        println!("Thats a total of {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}