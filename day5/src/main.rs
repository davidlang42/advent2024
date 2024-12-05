use std::fs;
use std::env;
use std::str::FromStr;

struct Set {
    orders: Vec<PageOrder>,
    updates: Vec<Update>
}

struct PageOrder {
    first: usize,
    second: usize
}

struct Update(Vec<usize>);

impl FromStr for Set {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let sections: Vec<&str> = text.split("\r\n\r\n").collect();
        if sections.len() != 2 {
            panic!("Incorrect number of sections: {}", sections.len());
        }
        let orders = sections[0].lines().map(|l| l.parse().unwrap()).collect();
        let updates = sections[1].lines().map(|l| l.parse().unwrap()).collect();
        Ok(Self {
            orders,
            updates
        })
    }
}

impl FromStr for PageOrder {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let pages: Vec<usize> = line.split("|").map(|p| p.parse().unwrap()).collect();
        if pages.len() != 2 {
            panic!("PageOrder has {} pages", pages.len());
        }
        Ok(Self {
            first: pages[0],
            second: pages[1]
        })
    }
}

impl FromStr for Update {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let pages: Vec<usize> = line.split(",").map(|p| p.parse().unwrap()).collect();
        Ok(Self(pages))
    }
}

impl Update {
    pub fn all_valid(&self, orders: &Vec<PageOrder>) -> bool {
        orders.iter().all(|o| self.one_valid(o))
    }

    fn one_valid(&self, order: &PageOrder) -> bool {
        let p1 = self.0.iter().position(|&p| p == order.first);
        let p2 = self.0.iter().position(|&p| p == order.second);
        if p1.is_none() || p2.is_none() {
            true
        } else {
            p1.unwrap() < p2.unwrap()
        }
    }

    pub fn middle_number(&self) -> usize {
        let middle: usize = self.0.len() / 2;
        self.0[middle]
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let set: Set = text.parse().unwrap();
        let mut sum = 0;
        for u in set.updates {
            if u.all_valid(&set.orders) {
                sum += u.middle_number();
            }
        }
        println!("Sum: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}