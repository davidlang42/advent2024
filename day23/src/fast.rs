use crate::{Computer, Selection};

#[derive(Debug)]
pub struct FastNetwork<const N: usize> {
    pub pcs: [Computer; N],
    pub map: [Selection<N>; N]
}

impl<const N: usize> FastNetwork<N> {
    pub fn largest(&self) -> Selection<N> {
        let mut largest: Option<Selection<N>> = None;
        // let mut avoid = HashSet::new();
        // let mut last = Instant::now();
        // let mut expand_cache = HashMap::new();
        // let mut common_cache = HashMap::new();
        for a in 0..N {
            // expand_cache.clear();
            // common_cache.clear();
            // println!("Starting {:?} ({}/{}={}%)", a, avoid.len(), self.map.len(), avoid.len() as f64 * 100.0 / self.map.len() as f64);
            let lan = Selection::one(a);
            largest = Some(self.expand_selection_to_largest(lan, largest));
            // avoid.insert(*a);
            // let duration = Instant::now() - last;
            // println!("Took {}s (expand cache: {}, common cache: {})", duration.as_secs(), expand_cache.len(), common_cache.len());
            // last = Instant::now();
        }
        largest.unwrap()
    }

    fn expand_selection_to_largest(&self, lan: Selection<N>, mut largest: Option<Selection<N>>) -> Selection<N> {
        let common = self.common_connections(&lan);
        if common.count() == 0 {
            // no further expansion possible
            if let Some(existing) = largest {
                if lan.count() > existing.count() {
                    println!("New largest: {:?}", lan.0);
                    return lan;
                } else {
                    return existing;
                }
            } else {
                println!("Default largest: {:?}", lan.0);
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

    fn common_connections(&self, pcs: &Selection<N>) -> Selection<N> {
        let indicies = pcs.selected();
        if indicies.len() == 0 {
            panic!("Tried to get common connections of nothing");
        } else {
            let mut common = self.map[indicies[0]].clone();
            for i in 1..indicies.len() {
                common.and(&self.map[indicies[i]]);
            }
            common
        }
    }
}