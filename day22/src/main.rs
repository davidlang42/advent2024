use std::fs;
use std::env;
use std::ops::BitXor;

#[derive(Debug)]
struct Secret(usize);

impl Secret {
    fn next_number(&mut self) {
        self.0 = Self::prune(Self::mix(self.0 * 64, self.0));
        self.0 = Self::prune(Self::mix(self.0 / 32, self.0));
        self.0 = Self::prune(Self::mix(self.0 * 2048, self.0));
    }

    fn mix(a: usize, b: usize) -> usize {
        a.bitxor(b)
    }

    fn prune(a: usize) -> usize {
        a % 16777216
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut secrets: Vec<Secret> = text.lines().map(|s| Secret(s.parse::<usize>().unwrap())).collect();
        for _i in 0..2000 {
            for s in &mut secrets {
                s.next_number();
            }
        }
        let sum: usize = secrets.iter().map(|s| s.0).sum();
        println!("Sum: {:?}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}