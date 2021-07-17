use std::error::Error;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;

#[derive(Eq, PartialEq)]
enum GridCell {
    Path,
    Wall,
}

struct Location {
    x: usize,
    y: usize,
    val: GridCell,
}

mod debug {
    use std::fmt::Debug;
    use super::Location;
    use super::GridCell;

    impl Debug for Location {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // write!(f, "({0}, {1})", self.x, self.y)
            write!(f, "{:?}", self.val)
        }
    }
    
    impl Debug for GridCell {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    
            match self {
                GridCell::Path => write!(f, "0"),
                _ => write!(f, "1")
            }
            
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let f: File = File::open("./maps/terrain03.txt")?;
    let reader: BufReader<File> = BufReader::new(f);

    let mut map: Vec<Vec<Location>> = Vec::new();

    for line in reader.lines() {
        map.push(parse_line(line?, map.len()));
    }

    for row in map {
        println!("{:?}", row);
    }

    Ok(())
}

fn parse_line(line: String, current_row: usize) -> Vec<Location> {
    let mut parsed_line: Vec<Location> = Vec::new();

    for c in line.chars() {
        let block: GridCell = match c {
            '0' => GridCell::Path,
            '1' => GridCell::Wall,
            _ => continue,
        };

        let point: Location = Location {
            x: parsed_line.len(),
            y: current_row,
            val: block,
        };

        parsed_line.push(point);
    }

    parsed_line
}
