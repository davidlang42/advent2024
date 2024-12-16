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
    fn solve_linear_equation_all(a_u: usize, b_u: usize, c_u: usize) -> HashSet<(usize, usize)> {
        // https://math.libretexts.org/Courses/Mount_Royal_University/Higher_Arithmetic/5%3A_Diophantine_Equations/5.1%3A_Linear_Diophantine_Equations
        // ax + by = c
        let a = a_u as isize;
        let b = b_u as isize;
        let c = c_u as isize;
        let mut solutions = HashSet::new();
        let d = a_u.gcd(b_u) as isize;
        if c.rem_euclid(d) == 0 {
            let (x0, y0) = Self::solve_linear_equation_any(a, b, d);
            let min_m = -1 * c * y0 / a;
            let max_m = c * x0 / b;
            for m in min_m..(max_m + 1) {
                let x = c * x0 / d - m * b / d;
                let y = a * m / d + c * y0 / d;
                if a * x + b * y == c {
                    solutions.insert((x as usize, y as usize));
                }
            }
        }
        solutions
    }

    fn solve_linear_equation_any(a: isize, b: isize, c: isize) -> (isize, isize) {
        let mut x = 0;
        loop {
            let y = (c - x * a) / b;
            if (c - x * a).rem_euclid(b) == 0 {
                return (x, y);
            }
            x += 1;
        }
    }

    fn win(&self) -> Option<Presses> {
        let row_solutions = Self::solve_linear_equation_all(self.a_delta.row, self.b_delta.row, self.target.row);
        let column_solutions = Self::solve_linear_equation_all(self.a_delta.col, self.b_delta.col, self.target.col);
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
        let claws: Vec<Claw> = text.split("\n\n").map(|s| s.parse().unwrap()).collect(); // "\r\n\r\n" on windows
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