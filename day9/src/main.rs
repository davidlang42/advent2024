use std::fs;
use std::env;
use std::str::FromStr;
use std::fmt::Display;
use std::fmt::Formatter;

struct DiskMap {
    used: Vec<usize>,
    free: Vec<usize> // free.len==used.len()-1
}

struct Disk {
    blocks: Vec<Option<Block>>
}

#[derive(Eq, PartialEq)]
struct Block {
    id: usize
}

impl FromStr for DiskMap {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut used = Vec::new();
        let mut free = Vec::new();
        let digits: Vec<usize> = line.chars().map(|c| c.to_string().parse().unwrap()).collect();
        let mut digits_iter = digits.iter();
        loop {
            used.push(*digits_iter.next().unwrap());
            if let Some(next) = digits_iter.next() {
                free.push(*next);
            } else {
                break;
            }
        }
        if free.len() != used.len() - 1 {
            panic!("Oops")
        }
        Ok(Self {
            used,
            free
        })
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for b in &self.blocks {
            if let Some(block) = b {
                write!(f, "{}", block.id)?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

impl DiskMap {
    fn to_disk(&self) -> Disk {
        let mut disk = Disk {
            blocks: Vec::new()
        };
        for i in 0..self.used.len() - 1 {
            disk.add_file(self.used[i], i);
            disk.add_space(self.free[i]);
        }
        disk.add_file(self.used[self.used.len() - 1], self.used.len() - 1);
        disk
    }
}

impl Disk {
    fn add_file(&mut self, size: usize, id: usize) {
        for _i in 0..size {
            self.blocks.push(Some(Block { id }));
        }
    }

    fn add_space(&mut self, size: usize) {
        for _i in 0..size {
            self.blocks.push(None);
        }
    }

    fn compact_by_block(&mut self) {
        let mut move_from = self.blocks.len() - 1;
        let mut move_to = 0;
        while move_from > move_to {
            if self.blocks[move_from].is_none() {
                move_from -= 1;
                continue;
            }
            if self.blocks[move_to].is_some() {
                move_to += 1;
                continue;
            }
            self.blocks.swap(move_from, move_to);
            move_to += 1;
            move_from -= 1;
        }
    }

    fn max_block_id(&self) -> usize {
        if self.blocks.len() == 0 {
            panic!("Empty disk");
        }
        let mut max = 0;
        for b in &self.blocks {
            if let Some(block) = b {
                if block.id > max {
                    max = block.id;
                }
            }
        }
        max
    }

    fn find_file(&self, id: usize) -> (usize, usize) {
        let mut start = None;
        for (i, b) in self.blocks.iter().enumerate() {
            if let Some(s) = start {
                if *b != Some(Block { id }) {
                    return (s, i - s);
                }
            } else if *b == Some(Block { id }) {
                start = Some(i);
            }
        }
        if let Some(s) = start {
            return (s, self.blocks.len() - s);
        }
        panic!("Block id not found")
    }

    fn has_space(&self, start: usize, length: usize) -> bool {
        for i in start..(start+length) {
            if self.blocks[i].is_some() {
                return false;
            }
        }
        true
    }

    fn swap_blocks(&mut self, mut a: usize, mut b: usize, length: usize) {
        for _i in 0..length {
            self.blocks.swap(a, b);
            a += 1;
            b += 1;
        }
    }

    fn compact_by_file(&mut self) {
        let max = self.max_block_id();
        for block_id in (0..(max+1)).rev() {
            let (start, length) = self.find_file(block_id);
            for move_to in 0..start {
                if self.has_space(move_to, length) {
                    self.swap_blocks(move_to, start, length);
                    break;
                }
            }
        }
    }

    fn checksum(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.blocks.len() {
            if let Some(block) = &self.blocks[i] {
                sum += i * block.id;
            }
        }
        sum
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let map: DiskMap = text.parse().unwrap();
        let mut disk = map.to_disk();
        println!("Original: {}", disk);
        disk.compact_by_block();
        println!("Compact: {}", disk);
        println!("Checksum: {}", disk.checksum());
        disk = map.to_disk(); //fresh copy
        disk.compact_by_file();
        println!("Compact by file: {}", disk);
        println!("Checksum by file: {}", disk.checksum());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}