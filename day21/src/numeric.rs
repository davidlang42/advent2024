use crate::keypad::{Key, Keypad};
use crate::directional::DirectionalKey;

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
#[derive(Clone, Hash, Eq, PartialEq)]
pub struct NumericKeypad {
    current: NumericKey,
    pub presses: Vec<NumericKey>
}

impl NumericKeypad {
    pub fn new() -> Self {
        Self{
            current: NumericKey::Activate,
            presses: Vec::new()
        }
    }
}

impl Keypad for NumericKeypad {
    fn valid_operations(&self) -> Vec<DirectionalKey> {
        let mut v = vec![DirectionalKey::Activate];
        if !self.current.key_above().is_none() {
            v.push(DirectionalKey::Up)
        }
        if !self.current.key_below().is_none() {
            v.push(DirectionalKey::Down)
        }
        if !self.current.key_left().is_none() {
            v.push(DirectionalKey::Left)
        }
        if !self.current.key_right().is_none() {
            v.push(DirectionalKey::Right)
        }
        v
    }

    fn operate(&mut self, operation: &DirectionalKey) {
        match operation {
            DirectionalKey::Activate => self.presses.push(self.current),
            DirectionalKey::Up => self.current = self.current.key_above().unwrap(),
            DirectionalKey::Down => self.current = self.current.key_below().unwrap(),
            DirectionalKey::Left => self.current = self.current.key_left().unwrap(),
            DirectionalKey::Right => self.current = self.current.key_right().unwrap(),
        }
    }

    fn underlying_code(&self) -> &Vec<NumericKey> {
        &self.presses
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Copy)]
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
