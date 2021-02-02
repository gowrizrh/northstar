use super::world::Location;

pub struct RefMap<'a> {
    cells: Vec<Vec<Location<'a>>>
}

impl<'a> RefMap<'a> {

    pub fn new() -> RefMap<'a> {

        let map_string: &str =
"0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0
0 0 0 1 1 1 1 1 0 0
0 0 0 1 0 0 0 1 0 0
0 0 0 1 0 0 0 1 0 0
0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0";

        let lines: Vec<&str> = map_string.split("\n").collect();
        let mut map: Vec<Vec<Location>> = Vec::with_capacity(lines.len());

        for x in 0..lines.len() {
            let line: Vec<&str> = lines[x].split_whitespace().collect();

            let mut rows: Vec<Location> = Vec::with_capacity(line.len());

            for y in 0..line.len() {

                rows.push(
                    Location::new(
                        x as u16, // TODO: Restrict lines.len() and line.len() to u16
                        y as u16,
                        line[y].parse::<u8>().unwrap()
                    )
                );
            }
            
            map.push(rows);
        }
        
        RefMap {
            cells: map
        }
    }

    /// A helper utility to visualize the map, also used for debugging purposes
    pub fn print_map(&self) {
        for row in &self.cells {
            for cell in row {
                print!("{0} ", cell.value);
            }
            println!();
        }
    }

    /// Returns an iterable collection of neighbours.
    /// 
    /// for example, if x is your given co-ordinates, then it's surrounding
    /// locations are its neighbours
    /// 
    /// 0 0 0 0 0 0 0 0
    /// 0 0 0 0 0 0 0 0
    /// 0 0 0 y 0 0 0 0
    /// 0 0 y x y 0 0 0
    /// 0 0 0 y 0 0 0 0
    /// 0 0 0 0 0 0 0 0
    /// 0 0 0 0 0 0 0 0
    /// 
    /// which is,
    /// (4,4) -> (4,2) (4,3) (2,4) (3,4)
    fn neighbours(&self, row: usize, col: usize ) -> Vec<&Location> {
        let mut neighbours: Vec<&Location> = Vec::with_capacity(4) ;

        if col > 0 {
            neighbours.push(&self.cells[row][col - 1]);
        }

        if col < self.cells[row].len() - 1 {
            neighbours.push(&self.cells[row][col + 1]);
        }

        if row > 0 {
            neighbours.push(&self.cells[row - 1][col]);
        }

        if row < self.cells.len() - 1 {
            neighbours.push(&self.cells[row + 1][col]);
        }

        neighbours
    }
}