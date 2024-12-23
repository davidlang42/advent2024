use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::{HashSet, HashMap};
//use std::time::Instant;

type Computer = [char; 2];

struct Pair(Computer, Computer);

impl FromStr for Pair {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let computers: Vec<&str> = line.split("-").collect();
        if computers.len()!= 2 {
            panic!()
        }
        Ok(Self(computers[0].chars().collect::<Vec<char>>().try_into().unwrap(), computers[1].chars().collect::<Vec<char>>().try_into().unwrap()))
    }
}

struct Network {
    pcs: Vec<Computer>,
    map: HashMap<Computer, HashSet<Computer>>
}

struct SelectedComputers(Vec<bool>);

impl Network {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            pcs: Vec::new()
        }
    }

    fn from(pairs: Vec<Pair>) -> Self {
        let mut map = Self::new();
        for p in pairs {
            map.add(p.0.clone(), p.1.clone());
            map.add(p.1, p.0);
        }
        map
    }

    fn add(&mut self, k: Computer, v: Computer) {
        if let Some(existing) = self.map.get_mut(&k) {
            existing.insert(v);
        } else {
            let mut new = HashSet::new();
            new.insert(v);
            self.map.insert(k.clone(), new);
            self.pcs.push(k);
            self.pcs.sort();
        }
    }

    fn triples(&self, starts_with: char) -> HashSet<(Computer, Computer, Computer)> {
        let mut set = HashSet::new();
        for (a, a_set) in &self.map {
            for b in a_set {
                if let Some(b_set) = self.map.get(b) {
                    for c in a_set.intersection(&b_set) {
                        let mut triple = vec![a, b, c];
                        if triple.iter().any(|s| s[0] == starts_with) {
                            triple.sort(); // in alphabetical order to de-duplicate
                            set.insert((triple[0].clone(), triple[1].clone(), triple[2].clone()));
                        }
                    }
                }
            }
        }
        set
    }

    fn largest(&self) -> Lan {
        let mut largest: Option<Lan> = None;
        // let mut avoid = HashSet::new();
        // let mut last = Instant::now();
        // let mut expand_cache = HashMap::new();
        // let mut common_cache = HashMap::new();
        for a in self.map.keys() {
            // expand_cache.clear();
            // common_cache.clear();
            // println!("Starting {:?} ({}/{}={}%)", a, avoid.len(), self.map.len(), avoid.len() as f64 * 100.0 / self.map.len() as f64);
            let lan = Lan::from(a.clone());
            largest = Some(lan.largest_expansion(self, largest));
            // avoid.insert(*a);
            // let duration = Instant::now() - last;
            // println!("Took {}s (expand cache: {}, common cache: {})", duration.as_secs(), expand_cache.len(), common_cache.len());
            // last = Instant::now();
        }
        largest.unwrap()
    }

    fn common_connections(&self, pcs: &HashSet<Computer>) -> HashSet<Computer> {
        let mut common = HashSet::new();
        for (other_pc, other_pc_connects_to) in &self.map {
            if pcs.iter().all(|pc| other_pc_connects_to.contains(pc)) {
                common.insert(other_pc.clone());
            }
        }
        common
    }
}

#[derive(Clone)]
struct Lan(HashSet<Computer>);

impl Lan {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn from(computer: Computer) -> Self {
        let mut set = HashSet::new();
        set.insert(computer);
        Self(set)
    }

    fn largest_expansion(self, network: &Network, mut largest: Option<Self>) -> Self {
        let common = network.common_connections(&self.0);
        if common.len() == 0 {
            // no further expansion possible
            if let Some(existing) = largest {
                if self.size() > existing.size() {
                    println!("New largest: {:?}", self.0);
                    return self;
                } else {
                    return existing;
                }
            } else {
                println!("Default largest: {:?}", self.0);
                return self;
            }
        }
        for c in common {
            let mut option = self.clone();
            option.0.insert(c.clone());
            largest = Some(option.largest_expansion(network, largest));
        }
        largest.unwrap()
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
        let triples = network.triples('t');
        println!("Triples: {}", triples.len());
        let mut largest: Vec<Computer> = network.largest().0.into_iter().collect();
        largest.sort();
        for s in largest {
            print!("{}{}", s[0], s[1]);
        }
        println!("");
    } else {
        println!("Please provide 1 argument: Filename");
    }
}