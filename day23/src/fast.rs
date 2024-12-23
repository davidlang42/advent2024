use crate::{Computer, Selection};
use std::time::Instant;
use std::collections::HashMap;

#[derive(Debug)]
pub struct FastNetwork<const N: usize> {
    pub pcs: [Computer; N],
    pub map: [Selection<N>; N],
    pub common_cache: HashMap<Selection<N>, Selection<N>>
}

impl<const N: usize> FastNetwork<N> {
    pub fn display(&self, selection: &Selection<N>) -> String {
        let mut s = String::new();
        for i in selection.selected() {
            s.push(self.pcs[i][0]);
            s.push(self.pcs[i][1]);
        }
        s
    }

    pub fn largest(&mut self) -> Selection<N> {
        let mut largest: Option<Selection<N>> = None;
        // let mut avoid = HashSet::new();
        let mut last = Instant::now();
        // let mut expand_cache = HashMap::new();
        // let mut common_cache = HashMap::new();
        let mut progress = 0;
        for a in 0..N {
            // expand_cache.clear();
            // common_cache.clear();
            println!("Starting {:?} ({}/{}={}%)", a, progress, self.map.len(), progress as f64 * 100.0 / self.map.len() as f64);
            let lan = Selection::one(a);
            largest = Some(self.expand_selection_to_largest(lan, largest));
            // avoid.insert(*a);
            let duration = Instant::now() - last;
            progress += 1;
            println!("Took {}s", duration.as_secs());
            //println!("Took {}s (expand cache: {}, common cache: {})", duration.as_secs(), expand_cache.len(), common_cache.len());
            last = Instant::now();
        }
        largest.unwrap()
    }

    fn expand_selection_to_largest(&mut self, lan: Selection<N>, mut largest: Option<Selection<N>>) -> Selection<N> {
        let common = self.common_connections(&lan);
        if common.count() == 0 {
            // no further expansion possible
            if let Some(existing) = largest {
                if lan.count() > existing.count() {
                    println!("New largest: {}", self.display(&lan));
                    return lan;
                } else {
                    return existing;
                }
            } else {
                println!("Default largest: {}", self.display(&lan));
                return lan;
            }
        }
        for i in common.selected() {
            let mut option = lan.clone();
            option.0[i] = true;
            largest = Some(self.expand_selection_to_largest(option, largest));
        }
        largest.unwrap()
    }

    fn common_connections(&mut self, pcs: &Selection<N>) -> Selection<N> {
        if let Some(cached) = self.common_cache.get(pcs) {
            cached.clone()
        } else {
            let indicies = pcs.selected();
            if indicies.len() == 0 {
                panic!("Tried to get common connections of nothing");
            }
            let mut common = self.map[indicies[0]].clone();
            for i in 1..indicies.len() {
                common.and(&self.map[indicies[i]]);
            }
            self.common_cache.insert(pcs.clone(), common.clone());
            common
        }
    }
}