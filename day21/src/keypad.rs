pub trait Key : Sized {
    fn from_char(c: char) -> Self;
    fn to_char(&self) -> char;
    fn key_above(&self) -> Option<Self>;
    fn key_below(&self) -> Option<Self>;
    fn key_left(&self) -> Option<Self>;
    fn key_right(&self) -> Option<Self>;
}