use std::cell::Cell;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Location<'a> {
    pub row: u16,
    pub col: u16,
    pub value: u8,
    pub f: Cell<i32>,
    pub g: Cell<i32>,
    pub parent: Option<&'a Location<'a>>
}

impl Eq for Location<'_> {}

impl Hash for Location<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.col.hash(state);
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

impl<'a> Location<'a> {
    pub fn new(row: u16, col : u16, value: u8) -> Location<'a> {
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

// Tests

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
fn location_equality() {
    let origin = Location::new(0, 0, 0);

    let origin_duplicate = Location::new(0, 0, 0);

    let random_point = Location::new(10, 10, 0);

    assert!(origin == origin_duplicate);
    assert!(origin_duplicate != random_point);
    assert!(origin != random_point);
}
