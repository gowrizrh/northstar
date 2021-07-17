use std::fmt::Debug;

pub enum GridCell {
    Path,
    Wall,
}

pub struct Location {
    pub x: usize,
    pub y: usize,
    pub val: GridCell,
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.val)
    }
}

impl Debug for GridCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridCell::Path => write!(f, "0"),
            _ => write!(f, "1"),
        }
    }
}