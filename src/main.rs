use std::cell::Cell;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;

#[derive(Debug)]
struct Location<'a> {
    row: u16,
    col: u16,
    value: u8,
    f: Cell<i32>,
    g: Cell<i32>,
    parent: Option<&'a Location<'a>>
}

struct RefMap<'a> {
    cells: Vec<Vec<Location<'a>>>
}

impl<'a> RefMap<'a> {

    fn new() -> RefMap<'a> {

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
    fn print_map(&self) {
        for row in &self.cells {
            for cell in row {
                print!("{0} ", cell.value);
            }
            println!();
        }
    }

    /// Returns an iterable collection of neighbours.
    /// 
    /// for example, if x is your given co-ordinates, then it's surroding
    /// locations are it's neighbours
    /// 
    /// 0 0 0 0 0 0 0 0
    /// 0 0 0 0 0 0 0 0
    /// 0 0 0 y 0 0 0 0
    /// 0 0 y x y 0 0 0
    /// 0 0 0 y 0 0 0 0
    /// 0 0 0 0 0 0 0 0
    /// 0 0 0 0 0 0 0 0
    /// 
    /// (4,4) -> (4,2) (4,3) (2,4) (3,4)
    fn neighbours(&self, row: usize, col: usize ) -> Vec<&Location> {
        let mut neighbours: Vec<&Location> = Vec::with_capacity(4) ;

        if col > 0 { neighbours.push(&self.cells[row][col - 1]); }

        if col < self.cells.len() - 1 {
            neighbours.push(&self.cells[row][col + 1]);
        }

        if row > 0 { neighbours.push(&self.cells[row - 1][col]); }

        if row < self.cells.len() - 1 { neighbours.push(&self.cells[row + 1][col]); }

        neighbours
    }
}

struct PathFinder<'a> {
    start: &'a Location<'a>,
    goal: &'a Location<'a>,
    map: RefMap<'a>
}

impl Hash for Location<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.col.hash(state);
    }
}

fn main() {
    let map: RefMap = RefMap::new();
    let start = Location::new(0, 0, 0);
    let goal = Location::new(5, 7, 0);
    let path_finder = PathFinder { start: &start, goal: &goal, map };

    path_finder.find();
}

impl Eq for Location<'_> {}

impl<'a> Location<'a> {
    fn new(row: u16, col : u16, value: u8) -> Location<'a> {
        Location {
            row,
            col,
            value,
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

impl PathFinder<'_> {

    fn heuristic(&self, location: &Location) -> i32 {
        let dx = (i32::from(location.row) - i32::from(self.goal.row)).abs();
        let dy = (i32::from(location.col) - i32::from(self.goal.col)).abs();

        dx + dy
    }

    fn find(&self) {
        println!("Start location is: ({0},{1})", self.start.row, self.start.col);
        println!("Goal location is: ({0},{1})\n", self.goal.row, self.goal.col);
        self.map.print_map();
    }
}

#[test]
fn location_equality() {
    let origin = Location::new(0, 0, 0);

    let origin_duplicate = Location::new(0, 0, 0);

    let random_point = Location::new(10, 10, 0);

    assert!(origin == origin_duplicate);
    assert!(origin_duplicate != random_point);
    assert!(origin != random_point);
}

#[test]
fn location_ordering() {
    let origin = Location {
        row: 0,
        col: 0,
        value: 0,
        f: Cell::new(0),
        g: Cell::new(0),
        parent: None
    };

    let origin_duplicate = Location {
        row: 0,
        col: 0,
        value: 0,
        f: Cell::new(5),
        g: Cell::new(0),
        parent: None
    };

    let random_point = Location {
        row: 10,
        col: 10,
        value: 0,
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

#[test]
fn hash_value_test() {
    let mut closed: HashSet<&Location> = HashSet::new();

    let a = Location::new(0, 0, 0);
    let b = Location::new(1, 0, 0);
    let c = Location::new(0, 0, 0);

    closed.insert(&a);
    closed.insert(&b);
    closed.insert(&c);

    assert_eq!(2, closed.len());

    // Clear hashset
    closed.clear();
    assert_eq!(0, closed.len());

    // Case to add multiple references of the same struct
    closed.insert(&a);
    closed.insert(&a);
    closed.insert(&a);

    // Value should be 1, not 3
    assert_eq!(1, closed.len());
}

#[test]
fn heuristic_test() {
    let map: RefMap = RefMap::new();
    let start = Location::new(0, 0, 0);
    let goal = Location::new(5, 7, 0);
    let path_finder = PathFinder { start: &start, goal: &goal, map };
 
    assert_eq!(12, path_finder.heuristic(&start));
}