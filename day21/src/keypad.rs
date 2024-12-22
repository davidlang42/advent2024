// use crate::NumericKey;
// use crate::directional::DirectionalKey;
// use std::hash::Hash;

pub trait Key : Sized {
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

// pub trait Keypad : Hash + Clone + Eq + PartialEq {
//     fn valid_operations(&self) -> Vec<DirectionalKey>;
//     fn operate(&mut self, operation: &DirectionalKey);
// }