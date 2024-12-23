use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::{HashSet, HashMap};

struct Pair(String, String);

impl FromStr for Pair {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let computers: Vec<&str> = line.split("-").collect();
        if computers.len()!= 2 {
            panic!()
        }
        Ok(Self(computers[0].to_string(), computers[1].to_string()))
    }
}

struct Network(HashMap<String, HashSet<String>>);

impl Network {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn from(pairs: Vec<Pair>) -> Self {
        let mut map = Self::new();
        for p in pairs {
            map.add(p.0.clone(), p.1.clone());
            map.add(p.1, p.0);
        }
        map
    }

    fn add(&mut self, k: String, v: String) {
        if let Some(existing) = self.0.get_mut(&k) {
            existing.insert(v);
        } else {
            let mut new = HashSet::new();
            new.insert(v);
            self.0.insert(k, new);
        }
    }

    fn triples(&self, starts_with: &str) -> HashSet<(String, String, String)> {
        let mut set = HashSet::new();
        for (a, a_set) in &self.0 {
            for b in a_set {
                if let Some(b_set) = self.0.get(b) {
                    for c in a_set.intersection(&b_set) {
                        let mut triple = vec![a, b, c];
                        if triple.iter().any(|s| s.starts_with(starts_with)) {
                            triple.sort(); // in alphabetical order to de-duplicate
                            set.insert((triple[0].clone(), triple[1].clone(), triple[2].clone()));
                        }
                    }
                }
            }
        }
        set
    }

    fn largest(&self) -> Vec<String> {
        
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let connections: Vec<Pair> = text.lines().map(|s| s.parse().unwrap()).collect();
        let network = Network::from(connections);
        let triples = network.triples("t");
        println!("Triples ({}):", triples.len());
        let mut largest = network.largest();
        largest.sort();
        for s in largest {
            print!("{}", s);
        }
        println!("");
    } else {
        println!("Please provide 1 argument: Filename");
    }
}