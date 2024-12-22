use crate::Code;
use crate::directional::{Direction, DirectionalKey};
use std::hash::Hash;
use pathfinding::prelude::bfs;

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

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Keypad<K: Key> {
    pub current: K,
    pub movements: Vec<DirectionalKey>//TODO exclude this from Eq/PartialEq
}

impl<K: Key> Keypad<K> {
    pub fn new() -> Self {
        Self{
            current: K::default(),
            movements: Vec::new()
        }
    }

    pub fn presses_string(&self) -> String {
        let mut s = String::new();
        for key in &self.movements {
            s.push(key.to_char());
        }
        s
    }

    pub fn shortest_path_to_code(&self, code: &Code<K>) -> Self {
        let mut result = self.clone();
        for key in &code.keys {
            result = result.shortest_path_to_key(key);
            result.movements.push(DirectionalKey::Activate);
        }
        result
    }

    fn shortest_path_to_key(&self, key: &K) -> Self {
        let result = bfs(self, |kp| kp.successors(), |kp| kp.current == *key).expect("No solution");
        result.into_iter().last().unwrap()
    }

    fn successors(&self) -> Vec<Self> {
        let mut v = Vec::new();
        for direction in self.valid_moves() {
            let mut clone = self.clone();
            clone.move_current(&direction);
            v.push(clone);
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