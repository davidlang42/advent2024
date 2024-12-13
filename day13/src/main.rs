use std::fs;
use std::env;
use std::str::FromStr;

#[derive(Debug)]
struct Claw {
    a_delta: Pos,
    b_delta: Pos,
    target: Pos
}

#[derive(Debug)]
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
    fn add(&mut self, delta: &Pos, times: usize) {
        self.row += delta.row * times;
        self.col += delta.col * times;
    }
}

impl Claw {
    fn win(&self) -> Option<Presses> {
        let mut min_press: Option<Presses> = None;
        for b in 0..101 {
            let mut pos = Pos {
                row: 0,
                col: 0
            };
            pos.add(&self.b_delta, b);
            if pos.row > self.target.row || pos.col > self.target.col {
                break;
            }
            for a in 0..101 {
                if pos.row == self.target.row && pos.col == self.target.col {
                    // found a way to win, keep track of cheapest
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
                pos.add(&self.a_delta, 1);
                if pos.row > self.target.row || pos.col > self.target.col {
                    break;
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
        let claws: Vec<Claw> = text.split("\r\n\r\n").map(|s| s.parse().unwrap()).collect();
        let mut tokens = 0;
        for claw in claws {
            if let Some(presses) = claw.win() {
                let cost = presses.cost();
                tokens += cost;
                println!("Press Ax{}, Bx{} = {} tokens", presses.a, presses.b, cost);
            } else {
                println!("Can't be won");
            }
        }
        println!("Total tokens: {:?}", tokens);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}