use crate::keypad::{Key, Keypad};
use crate::NumericKey;

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
#[derive(Clone, Hash, Eq, PartialEq)]
pub struct DirectionalKeypad<K: Keypad> {
    current: DirectionalKey,
    pub presses: Vec<DirectionalKey>,
    controlling_keypad: K
}

impl<K: Keypad> DirectionalKeypad<K> {
    pub fn new(controlling_keypad: K) -> Self {
        Self{
            current: DirectionalKey::Activate,
            presses: Vec::new(),
            controlling_keypad
        }
    }
    
    pub fn available_options(&self) -> Vec<Self> {
        let mut v = Vec::new();
        for op in self.controlling_keypad.valid_operations() {
            let mut clone = self.clone();
            clone.controlling_keypad.operate(&op);
            clone.presses.push(op);
            v.push(clone);
        }
        v
    }

    pub fn press_string(&self) -> String {
        self.presses.iter().map(|p| p.to_char()).collect()
    }
}

impl<K: Keypad> Keypad for DirectionalKeypad<K> {
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
        self.controlling_keypad.underlying_code()
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Copy)]
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
}
