use std::str::FromStr;
use std::fmt::Display;
use std::fmt::Formatter;
use crate::keypad::Key;
use crate::numeric::NumericKey;

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

impl Code<NumericKey> {
    pub fn numeric_part(&self) -> usize {
        let mut n = 0;
        for k in &self.keys {
            if let NumericKey::Digit(d) = k {
                n *= 10;
                n += *d as usize;
            }
        }
        n
    }
}