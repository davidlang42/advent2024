use crate::NumericKey;
use crate::directional::DirectionalKey;

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

pub trait Keypad : Clone {
    fn valid_operations(&self) -> Vec<DirectionalKey>;
    fn operate(&mut self, operation: &DirectionalKey);
    fn underlying_code(&self) -> &Vec<NumericKey>;

    fn code_is_possible(&self, goal_code: &Vec<NumericKey>) -> bool {
        let code_so_far = self.underlying_code();
        if code_so_far.len() > goal_code.len() {
            return false;
        }
        for i in 0..code_so_far.len() {
            if code_so_far[i] != goal_code[i] {
                return false;
            }
        }
        true
    }
}