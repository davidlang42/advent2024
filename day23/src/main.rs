use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::{HashSet, HashMap};

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

struct Network{
    map: HashMap<Computer, HashSet<Computer>>
}

impl Network {
    fn new() -> Self {
        Self {
            map: HashMap::new()
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
            self.map.insert(k, new);
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

    fn largest(&self) -> Vec<Computer> {
        let mut largest: Option<Lan> = None;
        for l in self.all_lans() {
            if largest.is_none() || largest.as_ref().unwrap().size() < l.size() {
                largest = Some(l);
            }
        }
        largest.unwrap().0.into_iter().collect::<Vec<Computer>>()
    }

    fn all_lans(&self) -> Vec<Lan> {
        let mut v = Vec::new();
        let mut cache = HashMap::new();
        for a in self.map.keys() {
            let lan = Lan::from(a.clone());
            v.append(&mut lan.expand(self, &mut cache));
        }
        v
    }

    fn common_connections(&self, pcs: &HashSet<Computer>, cache: &mut HashMap<Vec<Computer>,HashSet<Computer>>) -> HashSet<Computer> {
        let mut pcs_vec: Vec<_> = pcs.iter().cloned().collect();
        pcs_vec.sort();
        if let Some(cached) = cache.get(&pcs_vec) {
            cached.clone()
        } else {
            let mut common = HashSet::new();
            for (other_pc, other_pc_connects_to) in &self.map {
                if pcs.iter().all(|pc| other_pc_connects_to.contains(pc)) {
                    common.insert(other_pc.clone());
                }
            }
            cache.insert(pcs_vec, common.clone());
            common
        }
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

    fn expand(self, network: &Network, cache: &mut HashMap<Vec<Computer>,HashSet<Computer>>) -> Vec<Self> {
        let common = network.common_connections(&self.0, cache);
        if common.len() == 0 {
            // no further expansion possible
            if self.0.len() > 12 {
                println!("{:?}", self.0);
            }
            return vec![self];
        }
        let mut options = Vec::new();
        for c in common {
            let mut option = self.clone();
            option.0.insert(c.clone());
            for sub_option in option.expand(network, cache) {
                options.push(sub_option);
            }
        }
        options
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
        let mut largest = network.largest();
        largest.sort();
        for s in largest {
            print!("{}{}", s[0], s[1]);
        }
        println!("");
    } else {
        println!("Please provide 1 argument: Filename");
    }
}