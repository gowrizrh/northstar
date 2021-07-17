use std::error::Error;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;


mod location;
mod map;

pub fn init() -> Result<(), Box<dyn Error>> {
    let f: File = File::open("./maps/terrain23.txt")?;
    let reader: BufReader<File> = BufReader::new(f);

    let mut map: Vec<Vec<location::Location>> = Vec::new();

    for line in reader.lines() {
        map.push(parse_line(line?, map.len()));
    }

    let map: map::Map = map::Map::new(map);
    map.print();

    Ok(())
}

fn parse_line(line: String, current_row: usize) -> Vec<location::Location> {
    let mut parsed_line: Vec<location::Location> = Vec::new();

    for c in line.chars() {
        let block: location::GridCell = match c {
            '0' => location::GridCell::Path,
            '1' => location::GridCell::Wall,
            _ => continue,
        };

        let point: location::Location = location::Location {
            x: parsed_line.len(),
            y: current_row,
            val: block,
        };

        parsed_line.push(point);
    }

    parsed_line
}
