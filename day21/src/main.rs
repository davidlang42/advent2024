use std::fs;
use std::env;
use std::str::FromStr;
use pathfinding::prelude::bfs;
use std::fmt::Display;
use std::fmt::Formatter;

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
struct NumericKeypad {
    current: NumericKey,
    presses: Vec<NumericKey>
}

impl NumericKeypad {
    fn new() -> Self {
        Self{
            current: NumericKey::Activate,
            presses: Vec::new()
        }
    }

    fn move_up(&self) -> Option<Self> {
        Some(Self {
            current: self.current.move_up()?,
            presses: self.presses.clone()
        })
    }

    fn move_down(&self) -> Option<Self> {
        Some(Self {
            current: self.current.move_down()?,
            presses: self.presses.clone()
        })
    }

    fn move_left(&self) -> Option<Self> {
        Some(Self {
            current: self.current.move_left()?,
            presses: self.presses.clone()
        })
    }

    fn move_right(&self) -> Option<Self> {
        Some(Self {
            current: self.current.move_right()?,
            presses: self.presses.clone()
        })
    }
    
    fn press_current(&self) -> Self {
        let mut next = self.clone();
        next.presses.push(self.current);
        next
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Copy)]
enum NumericKey {
    Activate,
    Digit(u8)
}

impl NumericKey {
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

struct Code {
    keys: Vec<NumericKey>
}

impl FromStr for Code {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let keys = line.chars().map(|c| NumericKey::from_char(c)).collect();
        Ok(Self {
            keys
        })
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for k in &self.keys {
            write!(f, "{}", k.to_char())?
        }
        Ok(())
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct FirstRobot {
    keypad: NumericKeypad
}

impl FirstRobot {
    fn new() -> Self {
        Self {
            keypad: NumericKeypad::new()
        }
    }
}

impl FirstRobot {
    fn next_options(&self) -> Vec<FirstRobot> {
        let mut v = vec![Self {
            keypad: self.keypad.press_current()
        }];
        if let Some(keypad) = self.keypad.move_up() {
            v.push(Self { keypad });
        }
        if let Some(keypad) = self.keypad.move_down() {
            v.push(Self { keypad });
        }
        if let Some(keypad) = self.keypad.move_left() {
            v.push(Self { keypad });
        }
        if let Some(keypad) = self.keypad.move_right() {
            v.push(Self { keypad });
        }
        v
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let codes: Vec<Code> = text.lines().map(|s| s.parse().unwrap()).collect();
        let start = FirstRobot::new();
        for code in codes {
            println!("Code: {}", code);
            let result = bfs(&start, |r| r.next_options(), |r| r.keypad.presses == code.keys).expect("No solution");
            println!("Shortest path: {}", result.len() - 1);
            panic!();
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}