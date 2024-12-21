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
enum NumericKeypad {
    Activate,
    Digit(u8)
}

impl NumericKeypad {
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
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
enum DirectionalKeypad {
    Activate,
    Up,
    Left,
    Right,
    Down
}

impl DirectionalKeypad {
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
}

struct Code {
    keys: Vec<NumericKeypad>
}

impl FromStr for Code {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let keys = line.chars().map(|c| NumericKeypad::from_char(c)).collect();
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let codes: Vec<Code> = text.lines().map(|s| s.parse().unwrap()).collect();
        for code in codes {
            println!("Code: {}", code);
        }
        
    } else {
        println!("Please provide 1 argument: Filename");
    }
}