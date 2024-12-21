
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
pub enum DirectionalKey {
    Activate,
    Up,
    Left,
    Right,
    Down
}

impl DirectionalKey {
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

    fn move_up(&self) -> Option<Self> {
        match self {
            Self::Activate => None,
            Self::Up => None,
            Self::Left => None,
            Self::Right => Some(Self::Activate),
            Self::Down => Some(Self::Up)
        }
    }

    fn move_down(&self) -> Option<Self> {
        match self {
            Self::Activate => Some(Self::Right),
            Self::Up => Some(Self::Down),
            Self::Left => None,
            Self::Right => None,
            Self::Down => None
        }
    }

    fn move_left(&self) -> Option<Self> {
        match self {
            Self::Activate => Some(Self::Up),
            Self::Up => None,
            Self::Left => None,
            Self::Right => Some(Self::Down),
            Self::Down => Some(Self::Left)
        }
    }

    fn move_right(&self) -> Option<Self> {
        match self {
            Self::Activate => None,
            Self::Up => Some(Self::Activate),
            Self::Left => Some(Self::Down),
            Self::Right => None,
            Self::Down => Some(Self::Right)
        }
    }
}
