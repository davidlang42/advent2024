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

    // fn edges(&self) -> HashSet<Pos> {
    //     let mut edges = HashSet::new();
    //     for l in &self.locations {
    //         if l.adjacent().iter().any(|a| !self.locations.contains(&a)) {
    //             edges.insert(*l);
    //         }
    //     }
    //     edges
    // }

    fn corners(&self) -> usize {
        let mut corner_thirds = 0;
        for l in &self.locations {
            for count in self.count_corner_adjacent_locations(l) {
                if count == 0 {
                    // obtuse corner, this is the only time it will get counted, so it counts for 3
                    corner_thirds += 3;
                } else if count == 2 {
                    // acute corner, this will get counted 3 times, so it counts for 1
                    corner_thirds += 1;
                }
                // count == 1 or 3 means its not a corner
            }
        }
        corner_thirds / 3 // because each corner is counted 3 times
    }

    fn count_corner_adjacent_locations(&self, p: &Pos) -> [usize; 4] {
        // for each of the 4 corners of p, count how many (0-3) locations are adjacent other than p
        // 1 2 3
        // 4 p 6
        // 7 8 9
        // ie. [locations.contains(4,1,2), locations.contains(2,3,6), ..(6,9,8), ..(8,7,4)]
        let mut counts = [0; 4];
        let mut c = 0;
        for delta_row in [-1, 1] {
            for delta_col in [-1, 1] {
                if self.locations.contains(&p.delta(delta_row, 0)) {
                    counts[c] += 1;
                }
                if self.locations.contains(&p.delta(delta_row, delta_col)) {
                    counts[c] += 1;
                }
                if self.locations.contains(&p.delta(0, delta_col)) {
                    counts[c] += 1;
                }
                c += 1;
            }
        }
        counts
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