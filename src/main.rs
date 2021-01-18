use std::cell::Cell;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Location<'a> {
    row: i32,
    col: i32,
    f: Cell<i32>,
    g: Cell<i32>,
    parent: Option<&'a Location<'a>>
}

impl Eq for Location<'_> {}

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

struct ArrayMap {}

impl ArrayMap {
    // returns an iterable collection of neighbours
    fn neighbours(&self) {
        todo!();
    }
}

struct PathFinder<'a> {
    start: &'a Location<'a>,
    goal: &'a Location<'a>,
    map: ArrayMap
}

impl PathFinder<'_> {

    fn heuristic(&self) {
        todo!();
    }

    fn find(&self) {
        todo!();
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
