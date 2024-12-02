use std::fs;
use std::env;
use std::str::FromStr;

struct TwoLists {
    a: Vec<usize>,
    b: Vec<usize>
}

impl FromStr for TwoLists {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut a = Vec::new();
        let mut b = Vec::new();
        for line in text.lines() {
            let numbers: Vec<&str> = line.split("   ").collect();
            if numbers.len() != 2 {
                panic!("Line not 2 numbers: {}", line);
            }
            a.push(numbers[0].parse().unwrap());
            b.push(numbers[1].parse().unwrap());
        }
        Ok(Self {
            a,b
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut lists: TwoLists = text.parse().unwrap();
        lists.a.sort();
        lists.b.sort();
        let mut error = 0;
        let mut similarity = 0;
        for i in 0..lists.a.len() {
            error += lists.a[i].abs_diff(lists.b[i]);
            similarity += lists.a[i] * lists.b.iter().filter(|n| **n == lists.a[i]).count();
        }
        println!("Error: {}", error);
        println!("Similarity: {}", similarity);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}