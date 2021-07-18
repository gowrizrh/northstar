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

    /// This function returns an iterable containing the valid neighbours
    /// for the given location.
    pub fn neighbours(&self, location: &Location) -> Vec<&Location> {
        let mut neighbours: Vec<&Location> = Vec::new();

        // Bound checking
        if location.x >= self.x() || location.y >= self.y(location.x) {
            println!("{0}, {1}", location.x >= self.x(), location.y >= self.y(location.x));
            return neighbours;
        }

        if location.x > 0 {
            neighbours.push(&self.cells[location.x - 1][location.y])
        }

        if location.x < self.x() - 1 {
            neighbours.push(&self.cells[location.x + 1][location.y]);
        }

        if location.y > 0 {
            neighbours.push(&self.cells[location.x][location.y - 1])
        }

        if location.y < self.y(location.x) - 1 {
            neighbours.push(&self.cells[location.x][location.y + 1]);
        }

        neighbours
    }

    /// This function returns the number of rows that the map has
    pub fn x(&self) -> usize {
        return self.cells.len();
    }

    /// This function returns the number of cells in the current row
    pub fn y(&self, current_row: usize) -> usize {
        return self.cells[current_row].len();
    }
}
