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

#[derive(Debug)]
struct Presses {
    a: usize,
    b: usize
}

impl Presses {
    fn cost(&self) -> u128 {
        3 * self.a as u128 + self.b as u128
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
        // let mut row = LinearEquation::new(self.a_delta.row, self.b_delta.row, self.target.row).solve()?;
        // let mut col = LinearEquation::new(self.a_delta.col, self.b_delta.col, self.target.col).solve()?;
        // let mut change = true;
        // while change {
        //     println!("Row x range: {:?}", row.x_range);
        //     println!("Row y range: {:?}", row.y_range);
        //     println!("Col x range: {:?}", col.x_range);
        //     println!("Col y range: {:?}", col.y_range);
        //     change = row.limit(&col.x_range, &col.y_range);
        //     change = col.limit(&row.x_range, &row.y_range) || change;
        // }
        // let column_solutions = col.all();
        // println!("All col soln: {:?}", col.all());
        // let row_solutions = row.all();
        // println!("All row soln: {:?}", row.all());
        // let mut min_press: Option<Presses> = None;
        // for rs in row_solutions {
        //     if column_solutions.contains(&rs) {
        //         // found a way to win, keep track of cheapest
        //         let (a, b) = rs;
        //         let new = Presses {
        //             a, b
        //         };
        //         if let Some(existing) = &min_press {
        //             if new.cost() < existing.cost() {
        //                 min_press = Some(new);
        //             }
        //         } else {
        //             min_press = Some(new);
        //         }
        //     }
        // }
        // min_press
        println!("Look for win");
        let row = LinearEquation::new(self.a_delta.row, self.b_delta.row, self.target.row).solve()?;
        let col = LinearEquation::new(self.a_delta.col, self.b_delta.col, self.target.col).solve()?;
        let mut row_col = LinearEquation::new(self.a_delta.row + self.a_delta.col, self.b_delta.row + self.b_delta.col, self.target.row + self.target.col).solve()?;
        // println!("x_range: {:?}", row_col.x_range);
        // println!("y_range: {:?}", row_col.y_range);
        // println!("Got soln");
        row_col.limit(&row.x_range, &row.y_range)?;
        row_col.limit(&col.x_range, &col.y_range)?;
        // println!("x_range: {:?}", row_col.x_range);
        // println!("y_range: {:?}", row_col.y_range);
        // println!("limited");
        // let reasonable = LinearRange { min: 0, max: 100000000 };
        // row_col.limit(&reasonable, &reasonable);
        // println!("x_range: {:?}", row_col.x_range);
        // println!("y_range: {:?}", row_col.y_range);
        // println!("artificial limit");
        let (a,b) = row_col.first_satifying(&row, &col)?;
        return Some(Presses {a, b});
        let all_solutions = row_col.all();
        println!("found all: {}", all_solutions.len());
        let mut min_press: Option<Presses> = None;
        for (a, b) in all_solutions {
            if row.is_solution(a as isize, b as isize) && col.is_solution(a as isize,b as isize) {
                // found a way to win, keep track of cheapest
                //println!("found one: {},{}", a,b);
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
        println!("returned: {:?}", min_press);
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
        println!("Part1 tokens: {:?}", part1);
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
            let mut soln = LinearSolution::from(self);
            let x = LinearRange {
                min: 0.0,
                max: self.c as f64 / self.a as f64
            };
            let y = LinearRange {
                min: 0.0,
                max: self.c as f64 / self.b as f64
            };
            soln.limit(&x, &y)?;
            Some(soln)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct LinearSolution {
    a: isize,
    b: isize,
    c: isize,
    d: isize,
    x0: isize,
    y0: isize,
    x_range: LinearRange,
    y_range: LinearRange,
}

impl LinearSolution {
    fn from(equation: &LinearEquation) -> Self {
        let mut x0 = 0;
        let a = equation.a as isize;
        let b = equation.b as isize;
        let c = equation.c as isize;
        let d = equation.d as isize;
        loop {
            let numerator = d - x0 * a;
            let denominator = b;
            if numerator.rem_euclid(denominator) == 0 {
                let y0 = numerator / denominator;
                let min_m = -1 * c * y0 / a;
                let max_m = c * x0 / b;
                let x1 = c as f64 * x0 as f64 / d as f64 - min_m as f64 * b as f64 / d as f64;
                let x2 = c as f64 * x0 as f64 / d as f64 - max_m as f64 * b as f64 / d as f64;
                let y1 = a as f64 * min_m as f64 / d as f64 + c as f64 * y0 as f64 / d as f64;
                let y2 = a as f64 * max_m as f64 / d as f64 + c as f64 * y0 as f64 / d as f64;
                let x_range = LinearRange::from(x1, x2);
                let y_range = LinearRange::from(y1, y2);
                return Self {
                    a, b, c, d, x0, y0, x_range, y_range
                };
            }
            x0 += 1;
        }
    }

    fn limit(&mut self, x: &LinearRange, y: &LinearRange) -> Option<()> {
        self.x_range = self.x_range.overlap(x)?;
        self.y_range = self.y_range.overlap(y)?;
        Some(())
    }

    fn all(&self) -> HashSet<(usize, usize)> {
        let (min_m, max_m) = self.get_m_range();
        let mut solutions = HashSet::new();
        for m in min_m..(max_m + 1) {
            let x = self.c * self.x0 / self.d - m * self.b / self.d;
            let y = self.a * m / self.d + self.c * self.y0 / self.d;
            if self.is_solution(x, y) {
                solutions.insert((x as usize, y as usize));
            }
        }
        solutions
    }

    fn first_satifying(&self, other1: &LinearSolution, other2: &LinearSolution) -> Option<(usize, usize)> {
        let (min_m, max_m) = self.get_m_range();
        for m in min_m..(max_m + 1) {
            let x = self.c * self.x0 / self.d - m * self.b / self.d;
            let y = self.a * m / self.d + self.c * self.y0 / self.d;
            if other1.is_solution(x, y) && other2.is_solution(x, y) {
                return Some((x as usize, y as usize));
            }
        }
        None
    }

    fn is_solution(&self, x: isize, y: isize) -> bool {
        self.a * x + self.b * y == self.c
    }

    fn get_m_range(&self) -> (isize, isize) {
        let m_x1 = self.c as f64 * self.x0 as f64 / self.b as f64 - self.x_range.min * self.d as f64 / self.b as f64;
        let m_x2 = self.c as f64 * self.x0 as f64 / self.b as f64 - self.x_range.max * self.d as f64 / self.b as f64;
        let (m_min_x, m_max_x) = if m_x1 > m_x2 {
            (m_x2, m_x1)
        } else {
            (m_x1, m_x2)
        };
        let m_y1 = self.y_range.min * self.d as f64 / self.a as f64 - self.c as f64 * self.y0 as f64 / self.a as f64;
        let m_y2 = self.y_range.max * self.d as f64 / self.a as f64 - self.c as f64 * self.y0 as f64 / self.a as f64;
        let (m_min_y, m_max_y) = if m_y1 > m_y2 {
            (m_y2, m_y1)
        } else {
            (m_y1, m_y2)
        };
        let min_m = m_min_x.max(m_min_y);
        let max_m = m_max_x.min(m_max_y);
        (min_m.ceil() as isize, max_m.floor() as isize)
    }
}

#[derive(Debug, Copy, Clone)]
struct LinearRange {
    min: f64,
    max: f64
}

impl LinearRange {
    fn from(a: f64, b: f64) -> Self {
        if a > b {
            Self {
                min: b,
                max: a
            }
        } else {
            Self {
                min: a,
                max: b
            }
        }
    }

    fn overlap(&self, other: &Self) -> Option<Self> {
        let a1 = self.min;
        let b1 = self.max;
        let a2 = other.min;
        let b2 = other.max;
        if a2 <= a1 && b2 >= b1 {
            Some(*self)
        } else if a2 >= a1 && b2 <= b1 {
            Some(*other)
        } else if a2 < a1 && b2 < a1 {
            None
        } else if a2 > b1 && b2 > b1 {
            None
        } else if a2 <= a1 && b2 >= a1 {
            Some(Self { min: a1, max: b2 })
        } else if a2 <= b1 && b2 >= b1 {
            Some(Self { min: a2, max: b1 })
        } else {
            panic!()
        }
    }
}