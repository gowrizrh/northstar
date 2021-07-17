use super::location::Location;

pub struct Map {
    pub cells: Vec<Vec<Location>>,
}

impl Map {
    pub fn new(cells: Vec<Vec<Location>>) -> Self {
        Map { cells }
    }

    pub fn print(&self) -> () {
        for row in &self.cells {
            println!("{:?}", row);
        }
    }
}
