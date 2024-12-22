use crate::keypad::Key;
use crate::{NumericKey, NumericKeypad};
use crate::Code;

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct DirectionalKeypad {
    current: DirectionalKey,
    pub presses: Vec<DirectionalKey>
}

impl DirectionalKeypad {
    pub fn new() -> Self {
        Self{
            current: DirectionalKey::Activate,
            presses: Vec::new()
        }
    }

    pub fn presses_string(&self) -> String {
        let mut s = String::new();
        for key in &self.presses {
            s.push(key.to_char());
        }
        s
    }

    // fn valid_directions(&self) -> Vec<DirectionalKey> {
    //     let mut v = Vec::new();
    //     if !self.current.key_above().is_none() {
    //         v.push(DirectionalKey::Up)
    //     }
    //     if !self.current.key_below().is_none() {
    //         v.push(DirectionalKey::Down)
    //     }
    //     if !self.current.key_left().is_none() {
    //         v.push(DirectionalKey::Left)
    //     }
    //     if !self.current.key_right().is_none() {
    //         v.push(DirectionalKey::Right)
    //     }
    //     v
    // }

    // fn operate(&mut self, operation: &DirectionalKey) {
    //     match operation {
    //         DirectionalKey::Activate => self.presses.push(self.current),
    //         DirectionalKey::Up => self.current = self.current.key_above().unwrap(),
    //         DirectionalKey::Down => self.current = self.current.key_below().unwrap(),
    //         DirectionalKey::Left => self.current = self.current.key_left().unwrap(),
    //         DirectionalKey::Right => self.current = self.current.key_right().unwrap(),
    //     }
    // }
}

#[derive(Clone, Hash, Eq, PartialEq, Copy, Debug)]
pub enum DirectionalKey {
    Activate,
    Move(Direction)
}

#[derive(Clone, Hash, Eq, PartialEq, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Key for DirectionalKey {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::Activate,
            '^' => Self::Move(Direction::Up),
            '<' => Self::Move(Direction::Left),
            '>' => Self::Move(Direction::Right),
            'v' => Self::Move(Direction::Down),
            _ => panic!("Invalid directional key: {}", c)
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Activate => 'A',
            Self::Move(Direction::Up) => '^',
            Self::Move(Direction::Left) => '<',
            Self::Move(Direction::Right) => '>',
            Self::Move(Direction::Down) => 'v'
        }
    }

    fn key_above(&self) -> Option<Self> {
        match self {
            Self::Activate => None,
            Self::Move(Direction::Up) => None,
            Self::Move(Direction::Left) => None,
            Self::Move(Direction::Right) => Some(Self::Activate),
            Self::Move(Direction::Down) => Some(Self::Move(Direction::Up))
        }
    }

    fn key_below(&self) -> Option<Self> {
        match self {
            Self::Activate => Some(Self::Move(Direction::Right)),
            Self::Move(Direction::Up) => Some(Self::Move(Direction::Down)),
            Self::Move(Direction::Left) => None,
            Self::Move(Direction::Right) => None,
            Self::Move(Direction::Down) => None
        }
    }

    fn key_left(&self) -> Option<Self> {
        match self {
            Self::Activate => Some(Self::Move(Direction::Up)),
            Self::Move(Direction::Up) => None,
            Self::Move(Direction::Left) => None,
            Self::Move(Direction::Right) => Some(Self::Move(Direction::Down)),
            Self::Move(Direction::Down) => Some(Self::Move(Direction::Left))
        }
    }

    fn key_right(&self) -> Option<Self> {
        match self {
            Self::Activate => None,
            Self::Move(Direction::Up) => Some(Self::Activate),
            Self::Move(Direction::Left) => Some(Self::Move(Direction::Down)),
            Self::Move(Direction::Right) => None,
            Self::Move(Direction::Down) => Some(Self::Move(Direction::Right))
        }
    }

    fn row(&self) -> usize {
        match self {
            Self::Activate => 0,
            Self::Move(Direction::Up) => 0,
            _ => 1
        }
    }

    fn col(&self) -> usize {
        match self {
            Self::Move(Direction::Left) => 0,
            Self::Move(Direction::Up) => 1,
            Self::Move(Direction::Down) => 1,
            _ => 2
        }
    }
}