use crate::numeric::NumericKey;
use crate::directional::{Direction, DirectionalKey};
use std::hash::Hash;
use std::marker::PhantomData;

pub trait Key : Sized + Default + Clone + Copy + Hash + Eq + PartialEq {
    fn from_char(c: char) -> Self;
    fn to_char(&self) -> char;
    fn key_above(&self) -> Option<Self>;
    fn key_below(&self) -> Option<Self>;
    fn key_left(&self) -> Option<Self>;
    fn key_right(&self) -> Option<Self>;
    fn row(&self) -> usize;
    fn col(&self) -> usize;
    
    // fn minimum_distance_to(&self, other: &Self) -> usize {
    //     self.row().abs_diff(other.row()) + self.col().abs_diff(other.col())
    // }
}

pub trait Keypad<K: Key> : Clone + Hash + Eq + PartialEq {
    fn try_next_state(&self, press: &DirectionalKey) -> Option<Self>;
    fn final_key(&self) -> NumericKey;
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct RobotKeypad<KP: Keypad<K>, K: Key> {
    _phantom: PhantomData<K>,
    pub current: DirectionalKey,
    inner_keypad: KP
}

impl<KP: Keypad<K>, K: Key> Keypad<K> for RobotKeypad<KP, K> {
    fn try_next_state(&self, press: &DirectionalKey) -> Option<Self> {
        match press {
            DirectionalKey::Move(direction) => {
                let next_current = match direction {
                    Direction::Up => self.current.key_above(),
                    Direction::Down => self.current.key_below(),
                    Direction::Left => self.current.key_left(),
                    Direction::Right => self.current.key_right(),
                }?;
                let mut clone = self.clone();
                clone.current = next_current;
                Some(clone)
            },
            DirectionalKey::Activate => {
                let next_inner = self.inner_keypad.try_next_state(&self.current)?;
                let mut clone = self.clone();
                clone.inner_keypad = next_inner;
                Some(clone)
            }
        }
    }

    fn final_key(&self) -> NumericKey {
        self.inner_keypad.final_key()
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

    pub fn successors(&self) -> Vec<Self> {
        let mut v = Vec::new();
        for dk in &DirectionalKey::ALL {
            if let Some(next) = self.try_next_state(dk) {
                v.push(next);
            }
        }
        v
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct FinalKeypad {
    pub current: NumericKey
}

impl<K: Key> Keypad<K> for FinalKeypad {
    fn try_next_state(&self, press: &DirectionalKey) -> Option<Self> {
        match press {
            DirectionalKey::Move(direction) => {
                let next_current = match direction {
                    Direction::Up => self.current.key_above(),
                    Direction::Down => self.current.key_below(),
                    Direction::Left => self.current.key_left(),
                    Direction::Right => self.current.key_right(),
                }?;
                let mut clone = self.clone();
                clone.current = next_current;
                Some(clone)
            },
            DirectionalKey::Activate => {
                None
            }
        }
    }

    fn final_key(&self) -> NumericKey {
        self.current.clone()
    }
}

impl FinalKeypad {
    pub fn new() -> Self {
        Self {
            current: NumericKey::default()
        }
    }
}