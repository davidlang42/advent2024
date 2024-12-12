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

    fn outer_perimeter(&self) -> usize {
        let min_row = self.locations.iter().map(|l| l.row).min().unwrap();
        let max_row = self.locations.iter().map(|l| l.row).max().unwrap();
        let min_col = self.locations.iter().map(|l| l.col).min().unwrap();
        let max_col = self.locations.iter().map(|l| l.col).max().unwrap();
        (max_row - min_row + 1 + max_col - min_col + 1) * 2
    }

    fn perimeter(&self, all_other_regions: Vec<&Region>, map: &Map) -> usize {
        let regions_fully_enclosed = all_other_regions.iter().filter(|r| self.fully_contains(r, map));
        self.outer_perimeter() + regions_fully_enclosed.map(|r| r.outer_perimeter()).sum::<usize>()
    }

    fn fully_contains(&self, other: &Self, map: &Map) -> bool {
        let mut locations_around_other = HashSet::new();
        for l in &other.locations {
            for a in l.adjacent(&map.max) {
                if !other.locations.contains(&a) {
                    locations_around_other.insert(a);
                }
            }
        }
        locations_around_other.iter().all(|l| self.locations.contains(l))
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
    fn adjacent(&self, max: &Pos) -> Vec<Pos> {
        let mut v = Vec::new();
        if self.row > 0 {
            v.push(Pos {
                row: self.row - 1,
                col: self.col
            });
        }
        if self.col > 0 {
            v.push(Pos {
                row: self.row,
                col: self.col - 1
            });
        }
        if self.row < max.row {
            v.push(Pos {
                row: self.row + 1,
                col: self.col
            });
        }
        if self.col < max.col {
            v.push(Pos {
                row: self.row,
                col: self.col + 1
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
        let regions = map.regions();
        let mut sum = 0;
        for i in 0..regions.len() {
            let r = &regions[i];
            let mut other: Vec<&Region> = regions[0..i].iter().collect();
            other.append(&mut regions[i..regions.len()].iter().collect());
            let area = r.area();
            let perimeter = r.perimeter(other, &map);
            println!("Region '{}' with area {}, perimeter {}", r.plant.0, area, perimeter);
            sum += area * perimeter;
        }
        println!("Total price: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}