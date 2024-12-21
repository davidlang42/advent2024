use crate::keypad::Key;

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

    pub fn can_move_up(&self) -> bool {
        !self.current.key_above().is_none()
    }

    pub fn move_up(&mut self) {
        self.current = self.current.key_above().unwrap();
    }

    pub fn can_move_down(&self) -> bool {
        !self.current.key_below().is_none()
    }

    pub fn move_down(&mut self) {
        self.current = self.current.key_below().unwrap();
    }

    pub fn can_move_left(&self) -> bool {
        !self.current.key_left().is_none()
    }

    pub fn move_left(&mut self) {
        self.current = self.current.key_left().unwrap();
    }

    pub fn can_move_right(&self) -> bool {
        !self.current.key_right().is_none()
    }

    pub fn move_right(&mut self) {
        self.current = self.current.key_right().unwrap();
    }
    
    pub fn press_current(&mut self) {
        self.presses.push(self.current);
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
}
