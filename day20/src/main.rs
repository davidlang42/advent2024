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

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Cheat {
    start: Pos,
    end: Pos
}

impl Race {
    fn no_cheat_path(&self) -> usize {
        let (_, no_cheat_path) = astar(
            &self.start,
            |p| p.adjacent(&self.size).into_iter().filter(|p| !self.walls.contains(p)).map(|p| (p, 1)).collect::<Vec<(Pos, u32)>>(),
            |p| p.minimum_distance(&self.end),
            |p| *p == self.end
        ).expect("No solution");
        no_cheat_path as usize
    }

    fn cheat_paths(&self, moves: u32, threshold: u32) -> HashMap<Cheat, usize> { // cheat : picoseconds saved
        // find all poses we go through on the no cheat path
        let (no_cheat_path, total_no_cheat_length) = astar(
            &self.start,
            |p| p.adjacent(&self.size).into_iter().filter(|p| !self.walls.contains(p)).map(|p| (p, 1)).collect::<Vec<(Pos, u32)>>(),
            |p| p.minimum_distance(&self.end),
            |p| *p == self.end
        ).expect("No solution");
        // go through each pos and look for paths through walls which are less than 20 long
        let mut cheat_paths = HashMap::new();
        let mut skip = 1;
        for s in 0..no_cheat_path.len() {
            if s % 100 == 0 {
                println!("Start {}/{} ({}%)", s, no_cheat_path.len(), s as f64 * 100.0 / no_cheat_path.len() as f64);
            }
            for e in (s+1)..no_cheat_path.len() {
                let start = &no_cheat_path[s];
                let end = &no_cheat_path[e];
                let minimum_distance = start.minimum_distance(end);
                if minimum_distance > moves {
                    continue; // cheat wont be long enough
                }
                let no_cheat_length = e as u32 - s as u32;
                if no_cheat_length == minimum_distance {
                    continue; // cheat cant possibly help
                }
                if let Some((cheat_path, cheat_length)) = astar(
                    start,
                    |p| p.adjacent(&self.size).into_iter().filter(|p| self.walls.contains(p) || *p == *end).map(|p| (p, 1)).collect::<Vec<(Pos, u32)>>(),
                    |p| p.minimum_distance(end),
                    |p| *p == *end
                ) {
                    let total_cheat_length = s as u32 + cheat_length + no_cheat_path.len() as u32 - e as u32 - 1;
                    let pico_saved = no_cheat_length - cheat_length;
                    if cheat_length <= moves && cheat_length < no_cheat_length && pico_saved >= threshold {
                        let cheat = Cheat {
                            start: *start,
                            end: *end
                        };
                        if pico_saved == 50 {
                            if skip == 0 {
                                println!("50: {:?}", cheat);
                                println!("cheat-len: {}, no-cheat-len: {}, pico: {}, total_cheat_l: {}, total_no_cheat_l: {}", cheat_length, no_cheat_length, pico_saved, total_cheat_length, total_no_cheat_length);
                                // println!("cheat path: {}", cheat_path.len());
                                // for p in cheat_path {
                                //     println!("{:?}", p)
                                // }
                                panic!("HERE")
                            } else {
                                skip -= 1;
                            }
                        }
                        cheat_paths.insert(cheat, pico_saved as usize);
                    }
                }
            }
        }
        cheat_paths
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
        let threshold = 50;
        let moves = 20;
        let result = race.cheat_paths(moves, threshold);
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
        let mut cbs_vec: Vec<_> = count_by_saved.into_iter().collect();
        cbs_vec.sort_by(|a,b| a.0.cmp(&b.0));
        for (saved, count) in cbs_vec {
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