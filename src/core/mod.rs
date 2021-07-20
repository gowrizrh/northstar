mod heap;
mod location;
mod map;
mod path_finder;
mod parser;

use location::Location;
use std::fs::File;
use std::io::BufReader;

use self::map::Map;
use self::path_finder::PathFinder;

pub fn init() -> Map {
    let f= File::open("./maps/terrain22.txt").unwrap();
    let reader = BufReader::new(f);

    parser::parse(reader)
}

pub fn astar(map: &Map, start: (usize, usize), end: (usize, usize)) {
    //  note start.0 and start.1 are flipped between x and y
    let start = Location::default(start.1, start.0);
    let end = Location::default(end.1, end.0);

    let path_finder = path_finder::Astar::new(map, &start, &end);

    path_finder.find();
}
