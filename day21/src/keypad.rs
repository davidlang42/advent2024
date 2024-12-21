use crate::NumericKey;
use crate::directional::DirectionalKey;

pub trait Key : Sized {
    fn from_char(c: char) -> Self;
    fn to_char(&self) -> char;
    fn key_above(&self) -> Option<Self>;
    fn key_below(&self) -> Option<Self>;
    fn key_left(&self) -> Option<Self>;
    fn key_right(&self) -> Option<Self>;
}

pub trait Keypad : Clone {
    fn valid_operations(&self) -> Vec<DirectionalKey>;
    fn operate(&mut self, operation: &DirectionalKey);
    fn underlying_code(&self) -> &Vec<NumericKey>;
}