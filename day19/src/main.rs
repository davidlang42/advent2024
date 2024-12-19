use std::fs;
use std::env;
use std::str::FromStr;

enum Color {
    White,
    Blue,
    Black,
    Red,
    Green
}

impl Color {
    fn from_char(ch: char) -> Self {
        match ch {
            'w' => Self::White,
            'u' => Self::Blue,
            'b' => Self::Black,
            'r' => Self::Red,
            'g' => Self::Green,
            _ => panic!("Invalid color {}", ch)
        }
    }
}

struct Towel {
    stripes: Vec<Color>
}

impl FromStr for Towel {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let stripes: Vec<_> = line.chars().map(|ch| Color::from_char(ch)).collect();
        Ok(Self {
            stripes
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let sections: Vec<_> = text.split("\r\n\r\n").collect();
        if sections.len() != 2 {
            panic!("Invalid number of sections")
        }
        let available: Vec<Towel> = sections[0].split(", ").map(|s| s.parse().unwrap()).collect();
        let designs: Vec<Towel> = sections[1].lines().map(|s| s.parse().unwrap()).collect();
        
        println!("Answer: {}, {}", available.len(), designs.len());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}