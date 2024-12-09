use std::fs;
use std::env;
use std::str::FromStr;
use std::fmt::Display;
use std::fmt::Formatter;

struct DiskMap {
    used: Vec<File>,
    free: Vec<usize> // free.len==used.len()-1
}

struct Disk {
    blocks: Vec<Option<Block>>
}

struct DiskByFile {
    files: Vec<FileOrSpace>,
    max_id: usize
}

enum FileOrSpace {
    File(File),
    Space(usize)
}

#[derive(Copy, Clone)]
struct File {
    id: usize,
    size: usize
}

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
        let mut id = 0;
        loop {
            used.push(File {
                id,
                size: *digits_iter.next().unwrap()
            });
            id += 1;
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
            disk.add_file(self.used[i]);
            disk.add_space(self.free[i]);
        }
        disk.add_file(self.used[self.used.len() - 1]);
        disk
    }

    fn to_disk_by_file(&self) -> DiskByFile {
        let mut disk = DiskByFile {
            files: Vec::new(),
            max_id: 0
        };
        for i in 0..self.used.len() - 1 {
            disk.add_file(self.used[i]);
            disk.add_space(self.free[i]);
        }
        disk.add_file(self.used[self.used.len() - 1]);
        disk
    }
}

impl Disk {
    fn add_file(&mut self, file: File) {
        for _i in 0..file.size {
            self.blocks.push(Some(Block { id: file.id }));
        }
    }

    fn add_space(&mut self, size: usize) {
        for _i in 0..size {
            self.blocks.push(None);
        }
    }

    fn compact(&mut self) {
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

impl DiskByFile {
    fn add_file(&mut self, file: File) {
        self.files.push(FileOrSpace::File(file));
        if file.id > self.max_id {
            self.max_id = file.id;
        }
    }

    fn add_space(&mut self, size: usize) {
        self.files.push(FileOrSpace::Space(size));
    }

    fn compact(&mut self) {
        let mut move_from = self.files.len() - 1;
        let mut move_to = 0;
        for id in (0..(self.max_id + 1)).rev() {
            let mut required_size = 0;
            let move_from = self.files.iter().position(|f| {
                if let FileOrSpace::File(file) = f {
                    required_size = file.size;
                    file.id == id
                } else {
                    false
                }
            }).unwrap();
            for move_to in (0..move_from).rev() {
                if let FileOrSpace::Space(size) = self.files[move_to] && size >= required_size {
                    
                }
            }
        }

        while move_from > move_to {
            if self.files[move_from].is_none() {
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

    fn to_disk(&self) -> Disk {
        let mut disk = Disk {
            blocks: Vec::new()
        };
        for fs in &self.files {
            match fs {
                FileOrSpace::File(f) => disk.add_file(*f),
                FileOrSpace::Space(s) => disk.add_space(*s)
            }
        }
        disk
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
        disk.compact();
        println!("Compact: {}", disk);
        println!("Checksum: {}", disk.checksum());
        let mut disk_by_file = map.to_disk_by_file();
        disk_by_file.compact();
        println!("Compact by file: {}", disk_by_file.to_disk());
        println!("Checksum by file: {}", disk_by_file.to_disk().checksum());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}