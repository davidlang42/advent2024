use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq)]
struct Plant(char);

struct Map {
    plants: Vec<Vec<Plant>>,
    max: Pos
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Pos {
    row: usize,
    col: usize
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct OutOfMapPos {
    row: isize,
    col: isize
}

struct Region {
    plant: Plant,
    locations: HashSet<Pos>
}

impl FromStr for Map {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let plants: Vec<Vec<Plant>> = text.lines().map(|l| l.chars().map(|ch| Plant(ch)).collect()).collect();
        let row = plants.len() - 1;
        let col = plants[0].len() - 1;
        Ok(Self {
            plants,
            max: Pos {
                row,
                col
            }
        })
    }
}

impl Map {
    fn at(&self, location: &Pos) -> Plant {
        self.plants[location.row][location.col]
    }

    fn regions(&self) -> Vec<Region> {
        let mut regions = Vec::new();
        for plant in Plant::iter() {
            let mut locations = self.locate(plant);
            while locations.len() != 0 {
                let mut region = Region {
                    plant,
                    locations: HashSet::new()
                };
                let start = *locations.iter().next().unwrap();
                locations.remove(&start);
                region.expand(&start, self);
                for l in &region.locations {
                    locations.remove(&l);
                }
                regions.push(region);
            }
        }
        regions
    }

    fn locate(&self, plant: Plant) -> HashSet<Pos> {
        let mut set = HashSet::new();
        for row in 0..self.plants.len() {
            for col in 0..self.plants[row].len() {
                if self.plants[row][col] == plant {
                    set.insert(Pos { row, col });
                }
            }
        }
        set
    }
}

impl Region {
    fn expand(&mut self, start: &Pos, map: &Map) {
        if self.locations.insert(*start) {
            for adj in start.adjacent(&map.max) {
                if map.at(&adj) == self.plant {
                    self.expand(&adj, map);
                }
            }
        }
    }

    fn area(&self) -> usize {
        self.locations.len()
    }

    fn perimeter(&self, map: &Map) -> usize {
        let mut perimeter = 0;
        for l in &self.locations {
            let l_out_of_map = l.out_of_map();
            for a in l_out_of_map.adjacent_including_out_of_map() {
                if let Some(a_in_map) = a.in_map(map) {
                    if !self.locations.contains(&a_in_map) {
                        perimeter += 1;
                    }
                } else {
                    perimeter += 1;
                }
            }
        }
        perimeter
    }
}

impl Plant {
    fn iter() -> Vec<Self> {
        let mut v = Vec::new();
        let a = 'A' as u8;
        for i in a..(a+26) {
            v.push(Plant(i as char));
        }
        v
    }
}

impl Pos {
    fn out_of_map(&self) -> OutOfMapPos {
        OutOfMapPos {
            row: self.row as isize,
            col: self.col as isize
        }
    }

    fn adjacent(&self, max: &Pos) -> Vec<Pos> {
        let mut v = Vec::new();
        if self.row > 0 {
            v.push(Self {
                row: self.row - 1,
                col: self.col
            });
        }
        if self.col > 0 {
            v.push(Self {
                row: self.row,
                col: self.col - 1
            });
        }
        if self.row < max.row {
            v.push(Self {
                row: self.row + 1,
                col: self.col
            });
        }
        if self.col < max.col {
            v.push(Self {
                row: self.row,
                col: self.col + 1
            });
        }
        v
    }
}

impl OutOfMapPos {
    fn in_map(&self, map: &Map) -> Option<Pos> {
        if self.row >= 0 && self.col >= 0 && self.row <= map.max.row as isize && self.col <= map.max.col as isize {
            Some(Pos {
                row: self.row as usize,
                col: self.col as usize
            })
        } else {
            None
        }
    }

    fn adjacent_including_out_of_map(&self) -> Vec<Self> {
        let mut v = Vec::new();
        v.push(Self {
            row: self.row - 1,
            col: self.col
        });
        v.push(Self {
            row: self.row,
            col: self.col - 1
        });
        v.push(Self {
            row: self.row + 1,
            col: self.col
        });
        v.push(Self {
            row: self.row,
            col: self.col + 1
        });
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
        let regions = map.regions();
        let mut sum = 0;
        for r in regions {
            let area = r.area();
            let perimeter = r.perimeter(&map);
            println!("Region '{}' with area {}, perimeter {}", r.plant.0, area, perimeter);
            sum += area * perimeter;
        }
        println!("Total price: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}