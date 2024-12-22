use crate::keypad::Key;
use crate::Code;
use crate::directional::{Direction, DirectionalKey};
use pathfinding::prelude::bfs;

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct NumericKeypad {
    pub current: NumericKey,
    pub movements: Vec<DirectionalKey>//TODO exclude this from Eq/PartialEq
}

impl NumericKeypad {
    pub fn new() -> Self {
        Self{
            current: NumericKey::Activate,
            movements: Vec::new()
        }
    }

    pub fn valid_moves(&self) -> Vec<Direction> {
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

    pub fn presses_string(&self) -> String {
        let mut s = String::new();
        for key in &self.movements {
            s.push(key.to_char());
        }
        s
    }

    pub fn move_current(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.current = self.current.key_above().unwrap(),
            Direction::Down => self.current = self.current.key_below().unwrap(),
            Direction::Left => self.current = self.current.key_left().unwrap(),
            Direction::Right => self.current = self.current.key_right().unwrap(),
        }
        self.movements.push(DirectionalKey::Move(*direction))
    }

    pub fn shortest_path_to_code(&self, code: &Code<NumericKey>) -> Self {
        let mut result = self.clone();
        for key in &code.keys {
            result = result.shortest_path_to_key(key);
            result.movements.push(DirectionalKey::Activate);
        }
        result
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

    fn shortest_path_to_key(&self, key: &NumericKey) -> Self {
        let result = bfs(self, |kp| kp.successors(), |kp| kp.current == *key).expect("No solution");
        result.into_iter().last().unwrap()
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Copy, Debug)]
pub enum NumericKey {
    Activate,
    Digit(u8)
}

impl Key for NumericKey {
    fn from_char(c: char) -> Self {
        if c == 'A' {
            Self::Activate
        } else {
            let d = c as u8 - '0' as u8;
            if d > 9 {
                panic!("Invalid numeric key: {}", c);
            }
            Self::Digit(d)
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Activate => 'A',
            Self::Digit(d) => (d + '0' as u8) as char
        }
    }

    fn key_above(&self) -> Option<Self> {
        match self {
            Self::Activate => Some(Self::Digit(3)),
            Self::Digit(0) => Some(Self::Digit(2)),
            Self::Digit(top) if *top >= 7 => None,
            Self::Digit(d) => Some(Self::Digit(d + 3))
        }
    }

    fn key_below(&self) -> Option<Self> {
        match self {
            Self::Activate => None,
            Self::Digit(0) => None,
            Self::Digit(1) => None,
            Self::Digit(2) => Some(Self::Digit(0)),
            Self::Digit(3) => Some(Self::Activate),
            Self::Digit(d) => Some(Self::Digit(d - 3))
        }
    }

    fn key_left(&self) -> Option<Self> {
        match self {
            Self::Activate => Some(Self::Digit(0)),
            Self::Digit(0) => None,
            Self::Digit(left) if left % 3 == 1 => None,
            Self::Digit(d) => Some(Self::Digit(d - 1))
        }
    }

    fn key_right(&self) -> Option<Self> {
        match self {
            Self::Activate => None,
            Self::Digit(0) => Some(Self::Activate),
            Self::Digit(right) if right % 3 == 0 => None,
            Self::Digit(d) => Some(Self::Digit(d + 1))
        }
    }

    fn row(&self) -> usize {
        match self {
            Self::Digit(0) => 3,
            Self::Activate => 3,
            Self::Digit(d) if *d <= 3 => 2,
            Self::Digit(d) if *d <= 6 => 1,
            _ => 0
        }
    }

    fn col(&self) -> usize {
        match self {
            Self::Digit(0) => 1,
            Self::Activate => 2,
            Self::Digit(d) if d % 3 == 0 => 2,
            Self::Digit(d) if d % 3 == 1 => 0,
            _ => 1
        }
    }
}
