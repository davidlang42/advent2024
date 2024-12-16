use std::fs;
use std::env;
use std::str::FromStr;

struct Map {
    size: Pos,
    robots: Vec<Robot>
}

#[derive(Debug)]
struct Robot {
    position: Pos,
    velocity: Pos
}

#[derive(Debug)]
struct Pos {
    row: isize,
    col: isize
}

impl FromStr for Map {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let sections: Vec<_> = text.split("\r\n\r\n").collect();
        if sections.len() != 2 {
            panic!("Invalid section count")
        }
        let size = sections[0].parse().unwrap();
        let robots = sections[1].lines().map(|l| l.parse().unwrap()).collect();
        Ok(Self {
            size,
            robots
        })
    }
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let sections: Vec<_> = line.split(" ").collect();
        if sections.len() != 2 {
            panic!("Invalid robot")
        }
        let position = sections[0].split("=").nth(1).unwrap().parse().unwrap();
        let velocity = sections[1].split("=").nth(1).unwrap().parse().unwrap();
        Ok(Self {
            position,
            velocity
        })
    }
}

impl FromStr for Pos {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // 0,4 => row:4, col:0
        let numbers: Vec<isize> = line.split(",").map(|s| s.parse().unwrap()).collect();
        if numbers.len() != 2 {
            panic!("Invalid number count");
        }
        Ok(Self {
            row: numbers[1],
            col: numbers[0]
        })
    }
}

impl Map {
    fn simulate(&mut self, seconds: usize) {
        //TODO
    }

    fn count_robots(&self, from_row: isize, less_than_row: isize, from_col: isize, less_than_col: isize) -> usize {
        let mut count = 0;
        for r in &self.robots {
            if r.position.row >= from_row && r.position.row < less_than_row && r.position.col >= from_col && r.position.col < less_than_col {
                count += 1;
            }
        }
        count
    }

    fn quadrants(&self) -> [usize; 4] {
        let end = &self.size;
        let mid = Pos {
            row: end.row / 2,
            col: end.col / 2
        };
        [
            self.count_robots(0, mid.row, 0, mid.col),
            self.count_robots(0, mid.row, mid.col + 1, end.col),
            self.count_robots(mid.row + 1, end.row, 0, mid.col),
            self.count_robots(mid.row + 1, end.row, mid.col + 1, end.col)
        ]
    }

    fn safety_factor(&self) -> usize {
        self.quadrants().iter().product()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let seconds: usize = args[2].parse().unwrap();
        let mut map: Map = text.parse().unwrap();
        map.simulate(seconds);
        println!("Quadrants: {:?}", map.quadrants());
        println!("Safety factor: {}", map.safety_factor());
    } else {
        println!("Please provide 2 arguments: Filename, Seconds");
    }
}