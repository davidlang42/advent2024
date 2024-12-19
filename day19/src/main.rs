use std::fs;
use std::env;

// enum Color {
//     White,
//     Blue,
//     Black,
//     Red,
//     Green
// }

// impl Color {
//     fn from_char(ch: char) -> Self {
//         match ch {
//             'w' => Self::White,
//             'u' => Self::Blue,
//             'b' => Self::Black,
//             'r' => Self::Red,
//             'g' => Self::Green,
//             _ => panic!("Invalid color {}", ch)
//         }
//     }
// }

// struct Towel {
//     stripes: Vec<Color>
// }

// impl FromStr for Towel {
//     type Err = String;

//     fn from_str(line: &str) -> Result<Self, Self::Err> {
//         let stripes: Vec<_> = line.chars().map(|ch| Color::from_char(ch)).collect();
//         Ok(Self {
//             stripes
//         })
//     }
// }

// impl Towel {
//     fn can_be_made_from(&self, subs: &Vec<Towel>) -> bool {
//         todo!()
//     }
// }

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
        let available: Vec<&str> = sections[0].split(", ").collect();
        let designs: Vec<&str> = sections[1].lines().collect();
        let mut possible = 0;
        let mut combos = 0;
        for d in &designs {
            let number = number_of_ways_to_make(&d, &available);
            if number > 0 {
                possible += 1;
            }
            combos += number;
        }
        println!("Possible: {} out of {}", possible, designs.len());
        println!("Combos: {}", combos);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn number_of_ways_to_make(target: &str, available: &Vec<&str>) -> usize {
    let mut count = 0;
    for a in available {
        if target.starts_with(a) {
            let remaining = &target[a.len()..target.len()];
            if remaining.len() == 0 {
                count += 1;
            } else {
                count += number_of_ways_to_make(remaining, available);
            }
        }
    }
    count
}
