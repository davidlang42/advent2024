use std::fs;
use std::env;
use std::str::FromStr;

#[derive(Debug)]
struct Grid {
    lines: Vec<Vec<char>>,
    cols: Vec<Vec<char>>,
    diagonals: Vec<Vec<char>>
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let lines: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
        let cols = transpose(&lines);
        let max_x = cols.len() - 1;
        let max_y = lines.len() - 1;
        let mut diagonals = Vec::new();
        for i in 0..(max_x + 1) {
            let mut x = i;
            let mut y = 0;
            let mut d = Vec::new();
            while x <= max_x && y <= max_y {
                d.push(lines[y][x]);
                x += 1;
                y += 1;
            }
            diagonals.push(d);
            
        }
        for i in 1..(max_y + 1) {
            let mut x = 0;
            let mut y = i;
            let mut d = Vec::new();
            while x <= max_x && y <= max_y {
                d.push(lines[y][x]);
                x += 1;
                y += 1;
            }
            diagonals.push(d);
        }
        for i in 0..(max_x + 1) {
            let mut x = i;
            let mut y = 0;
            let mut d = Vec::new();
            while y <= max_y {
                d.push(lines[y][x]);
                if x == 0 {
                    break;
                }
                x -= 1;
                y += 1;
            }
            diagonals.push(d);
            
        }
        for i in 1..(max_y + 1) {
            let mut x = max_x;
            let mut y = i;
            let mut d = Vec::new();
            while y <= max_y {
                d.push(lines[y][x]);
                if x == 0 {
                    break;
                }
                x -= 1;
                y += 1;
            }
            diagonals.push(d);
        }
        Ok(Self{ lines, cols, diagonals })
    }
}

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

impl Grid {
    pub fn find(&self, s: &[char; 4]) -> usize {
        let mut total = 0;
        for l in &self.lines {
            total += Self::find_in_vec(s, l);
            total += Self::find_in_vec(s, &l.iter().rev().cloned().collect());
        }
        for c in &self.cols {
            total += Self::find_in_vec(s, c);
            total += Self::find_in_vec(s, &c.iter().rev().cloned().collect());
        }
        for d in &self.diagonals {
            total += Self::find_in_vec(s, d);
            total += Self::find_in_vec(s, &d.iter().rev().cloned().collect());
        }
        total
    }

    fn find_in_vec(s: &[char; 4], v: &Vec<char>) -> usize {
        if v.len() < 4 {
            return 0;
        }
        let mut count = 0;
        for i in 0..(v.len() - 4 + 1) {
            if &v[i..(i+4)] == s {
                count += 1;
            }
        }
        count
    }

    pub fn find_x(&self) -> usize {
        Self::find_x_vec(&self.lines)
    }

    fn find_x_vec(v: &Vec<Vec<char>>) -> usize {
        let mut count = 0;
        for y in 0..(v.len() - 2) {
            for x in 0..(v[0].len() -2) {
                if (v[y][x] == 'M' && v[y + 1][x + 1] == 'A' && v[y + 2][x] == 'M' && v[y][x + 2] == 'S' && v[y + 2][x + 2] == 'S')
                    || (v[y][x] == 'M' && v[y + 1][x + 1] == 'A' && v[y + 2][x] == 'S' && v[y][x + 2] == 'M' && v[y + 2][x + 2] == 'S')
                    || (v[y][x] == 'S' && v[y + 1][x + 1] == 'A' && v[y + 2][x] == 'M' && v[y][x + 2] == 'S' && v[y + 2][x + 2] == 'M')
                    || (v[y][x] == 'S' && v[y + 1][x + 1] == 'A' && v[y + 2][x] == 'S' && v[y][x + 2] == 'M' && v[y + 2][x + 2] == 'M')
                {
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let grid: Grid = text.parse().unwrap();
        let search: [char; 4] = ['X','M','A','S'];
        println!("Count: {}", grid.find(&search));
        println!("X-Count: {}", grid.find_x());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}