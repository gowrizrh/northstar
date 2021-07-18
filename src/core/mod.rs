mod heap;
mod location;
mod map;
mod path_finder;

use location::{GridCell, Location};
use std::cell::Cell;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;

use self::map::Map;
use self::path_finder::PathFinder;

pub fn init() -> Map {
    let f = File::open("./maps/terrain22.txt").unwrap();
    let reader = BufReader::new(f);

    let mut map: Vec<Vec<Location>> = Vec::new();

    for line in reader.lines() {
        map.push(parse_line(line.unwrap(), map.len()));
    }

    map::Map::new(map)
}

pub fn heuristic(location: &Location, goal: &Location) -> i64 {
    let lx: i64 = location.x.try_into().unwrap();
    let ly: i64 = location.y.try_into().unwrap();

    let gx: i64 = goal.x.try_into().unwrap();
    let gy: i64 = goal.y.try_into().unwrap();

    let dx = lx - gx;
    let dy = ly - gy;

    dx.abs() + dy.abs()
}

pub fn astar(map: &Map, start: (usize, usize), end: (usize, usize)) {
    //  note start.0 and start.1 are flipped between x and y
    let start = Location::default(start.1, start.0);
    let end = Location::default(end.1, end.0);

    let path_finder = path_finder::Astar::new(map, &start, &end);

    path_finder.find();
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
