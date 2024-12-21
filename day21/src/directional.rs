use crate::keypad::Key;

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
#[derive(Clone, Hash, Eq, PartialEq)]
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
