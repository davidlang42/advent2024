use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::HashSet;

struct Set {
    orders: Vec<PageOrder>,
    updates: Vec<Update>
}

#[derive(Debug, Copy, Clone)]
struct PageOrder {
    first: usize,
    second: usize
}

#[derive(Debug)]
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

    pub fn fix(&self, orders: &Vec<PageOrder>) -> Self {
        let mut remaining = HashSet::new();
        for n in &self.0 {
            remaining.insert(n);
        }
        let mut orders_that_matter: Vec<PageOrder> = Vec::new();
        for o in orders {
            if remaining.get(&o.first).is_some() && remaining.get(&o.second).is_some() { // otherwise it doesn't matter
                orders_that_matter.push(*o);
            }
        }
        let mut new_update = Vec::new();
        for next in &self.0 {
            let must_be_before_values: Vec<usize> = orders_that_matter.iter().filter(|o| o.first == *next).map(|o| o.second).collect();
            let must_be_before_indices = must_be_before_values.iter().map(|v| new_update.iter().position(|p| p == v)).filter(|i| i.is_some()).map(|i| i.unwrap());
            if let Some(can_go_at_index) = must_be_before_indices.min() {
                new_update.insert(can_go_at_index, *next);
            } else {
                new_update.push(*next);
            }
            
        }
        Self(new_update)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let set: Set = text.parse().unwrap();
        let mut correct_sum = 0;
        let mut fixed_sum = 0;
        for u in set.updates {
            if u.all_valid(&set.orders) {
                correct_sum += u.middle_number();
            } else {
                fixed_sum += u.fix(&set.orders).middle_number();
            }
        }
        println!("Correct sum: {}", correct_sum);
        println!("Fixed sum: {}", fixed_sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}