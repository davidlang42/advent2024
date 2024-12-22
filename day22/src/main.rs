use std::fs;
use std::env;
use std::ops::BitXor;

#[derive(Debug)]
struct Secret {
    number: usize,
    prices: Vec<u8>,
    deltas: Vec<i8>
}

impl Secret {
    fn new(number: usize) -> Self {
        let mut s = Self {
            number,
            prices: Vec::new(),
            deltas: Vec::new()
        };
        s.prices.push(s.price());
        s
    }

    fn next_number(&mut self) {
        self.number = Self::prune(Self::mix(self.number * 64, self.number));
        self.number = Self::prune(Self::mix(self.number / 32, self.number));
        self.number = Self::prune(Self::mix(self.number * 2048, self.number));
        self.prices.push(self.price());
        self.deltas.push(self.prices[self.prices.len() - 1] as i8 - self.prices[self.prices.len() - 2] as i8); 
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

    fn price_after_deltas(&self, deltas: [i8; 4]) -> Option<u8> {
        for i in 0..(self.deltas.len() - 3) {
            if self.deltas[i..(i+4)] == deltas {
                return Some(self.prices[i+4]);
            }
        }
        None
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
        println!("Part1 sum: {:?}", sum);
        let min = -9;
        let max = 10;
        let mut best_bananas: usize = 0;
        let mut best = None;
        for a in min..max {
            for b in min..max {
                for c in min..max {
                    for d in min..max {
                        let mut bananas: usize = 0;
                        for s in &secrets {
                            if let Some(price) = s.price_after_deltas([a,b,c,d]) {
                                bananas += price as usize;
                            }
                        }
                        if bananas > best_bananas {
                            best_bananas = bananas;
                            best = Some([a,b,c,d]);
                        }
                    }
                } 
            }
        }
        println!("Best bananas: {} ({:?})", best_bananas, best);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}