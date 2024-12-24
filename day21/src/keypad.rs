use crate::numeric::NumericKey;
use crate::Code;
use crate::directional::{Direction, DirectionalKey};
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
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

pub trait Keypad<K: Key> {
    fn press(&mut self, directional_key: &DirectionalKey);
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct RobotKeypad<KP: Keypad<K>, K: Key> {
    _phantom: PhantomData<K>,
    pub current: DirectionalKey,
    inner_keypad: KP
}

impl<KP: Keypad<K>, K: Key> Keypad<K> for RobotKeypad<KP, K> {
    fn press(&mut self, directional_key: &DirectionalKey) {
        match directional_key {
            DirectionalKey::Move(direction) => {
                match direction {
                    Direction::Up => self.current = self.current.key_above().unwrap(),
                    Direction::Down => self.current = self.current.key_below().unwrap(),
                    Direction::Left => self.current = self.current.key_left().unwrap(),
                    Direction::Right => self.current = self.current.key_right().unwrap(),
                }
            },
            DirectionalKey::Activate => self.inner_keypad.press(&self.current)
        }
    }
}

impl<KP: Keypad<K>, K: Key> RobotKeypad<KP, K> {
    pub fn controlling(inner_keypad: KP) -> Self {
        Self {
            current: DirectionalKey::default(),
            inner_keypad,
            _phantom: PhantomData
        }
    }

    pub fn shortest_path_to_code(&mut self, code: &Code<NumericKey>) -> usize {
        let mut shortest = 0;
        for nk in &code.keys {
            shortest += self.shortest_path_to_key(nk);
            // activate?
        }
        shortest
    }

    fn shortest_path_to_key(&mut self, key: &NumericKey) -> usize {

    }

    fn valid_directions(&self) -> Vec<Direction> {
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
}

pub struct FinalKeypad {
    pub current: NumericKey
}

impl<K: Key> Keypad<K> for FinalKeypad {
    fn press(&mut self, directional_key: &DirectionalKey) {
        match directional_key {
            DirectionalKey::Move(direction) => {
                match direction {
                    Direction::Up => self.current = self.current.key_above().unwrap(),
                    Direction::Down => self.current = self.current.key_below().unwrap(),
                    Direction::Left => self.current = self.current.key_left().unwrap(),
                    Direction::Right => self.current = self.current.key_right().unwrap(),
                }
            },
            DirectionalKey::Activate => { }
        }
    }
}

impl FinalKeypad {
    pub fn new() -> Self {
        Self {
            current: NumericKey::default()
        }
    }

    // fn successors(&self) -> Vec<(Self, usize)> {
    //     let mut v = Vec::new();
    //     for direction in self.valid_moves() {
    //         let mut clone = self.clone();
    //         clone.move_current(&direction);
    //         v.push((clone, 1));
    //     }
    //     v
    // }
}