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

    pub fn move_up(&self) -> Option<Self> {
        Some(Self {
            current: self.current.move_up()?,
            presses: self.presses.clone()
        })
    }

    pub fn move_down(&self) -> Option<Self> {
        Some(Self {
            current: self.current.move_down()?,
            presses: self.presses.clone()
        })
    }

    pub fn move_left(&self) -> Option<Self> {
        Some(Self {
            current: self.current.move_left()?,
            presses: self.presses.clone()
        })
    }

    pub fn move_right(&self) -> Option<Self> {
        Some(Self {
            current: self.current.move_right()?,
            presses: self.presses.clone()
        })
    }
    
    pub fn press_current(&self) -> Self {
        let mut next = self.clone();
        next.presses.push(self.current);
        next
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Copy)]
pub enum NumericKey {
    Activate,
    Digit(u8)
}

impl NumericKey {
    pub fn from_char(c: char) -> Self {
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

    pub fn to_char(&self) -> char {
        match self {
            Self::Activate => 'A',
            Self::Digit(d) => (d + '0' as u8) as char
        }
    }

    fn move_up(&self) -> Option<Self> {
        match self {
            Self::Activate => Some(Self::Digit(3)),
            Self::Digit(0) => Some(Self::Digit(2)),
            Self::Digit(top) if *top >= 7 => None,
            Self::Digit(d) => Some(Self::Digit(d + 3))
        }
    }

    fn move_down(&self) -> Option<Self> {
        match self {
            Self::Activate => None,
            Self::Digit(0) => None,
            Self::Digit(1) => None,
            Self::Digit(2) => Some(Self::Digit(0)),
            Self::Digit(3) => Some(Self::Activate),
            Self::Digit(d) => Some(Self::Digit(d - 3))
        }
    }

    fn move_left(&self) -> Option<Self> {
        match self {
            Self::Activate => Some(Self::Digit(0)),
            Self::Digit(0) => None,
            Self::Digit(left) if left % 3 == 1 => None,
            Self::Digit(d) => Some(Self::Digit(d - 1))
        }
    }

    fn move_right(&self) -> Option<Self> {
        match self {
            Self::Activate => None,
            Self::Digit(0) => Some(Self::Activate),
            Self::Digit(right) if right % 3 == 0 => None,
            Self::Digit(d) => Some(Self::Digit(d + 1))
        }
    }
}
