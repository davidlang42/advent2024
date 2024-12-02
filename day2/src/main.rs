use std::fs;
use std::env;
use std::str::FromStr;

#[derive(Debug)]
struct Report {
    levels: Vec<usize>
}

impl FromStr for Report {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let levels: Vec<usize> = line.split(" ").map(|n| n.parse().unwrap()).collect();
        Ok(Self{ levels })
    }
}

impl Report {
    fn reversed(&self) -> Self {
        Self {
            levels: self.levels.iter().rev().cloned().collect::<Vec<usize>>()
        }
    }

    pub fn is_safe(&self) -> bool {
        self.is_gradually_ascending() || self.reversed().is_gradually_ascending()
    }

    fn is_gradually_ascending(&self) -> bool {
        let mut last = self.levels[0];
        for i in 1..self.levels.len() {
            let delta = self.levels[i] as isize - last as isize;
            if delta < 1 || delta > 3 {
                return false;
            }
            last = self.levels[i];
        }
        true
    }

    pub fn is_safe_with_dampener(&self) -> bool {
        if self.is_safe() {
            return true;
        }
        for i in 0..self.levels.len() {
            let mut subset = Vec::new();
            for j in 0..self.levels.len() {
                if i != j {
                    subset.push(self.levels[j]);
                }
            }
            let sub_report = Self {
                levels: subset
            };
            if sub_report.is_safe() {
                return true;
            }
        }
        false
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let reports: Vec<Report> = text.lines().map(|s| s.parse().unwrap()).collect();
        let mut safe = 0;
        let mut with_damp = 0;
        for r in reports {
            if r.is_safe() {
                safe += 1;
            }
            if r.is_safe_with_dampener() {
                with_damp += 1;
            }
        }
        println!("Safe: {}", safe);
        println!("With dampener: {}", with_damp);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}