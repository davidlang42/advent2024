use regex::Regex;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let re = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)|(don't)(\(\))|(do)(\(\))").unwrap();
        let mut sum = 0;
        let mut enabled = true;
        let mut sum_with_enabled = 0;
        for (_, [a, b]) in re.captures_iter(&text).map(|c| c.extract()) {
            if b == "()" {
                if a == "do" {
                    println!("do");
                    enabled = true;
                } else {
                    println!("don't");
                    enabled = false;
                }
            } else {
                let n1: usize = a.parse().unwrap();
                let n2: usize = b.parse().unwrap();
                println!("{}*{}={}", n1, n2, n1 * n2);
                sum += n1 * n2;
                if enabled {
                    sum_with_enabled += n1 * n2;
                }
            }
        }
        println!("Sum: {}", sum);
        println!("Sum with enabled: {}", sum_with_enabled);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
