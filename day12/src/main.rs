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
        let max = Pos {
            row:  plants.len() as isize - 1,
            col: plants[0].len() as isize - 1
        };
        Ok(Self {
            plants,
            max
        })
    }
}

impl Map {
    fn at(&self, location: &Pos) -> Option<Plant> {
        if location.row >= 0 && location.col >= 0 && location.row <= self.max.row && location.col <= self.max.col {
            Some(self.plants[location.row as usize][location.col as usize])
        } else {
            None
        }
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
        for row in 0..(self.max.row + 1) {
            for col in 0..(self.max.col + 1) {
                let p = Pos { row, col };
                if self.at(&p) == Some(plant) {
                    set.insert(p);
                }
            }
        }
        set
    }
}

impl Region {
    fn expand(&mut self, start: &Pos, map: &Map) {
        if self.locations.insert(*start) {
            for adj in start.adjacent() {
                if map.at(&adj) == Some(self.plant) {
                    self.expand(&adj, map);
                }
            }
        }
    }

    fn area(&self) -> usize {
        self.locations.len()
    }

    fn perimeter(&self) -> usize {
        let mut perimeter = 0;
        for l in &self.locations {
            for a in l.adjacent() {
                if !self.locations.contains(&a) {
                    perimeter += 1;
                }
            }
        }
        perimeter
    }

    fn edges(&self) -> HashSet<Pos> {
        let mut edges = HashSet::new();
        for l in &self.locations {
            if l.adjacent().iter().any(|a| !self.locations.contains(&a)) {
                edges.insert(*l);
            }
        }
        edges
    }

    fn corners(&self) -> usize {
        let mut corners = 0;
        for l in &self.locations {
            for triple in self.side_triples(l) {
                corners += match triple {
                    (true, true, true) => 0,
                    (false, false, false) => 0,
                    (true, false, true) => 2,
                    (false, true, false) => 2,
                    (true, false, false) => 1,
                    (true, true, false) => 1,
                    (false, true, true) => 1,
                    (false, false, true) => 1
                };
            }
        }
        corners / 2 // because every corner would have been counted twice
    }

    fn side_triples(&self, p: &Pos) -> [(bool, bool, bool); 4] {
        [
            (self.locations.contains(&p.delta(-1, -1)), self.locations.contains(&p), self.locations.contains(&p.delta(-1, 1))),
            (self.locations.contains(&p.delta(-1, -1)), self.locations.contains(&p), self.locations.contains(&p.delta(1, -1))),
            (self.locations.contains(&p.delta(1, -1)), self.locations.contains(&p), self.locations.contains(&p.delta(1, 1))),
            (self.locations.contains(&p.delta(-1, 1)), self.locations.contains(&p), self.locations.contains(&p.delta(1, 1))),
        ]
    }

    // fn sides(&self) -> usize {
    //     let mut edges = self.edges();
    //     let mut ordered = Vec::new();
    //     let mut corners = 0;
    //     while edges.len() != 0 {
            
            

    //         // take any point out of the set
    //         let start_of_loop: Pos = *edges.iter().next().unwrap();
    //         edges.remove(&start_of_loop);
    //         ordered.push(start_of_loop);

    //         let start_of_side = start_of_loop;
    //         // find either of the 2 points adjacent to it
    //         let mut next = start_of_loop.adjacent().filter(|a| edges.contains(&a)).next().unwrap();
    //         // measure the delta (ie. direction of travel)
    //         let delta = Pos {
    //             row: next.row - start.row,
    //             col: next.col - start.col
    //         };
    //         // continue in that direction until the next point isn't in the set
    //         while edges.remove(&next) {
    //             ordered.push(next);
    //             next = Pos {
    //                 row: next.row + delta.row,
    //                 col: next.col + delta.col
    //             };
    //         }
    //         // the last edge point we removed is a corner, start a new side from here
    //         corners += 1;
    //         start = ordered.iter().last().unwrap();
    //     }
    //     sides
    // }
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
    fn delta(&self, delta_row: isize, delta_col: isize) -> Self {
        Self {
            row: self.row + delta_row,
            col: self.col + delta_col
        }
    }

    fn adjacent(&self) -> [Pos; 4] {
        [self.delta(-1, 0), self.delta(0, -1), self.delta(1, 0), self.delta(0, 1)]
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
        let mut part1 = 0;
        let mut part2 = 0;
        for r in regions {
            let area = r.area();
            let perimeter = r.perimeter();
            let sides = r.corners();
            println!("Region '{}' with area {}, perimeter {}, sides {}", r.plant.0, area, perimeter, sides);
            part1 += area * perimeter;
            part2 += area * sides;
        }
        println!("Part1 price by perimeter: {}", part1);
        println!("Part2 price by sides: {}", part2);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}