use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::HashMap;
use itertools::Itertools;
use std::collections::HashSet;

struct Map {
    antennas: HashMap<Pos, Antenna>,
    max_pos: Pos
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Pos {
    row: usize,
    col: usize
}

#[derive(Debug, PartialEq)]
struct Antenna(char);

impl Antenna {
    fn iter() -> Vec<Antenna> {
        let mut v = Vec::new();
        for i in 0..26 {
            if i < 10 {
                v.push(Antenna((i + 48) as u8 as char));
            }
            v.push(Antenna((i + 65) as u8 as char));
            v.push(Antenna((i + 97) as u8 as char));
        }
        v
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut antennas = HashMap::new();
        let mut max_pos = Pos {
            row: 0,
            col: 0
        };
        for (row, line) in text.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch != '.' {
                    antennas.insert(Pos { row, col }, Antenna(ch));
                }
                max_pos.col = col;
            }
            max_pos.row = row;
        }
        Ok(Self {
            antennas,
            max_pos
        })
    }
}

impl Map {
    fn antinodes(&self) -> HashSet<Pos> {
        let mut antinodes = HashSet::new();
        for antenna in Antenna::iter() {
            let locations: Vec<&Pos> = self.antennas.iter().filter(|(_k,v)| **v == antenna).map(|(k,_v)| k).collect();
            for pair in locations.iter().permutations(2) {
                let delta_row = pair[1].row as isize - pair[0].row as isize;
                let delta_col = pair[1].col as isize - pair[0].col as isize;
                let mut row: isize = pair[1].row as isize;
                let mut col: isize = pair[1].col as isize;
                while row >= 0 && row as usize <= self.max_pos.row && col >= 0 && col as usize <= self.max_pos.col {
                    antinodes.insert(Pos {
                        row: row as usize,
                        col: col as usize
                    });
                    row += delta_row;
                    col += delta_col;
                }
            }
        }
        antinodes
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let map: Map = text.parse().unwrap();
        println!("Anitnodes: {}", map.antinodes().len());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}