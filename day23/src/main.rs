use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::{HashSet, HashMap};
use crate::fast::FastNetwork;

type Computer = [char; 2];

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Selection<const N: usize>([bool; N]);

impl<const N: usize> Selection<N> {
    fn one(index: usize) -> Self {
        let mut bools = [false; N];
        bools[index] = true;
        Self(bools)
    }

    fn selected(&self) -> Vec<usize> {
        let mut indicies = Vec::new();
        for i in 0..N {
            if self.0[i] {
                indicies.push(i);
            }
        }
        indicies
    }

    fn count(&self) -> usize {
        let mut count = 0;
        for i in 0..N {
            if self.0[i] {
                count += 1;
            }
        }
        count
    }

    fn and(&mut self, other: &Self) {
        for i in 0..N {
            self.0[i] &= other.0[i]
        }
    }
}

struct Pair(Computer, Computer);

mod fast;

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

    fn to_fast<const N: usize>(&self) -> FastNetwork<N> {
        if self.pcs.len() != N {
            panic!("Wrong length: {}", self.pcs.len());
        }
        let mut pcs = Vec::new();
        let mut map = Vec::new();
        for pc in self.pcs.iter() {
            pcs.push(pc.clone());
            let set = self.map.get(pc).unwrap();
            let mut selected = Vec::new();
            for mapped_pc in self.pcs.iter() {
                selected.push(set.contains(mapped_pc));
            }
            map.push(Selection(selected.try_into().unwrap()));
        }
        FastNetwork {
            pcs: pcs.try_into().unwrap(),
            map: map.try_into().unwrap(),
            common_cache: HashMap::new(),
            expand_cache: HashMap::new()
        }
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
        if network.pcs.len() == 16 {
            let mut fast: FastNetwork<16> = network.to_fast();
            let largest = fast.largest();
            println!("{}", fast.display(&largest));
        } else if network.pcs.len() == 520 {
            let mut fast: FastNetwork<520> = network.to_fast();
            let largest = fast.largest();
            println!("{}", fast.display(&largest));
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}