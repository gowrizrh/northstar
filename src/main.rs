use std::cell::Cell;

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

impl<'a> PartialEq for Location<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
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
