use crate::Code;
use crate::directional::{Direction, DirectionalKey};
use std::collections::HashMap;
use std::hash::Hash;
use pathfinding::prelude::astar_bag;

pub trait Key : Sized + Default + Clone + Copy + Hash + Eq + PartialEq {
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

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Keypad<K: Key> {
    pub current: K,
    pub movements: Vec<DirectionalKey>
}

impl<K: Key> Keypad<K> {
    pub fn new() -> Self {
        Self{
            current: K::default(),
            movements: Vec::new()
        }
    }

    pub fn _movements_string(&self) -> String {
        let mut s = String::new();
        for key in &self.movements {
            s.push(key.to_char());
        }
        s
    }

    pub fn shortest_paths_to_code(&self, code: &Code<K>, cache: &mut HashMap<(K, Vec<K>), Vec<Self>>) -> Vec<Self> {
        let mut results = Vec::new();
        for mut sub_result in Self::shortest_paths_to_code_recursive(&self.current, &code.keys, cache) {
            let mut new_movements = self.movements.clone();
            new_movements.append(&mut sub_result.movements);
            sub_result.movements = new_movements;
            results.push(sub_result);
        }
        results
    }

    fn shortest_paths_to_code_recursive(start_key: &K, code: &[K], cache: &mut HashMap<(K, Vec<K>), Vec<Self>>) -> Vec<Self> {
        let cache_key: (K, Vec<_>) = (*start_key, code.into());
        if let Some(existing) = cache.get(&cache_key) {
            existing.clone()
        } else {
            let mut results = Vec::new();
            if code.len() == 1 {
                // these are finished results, they will always be the shortest (due to astar_bag)
                for mut result in Self::shortest_paths_to_key(start_key, &code[0]) {
                    result.movements.push(DirectionalKey::Activate);
                    results.push(result);
                }
            } else {
                // continue recursively by splitting in half
                let split = code.len() / 2;
                for first_half_result in Self::shortest_paths_to_code_recursive(start_key, &code[0..split], cache) {
                    for mut second_half_result in Self::shortest_paths_to_code_recursive(&first_half_result.current, &code[split..code.len()], cache) {
                        let mut new_movements = first_half_result.movements.clone();
                        new_movements.append(&mut second_half_result.movements);
                        second_half_result.movements = new_movements;
                        results.push(second_half_result);
                    }
                }
            }
            // save in cache
            cache.insert(cache_key, results.clone());
            results
        }
    }

    fn shortest_paths_to_key(start_key: &K, key: &K) -> Vec<Self> {
        let start = Self {
            current: *start_key,
            movements: Vec::new()
        };
        let (results, _) = astar_bag(&start, |kp| kp.successors(), |kp| kp.current.minimum_distance_to(key), |kp| kp.current == *key).expect("No solution");
        results.into_iter().map(|r| r.into_iter().last().unwrap()).collect()
    }

    fn successors(&self) -> Vec<(Self, usize)> {
        let mut v = Vec::new();
        for direction in self.valid_moves() {
            let mut clone = self.clone();
            clone.move_current(&direction);
            v.push((clone, 1));
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
        self.movements.push(DirectionalKey::Move(*direction))
    }
}