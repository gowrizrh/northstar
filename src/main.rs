use std::cell::Cell;
use std::cmp::Ordering;

#[derive(Debug)]
struct Location<'a> {
    row: i32,
    col: i32,
    f: Cell<i32>,
    g: Cell<i32>,
    parent: Option<&'a Location<'a>>
}

struct ArrayMap {
    cells: [[u8; 10]; 10]
}

struct PathFinder<'a> {
    start: &'a Location<'a>,
    goal: &'a Location<'a>,
    map: ArrayMap
}

fn main() {
    // Create a map, a start and a goal
    let map: ArrayMap = ArrayMap::demo_map();
    let start = Location::new(0, 0);
    let goal = Location::new(4, 5);

    // Feed it into the pathfinder
    let path_finder = PathFinder { start: &start, goal: &goal, map };

    // Visualize the map, also used for debugging purposes
    path_finder.print_map();

    // A*!
    path_finder.find();
}

impl Eq for Location<'_> {}

impl<'a> Location<'a> {
    fn new(row: i32, col : i32) -> Location<'a> {
        Location {
            row,
            col,
            f: Cell::new(i32::MAX),
            g: Cell::new(i32::MAX),
            parent: None
        }
    }
}

impl<'a> PartialEq for Location<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl<'a> Ord for Location<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let diff: i32 = self.f.get() - other.f.get();

        if diff > 0 { Ordering::Greater }
        else if diff < 0 { Ordering::Less }
        else { Ordering::Equal }
    }
}

impl<'a> PartialOrd for Location<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ArrayMap {
    // returns an iterable collection of neighbours
    fn neighbours(&self) {
        todo!();
    }

    fn demo_map() -> ArrayMap {
        // Statically fixed array of u8s are fine for now as primitive array
        // types cannot be dynamic
        let mut building_cells: [[u8; 10]; 10] = [[0; 10]; 10];

        // This hardcoded map will do for now, 0 are traversable cells
        // and 1 is a wall and cannot be passed through
        let map =
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

        let mut row: usize = 0;
        let mut col: usize = 0;

        for line in map.lines() {
            for character in line.split_whitespace() {
                // print!("({0}, {1})", row, col);

                building_cells[row][col] = character.parse::<u8>().unwrap();
                col = col +  1;
            }
            col = 0;
            row = row + 1;
            // println!();
        }

        ArrayMap { cells: building_cells }
    }
}


impl PathFinder<'_> {

    fn heuristic(&self) {
        todo!();
    }

    fn find(&self) {
        println!("hey hey!");
    }

    fn print_map(&self) {
        for row in &self.map.cells {
            for col in row {
                print!("{0} ", col);
            }
            println!();
        }
    }
}

#[test]
fn location_equality() {
    let origin = Location {
        row: 0,
        col: 0,
        f: Cell::new(0),
        g: Cell::new(0),
        parent: None
    };

    let origin_duplicate = Location {
        row: 0,
        col: 0,
        f: Cell::new(0),
        g: Cell::new(0),
        parent: None
    };

    let random_point = Location {
        row: 10,
        col: 10,
        f: Cell::new(0),
        g: Cell::new(0),
        parent: None
    };

    assert!(origin == origin_duplicate);
    assert!(origin_duplicate != random_point);
    assert!(origin != random_point);
}

#[test]
fn location_ordering() {
    let origin = Location {
        row: 0,
        col: 0,
        f: Cell::new(0),
        g: Cell::new(0),
        parent: None
    };

    let origin_duplicate = Location {
        row: 0,
        col: 0,
        f: Cell::new(5),
        g: Cell::new(0),
        parent: None
    };

    let random_point = Location {
        row: 10,
        col: 10,
        f: Cell::new(10),
        g: Cell::new(0),
        parent: None
    };

    assert!(origin < origin_duplicate);
    assert!(origin_duplicate < random_point);

    origin.f.set(5);

    // 5 is not less than 5
    assert!(!(origin < origin_duplicate));
    assert!(!(origin > origin_duplicate));

    // 5 is less than or equal to 5
    assert!(origin <= origin_duplicate);
    assert!(origin >= origin_duplicate);

    random_point.f.set(5);

    assert!(origin <= random_point);
    assert!(origin >= random_point);
}
