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

    fn cheat_paths(&self, threshold: usize) -> Vec<(Pos, usize)> { // pos of cheated wall : picoseconds saved
        let no_cheat = self.no_cheat_path();
        let mut v = Vec::new();
        let mut progress = 0;
        let total = self.walls.len();

        for cheat_wall in &self.walls {
            let mut walls_without_cheat_wall = self.walls.clone();
            walls_without_cheat_wall.remove(&cheat_wall);
            let (result, _) = astar(
                &self.start,
                |p| p.adjacent(&self.size).into_iter().filter(|p| !walls_without_cheat_wall.contains(p)).map(|p| (p, 1)).collect::<Vec<(Pos, u32)>>(),
                |p| p.minimum_distance(&self.end),
                |p| *p == self.end
            ).expect("No solution");
            let cheat_path = result.len() - 1;
            let pico_saved = no_cheat - cheat_path;
            if pico_saved >= threshold {
                v.push((*cheat_wall, pico_saved));
            }
            progress += 1;
            if progress % 100 == 0 {
                println!("{}/{}={}%", progress, total, progress as f64 * 100.0 / total as f64);
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
        let race: Race = text.parse().unwrap();
        println!("No cheat path: {}", race.no_cheat_path());
        // test threshold 2, result 44
        // input threshold 100, result ?
        let threshold = 100;
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