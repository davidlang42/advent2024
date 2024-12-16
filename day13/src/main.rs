use std::fs;
use std::env;
use std::str::FromStr;
use gcd::Gcd;
use std::collections::HashSet;

#[derive(Debug)]
struct Claw {
    a_delta: Pos,
    b_delta: Pos,
    target: Pos
}

#[derive(Debug, Clone)]
struct Pos {
    row: usize,
    col: usize
}

struct Presses {
    a: usize,
    b: usize
}

impl Presses {
    fn cost(&self) -> usize {
        3 * self.a + self.b
    }
}

impl FromStr for Claw {
    type Err = String;

    fn from_str(section: &str) -> Result<Self, Self::Err> {
        let mut lines = section.lines();
        let a_delta = lines.next().unwrap().split(": ").nth(1).unwrap().parse().unwrap();
        let b_delta = lines.next().unwrap().split(": ").nth(1).unwrap().parse().unwrap();
        let target = lines.next().unwrap().split(": ").nth(1).unwrap().parse().unwrap();
        Ok(Self {
            a_delta,
            b_delta,
            target
        })
    }
}

impl FromStr for Pos {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        //eg. X+94, Y+34
        //eg. X=8400, Y=5400
        let mut numbers = line.split(", ");
        let row = numbers.next().unwrap().chars().skip(2).collect::<String>().parse().unwrap();
        let col = numbers.next().unwrap().chars().skip(2).collect::<String>().parse().unwrap();
        Ok(Self {
            row,
            col
        })
    }
}

impl Pos {
    fn add(&mut self, delta: &Pos) {
        self.row += delta.row;
        self.col += delta.col;
    }
}

impl Claw {
    fn win(&self) -> Option<Presses> {
        let row = LinearEquation::new(self.a_delta.row, self.b_delta.row, self.target.row);
        let col = LinearEquation::new(self.a_delta.col, self.b_delta.col, self.target.col);
        if !row.has_soln() || !col.has_soln() {
            return None;
        }
        let row_solutions = row.solve().unwrap().all();
        let column_solutions = col.solve().unwrap().all();
        let mut min_press: Option<Presses> = None;
        for rs in row_solutions {
            if column_solutions.contains(&rs) {
                // found a way to win, keep track of cheapest
                let (a, b) = rs;
                let new = Presses {
                    a, b
                };
                if let Some(existing) = &min_press {
                    if new.cost() < existing.cost() {
                        min_press = Some(new);
                    }
                } else {
                    min_press = Some(new);
                }
            }
        }
        min_press
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let claws: Vec<Claw> = text.split("\r\n\r\n").map(|s| s.parse().unwrap()).collect(); // "\n\n" on unix
        println!("PART1");
        let mut part1 = 0;
        for claw in &claws {
            if let Some(presses) = claw.win() {
                let cost = presses.cost();
                part1 += cost;
                println!("Press Ax{}, Bx{} = {} tokens", presses.a, presses.b, cost);
            } else {
                //println!("Can't be won");
            }
        }
        println!("PART2");
        let mut part2 = 0;
        for mut claw in claws {
            claw.target.add(&Pos { row: 10000000000000, col: 10000000000000 });
            if let Some(presses) = claw.win() {
                let cost = presses.cost();
                part2 += cost;
                println!("Press Ax{}, Bx{} = {} tokens", presses.a, presses.b, cost);
            } else {
                //println!("Can't be won");
            }
        }
        println!("RESULTS");
        println!("Part1 tokens: {:?}", part1);
        println!("Part2 tokens: {:?}", part2);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

struct LinearEquation {
    a: usize,
    b: usize,
    c: usize,
    d: usize
}

impl LinearEquation {
    // https://math.libretexts.org/Courses/Mount_Royal_University/Higher_Arithmetic/5%3A_Diophantine_Equations/5.1%3A_Linear_Diophantine_Equations
    // ax + by = c

    fn new(a: usize, b: usize, c: usize) -> Self {
        let d = a.gcd(b);
        Self {
            a, b, c, d
        }
    }

    fn has_soln(&self) -> bool {
        self.c.rem_euclid(self.d) == 0
    }

    fn solve(&self) -> Option<LinearSolution> {
        if self.has_soln() {
            Some(LinearSolution::from(self))
        } else {
            None
        }
    }
}

struct LinearSolution {
    a: isize,
    b: isize,
    c: isize,
    d: isize,
    x0: isize,
    y0: isize
}

impl LinearSolution {
    fn from(equation: &LinearEquation) -> Self {
        let mut x0 = 0;
        loop {
            let numerator = equation.d as isize - x0 * equation.a as isize;
            let denominator = equation.b as isize;
            if numerator.rem_euclid(denominator) == 0 {
                let y0 = numerator / denominator;
                return Self {
                    a: equation.a as isize,
                    b: equation.b as isize,
                    c: equation.c as isize,
                    d: equation.d as isize,
                    x0, y0
                }
            }
            x0 += 1;
        }
    }

    fn all(&self) -> HashSet<(usize, usize)> {
        let mut solutions = HashSet::new();
        let min_m = -1 * self.c * self.y0 / self.a;
        let max_m = self.c * self.x0 / self.b;
        for m in min_m..(max_m + 1) {
            let x = self.c * self.x0 / self.d - m * self.b / self.d;
            let y = self.a * m / self.d + self.c * self.y0 / self.d;
            if self.a * x + self.b * y == self.c {
                solutions.insert((x as usize, y as usize));
            }
        }
        solutions
    }
}