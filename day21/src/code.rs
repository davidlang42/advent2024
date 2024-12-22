use std::str::FromStr;
use std::fmt::Display;
use std::fmt::Formatter;
use crate::keypad::Key;

pub struct Code<K: Key> {
    pub keys: Vec<K>
}

impl<K: Key> FromStr for Code<K> {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let keys = line.chars().map(|c| K::from_char(c)).collect();
        Ok(Self {
            keys
        })
    }
}

impl<K: Key> Display for Code<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for k in &self.keys {
            write!(f, "{}", k.to_char())?
        }
        Ok(())
    }
}