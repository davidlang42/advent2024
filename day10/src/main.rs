use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::HashSet;

struct Map {
    heights: Vec<Vec<usize>>,
    max_pos: Pos
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Pos {
    row: usize,
    col: usize
}

impl FromStr for Map {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let heights: Vec<_> = text.lines().map(|l| l.chars().map(|c| if c == '.' {
            100 // impassable
        } else {
            c.to_string().parse().unwrap()
        }).collect::<Vec<usize>>()).collect();
        let max_pos = Pos {
            row: heights.len() - 1,
            col: heights[0].len() - 1
        };
        Ok(Self {
            heights,
            max_pos
        })
    }
}

impl Map {
    fn trail_heads(&self) -> Vec<(Pos, usize, usize)> {
        let mut v = Vec::new();
        for row in 0..self.heights.len() {
            for col in 0..self.heights[0].len() {
                if self.heights[row][col] == 0 {
                    let pos = Pos { row, col };
                    let trail_ends = self.trail_ends(pos, 0, 9);
                    let trails = self.trails(pos, 0, 9);
                    v.push((pos, trail_ends.len(), trails));
                }
            }
        }
        v
    }

    fn trail_ends(&self, at: Pos, from: usize, to: usize) -> HashSet<Pos> {
        let mut ends = HashSet::new();
        if from == to {
            ends.insert(at);
        } else {
            for adj in self.adjacent_pos(at) {
                if self.heights[adj.row][adj.col] == from + 1 {
                    for end in self.trail_ends(adj, from + 1, to) {
                        ends.insert(end);
                    }
                }
            }
        }
        ends
    }

    fn trails(&self, at: Pos, from: usize, to: usize) -> usize {
        if from == to {
            1
        } else {
            let mut count = 0;
            for adj in self.adjacent_pos(at) {
                if self.heights[adj.row][adj.col] == from + 1 {
                    count += self.trails(adj, from + 1, to);
                }
            }
            count
        }
    }

    fn adjacent_pos(&self, pos: Pos) -> Vec<Pos> {
        let mut v = Vec::new();
        if pos.row > 0 {
            v.push(Pos {
                row: pos.row - 1,
                col: pos.col
            });
        }
        if pos.col > 0 {
            v.push(Pos {
                row: pos.row,
                col: pos.col - 1
            });
        }
        if pos.row < self.max_pos.row {
            v.push(Pos {
                row: pos.row + 1,
                col: pos.col
            });
        }
        if pos.col < self.max_pos.col {
            v.push(Pos {
                row: pos.row,
                col: pos.col + 1
            });
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
        let map: Map = text.parse().unwrap();
        let mut sum = 0;
        let mut ratings = 0;
        for (trail_head, score, rating) in map.trail_heads() {
            println!("Trailhead at ({},{}) with score {}, rating {}", trail_head.row, trail_head.col, score, rating);
            sum += score;
            ratings += rating;
        }
        println!("Total score: {}", sum);
        println!("Total rating: {}", ratings);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}