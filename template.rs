use std::fs;
use std::env;
use std::str::FromStr;

struct Object {
    ...
}

impl FromStr for Object {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        ...
        Ok(Self {
            ...
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let objects: Vec<Object> = text.lines().map(|s| s.parse().unwrap()).collect();
        ...
        println!("Answer: {}", ...);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}