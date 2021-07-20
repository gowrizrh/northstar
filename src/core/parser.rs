use super::location::{GridCell, Location};
use super::map::Map;
use std::cell::Cell;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;

pub fn parse(reader: BufReader<File>) -> Map {
    let mut map: Vec<Vec<Location>> = Vec::new();

    for line in reader.lines() {
        map.push(parse_line(line.unwrap(), map.len()));
    }

    Map::new(map)
}

fn parse_line(line: String, current_row: usize) -> Vec<Location> {
    let mut parsed_line: Vec<Location> = Vec::new();

    for c in line.chars() {
        let block = match c {
            '0' => GridCell::Path,
            '1' => GridCell::Wall,
            _ => continue,
        };

        let point = Location::new(
            parsed_line.len(),
            current_row,
            block,
            Cell::new(i64::MAX),
            Cell::new(i64::MAX),
        );

        parsed_line.push(point);
    }

    parsed_line
}
