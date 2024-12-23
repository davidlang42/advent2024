use crate::Code;
use crate::directional::{Direction, DirectionalKey};
use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::Add;
use pathfinding::num_traits::Zero;
use pathfinding::prelude::astar_bag;

pub trait Key : Sized + Default + Clone + Hash + Eq + PartialEq {
    fn from_char(c: char) -> Self;
    fn to_char(&self) -> char;
    fn key_above(&self) -> Option<Self>;
    fn key_below(&self) -> Option<Self>;
    fn key_left(&self) -> Option<Self>;
    fn key_right(&self) -> Option<Self>;
    fn row(&self) -> usize;
    fn col(&self) -> usize;
    
    fn minimum_distance_to(&self, other: &Self) -> usize {
        self.row().abs_diff(other.row()) + self.col().abs_diff(other.col())
    }
}

#[derive(Clone)]
struct Cost {
    movements: Vec<DirectionalKey>
}

impl Cost {
    fn from(directional_key: DirectionalKey) -> Self {
        Self {
            movements: vec![directional_key]
        }
    }

    fn of(size: usize) -> Self {
        let mut movements = Vec::new();
        for _ in 0..size {
            movements.push(DirectionalKey::Activate);
        }
        Self {
            movements
        }
    }
}

impl PartialEq for Cost {
    fn eq(&self, other: &Self) -> bool {
        self.movements.len() == other.movements.len()
    }
}

impl Eq for Cost {

}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.movements.len().partial_cmp(&other.movements.len())
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> Ordering {
        self.movements.len().cmp(&other.movements.len())
    }
}

impl Add for Cost {
    fn add(self, mut rhs: Cost) -> Self {
        let mut movements = self.movements.clone();
        movements.append(&mut rhs.movements);
        Self {
            movements
        }
    }
    
    type Output = Self;
}

impl Zero for Cost {
    fn zero() -> Self {
        Self {
            movements: Vec::new()
        }
    }

    fn is_zero(&self) -> bool {
        self.movements.len() == 0
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Keypad<K: Key> {
    pub current: K
}

impl<K: Key> Keypad<K> {
    pub fn new() -> Self {
        Self{
            current: K::default()
        }
    }

    pub fn shortest_paths_to_code(&self, code: &Code<K>) -> Vec<Self> {
        Self::shortest_paths_to_code_recursive(self, &code.keys)
    }

    //TODO add caching if needed
    fn shortest_paths_to_code_recursive(start: &Self, code: &[K]) -> Vec<Self> {
        todo!()
        // let mut results = Vec::new();
        // for mut result in Self::shortest_paths_to_key(start, &code[0]) {
        //     result.movements.push(DirectionalKey::Activate);
        //     if code.len() == 1 {
        //         // this is a finished result, these will always be the shortest (due to astar_bag)
        //         results.push(result);
        //     } else {
        //         // continue (recurisively) to the next key
        //         results.append(&mut Self::shortest_paths_to_code_recursive(&result, &code[1..code.len()]));
        //     }
        // }
        // // filter out results which are no longer the shortest (due to combining with upstream results)
        // let shortest = results.iter().map(|r| r.movements.len()).min().unwrap();
        // results.into_iter().filter(|r| r.movements.len() == shortest).collect()
    }

    fn shortest_paths_to_key(start: &Self, key: &K) -> Vec<Self> {
        let (results, cost) = astar_bag(start, |kp| kp.successors(), |kp| Cost::of(kp.current.minimum_distance_to(key)), |kp| kp.current == *key).expect("No solution");
        results.into_iter().map(|r| r.into_iter().last().unwrap()).collect()
    }

    fn successors(&self) -> Vec<(Self, Cost)> {
        let mut v = Vec::new();
        for direction in self.valid_moves() {
            let mut clone = self.clone();
            clone.move_current(&direction);
            v.push((clone, Cost::from(DirectionalKey::Move(direction))));
        }
        v
    }

    fn valid_moves(&self) -> Vec<Direction> {
        let mut v = Vec::new();
        if !self.current.key_above().is_none() {
            v.push(Direction::Up)
        }
        if !self.current.key_below().is_none() {
            v.push(Direction::Down)
        }
        if !self.current.key_left().is_none() {
            v.push(Direction::Left)
        }
        if !self.current.key_right().is_none() {
            v.push(Direction::Right)
        }
        v
    }

    fn move_current(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.current = self.current.key_above().unwrap(),
            Direction::Down => self.current = self.current.key_below().unwrap(),
            Direction::Left => self.current = self.current.key_left().unwrap(),
            Direction::Right => self.current = self.current.key_right().unwrap(),
        }
    }
}