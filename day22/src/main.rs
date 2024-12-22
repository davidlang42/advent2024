use std::fs;
use std::env;
use std::ops::BitXor;

#[derive(Debug)]
struct Secret {
    number: usize,
    price_deltas: Vec<i8>
}

impl Secret {
    fn new(number: usize) -> Self {
        Self {
            number,
            price_deltas: Vec::new()
        }
    }

    fn next_number(&mut self) {
        let price_before = self.price();
        self.number = Self::prune(Self::mix(self.number * 64, self.number));
        self.number = Self::prune(Self::mix(self.number / 32, self.number));
        self.number = Self::prune(Self::mix(self.number * 2048, self.number));
        let price_after = self.price();
        self.price_deltas.push(price_before as i8 - price_after as i8);
    }

    fn price(&self) -> u8 {
        (self.number % 10) as u8
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
        let mut secrets: Vec<Secret> = text.lines().map(|s| Secret::new(s.parse::<usize>().unwrap())).collect();
        for _i in 0..2000 {
            for s in &mut secrets {
                s.next_number();
            }
        }
        let sum: usize = secrets.iter().map(|s| s.number).sum();
        println!("Sum: {:?}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}