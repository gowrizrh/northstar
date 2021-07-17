mod location;
mod map;

use location::{GridCell, Location};
use std::cell::Cell;
use std::error::Error;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;

pub fn init() -> Result<(), Box<dyn Error>> {
    let f: File = File::open("./maps/terrain23.txt")?;
    let reader: BufReader<File> = BufReader::new(f);

    let mut map: Vec<Vec<Location>> = Vec::new();

    for line in reader.lines() {
        map.push(parse_line(line?, map.len()));
    }

    let map: map::Map = map::Map::new(map);
    map.print();

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
            f: Cell::new(0),
        };

        parsed_line.push(point);
    }

    parsed_line
}
