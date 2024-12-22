use crate::keypad::Key;
use crate::{NumericKey, NumericKeypad};
use crate::Code;
use pathfinding::prelude::bfs;

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct DirectionalKeypad {
    current: DirectionalKey,
    pub presses: Vec<DirectionalKey>,
    controlling_keypad: NumericKeypad
}

impl DirectionalKeypad {
    pub fn new(controlling_keypad: NumericKeypad) -> Self {
        Self{
            current: DirectionalKey::Activate,
            presses: Vec::new(),
            controlling_keypad
        }
    }

    pub fn presses_string(&self) -> String {
        let mut s = String::new();
        for key in &self.presses {
            s.push(key.to_char());
        }
        s
    }
    
    pub fn available_options(&self) -> Vec<Self> {
        let mut v = Vec::new();
        for op in self.controlling_keypad.valid_directions() {
            let mut clone = self.clone();
            clone.controlling_keypad.operate(&op);
            clone.presses.push(op);
            v.push(clone)
        }
        v
    }

    pub fn shortest_path_to_code(&self, code: &Code) -> Self {
        let mut result = self.clone();
        for key in &code.keys {
            result = result.shortest_path_to_key(key);
            result.presses.push(DirectionalKey::Activate);
        }
        result
    }

    fn shortest_path_to_key(&self, key: &NumericKey) -> Self {
        let result = bfs(self, |dk| dk.available_options(), |dk| dk.controlling_keypad.current == *key).expect("No solution");
        result.into_iter().last().unwrap()
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
    Up,
    Left,
    Right,
    Down
}

impl Key for DirectionalKey {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::Activate,
            '^' => Self::Up,
            '<' => Self::Left,
            '>' => Self::Right,
            'v' => Self::Down,
            _ => panic!("Invalid directional key: {}", c)
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Activate => 'A',
            Self::Up => '^',
            Self::Left => '<',
            Self::Right => '>',
            Self::Down => 'v'
        }
    }

    fn key_above(&self) -> Option<Self> {
        match self {
            Self::Activate => None,
            Self::Up => None,
            Self::Left => None,
            Self::Right => Some(Self::Activate),
            Self::Down => Some(Self::Up)
        }
    }

    fn key_below(&self) -> Option<Self> {
        match self {
            Self::Activate => Some(Self::Right),
            Self::Up => Some(Self::Down),
            Self::Left => None,
            Self::Right => None,
            Self::Down => None
        }
    }

    fn key_left(&self) -> Option<Self> {
        match self {
            Self::Activate => Some(Self::Up),
            Self::Up => None,
            Self::Left => None,
            Self::Right => Some(Self::Down),
            Self::Down => Some(Self::Left)
        }
    }

    fn key_right(&self) -> Option<Self> {
        match self {
            Self::Activate => None,
            Self::Up => Some(Self::Activate),
            Self::Left => Some(Self::Down),
            Self::Right => None,
            Self::Down => Some(Self::Right)
        }
    }

    fn row(&self) -> usize {
        match self {
            Self::Activate => 0,
            Self::Up => 0,
            _ => 1
        }
    }

    fn col(&self) -> usize {
        match self {
            Self::Left => 0,
            Self::Up => 1,
            Self::Down => 1,
            _ => 2
        }
    }
}
