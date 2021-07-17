mod location;
mod map;

use location::{GridCell, Location};
use std::cell::Cell;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;

use self::map::Map;

pub fn init() -> Map {
    let f= File::open("./maps/terrain22.txt").unwrap();
    let reader= BufReader::new(f);

    let mut map: Vec<Vec<Location>> = Vec::new();

    for line in reader.lines() {
        map.push(parse_line(line.unwrap(), map.len()));
    }

    map::Map::new(map)
}

pub fn astar(map: &Map, start: (usize, usize), end: (usize, usize)) {
    let start = Location::default(start.0, start.1);
    let end = Location::default(end.0, end.1);

    map.print();
}

fn parse_line(line: String, current_row: usize) -> Vec<Location> {
    let mut parsed_line: Vec<Location> = Vec::new();

    for c in line.chars() {
        let block= match c {
            '0' => GridCell::Path,
            '1' => GridCell::Wall,
            _ => continue,
        };

        let point = Location::new(parsed_line.len(), current_row, block, Cell::new(0));

        parsed_line.push(point);
    }

    parsed_line
}
