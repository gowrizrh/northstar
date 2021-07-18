use std::cell::Cell;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};

pub enum GridCell {
    Path,
    Wall,
}

pub struct Location {
    pub x: usize,
    pub y: usize,
    pub val: GridCell,
    pub f: Cell<i64>,
    pub g: Cell<i64>,
}

impl Location {
    pub fn new(x: usize, y: usize, val: GridCell, f: Cell<i64>, g: Cell<i64>) -> Self {
        Location { x, y, val, f, g }
    }

    pub fn default(x: usize, y: usize) -> Self {
        Location {
            x,
            y,
            val: GridCell::Path,
            f: Cell::new(0),
            g: Cell::new(0),
        }
    }
}

impl Eq for Location {}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Location {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        let diff: i64 = self.f.get() - other.f.get();

        if diff > 0 {
            Ordering::Greater
        } else if diff < 0 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.val)
    }
}

impl Display for GridCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridCell::Path => write!(f, "0"),
            _ => write!(f, "1"),
        }
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({0}, {1})", self.x, self.y)
    }
}

impl Debug for GridCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridCell::Path => write!(f, "0"),
            _ => write!(f, "1"),
        }
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::{Cell, GridCell, Location};
    use std::collections::HashSet;

    #[test]
    fn location_equality() {
        let origin = Location {
            x: 0,
            y: 0,
            val: GridCell::Path,
            f: Cell::new(i64::MAX),
            g: Cell::new(i64::MAX),
        };

        let origin_duplicate = Location {
            x: 0,
            y: 0,
            val: GridCell::Path,
            f: Cell::new(i64::MAX),
            g: Cell::new(i64::MAX),
        };

        let random_point = Location {
            x: 10,
            y: 150,
            val: GridCell::Path,
            f: Cell::new(i64::MAX),
            g: Cell::new(i64::MAX),
        };

        // origin is equal to origin duplicate even though they are separate objects
        assert!(origin == origin_duplicate);

        assert!(origin_duplicate != random_point);
        assert!(origin != random_point);
    }

    #[test]
    fn hash_value_test() {
        let mut closed: HashSet<&Location> = HashSet::new();

        let a = Location {
            x: 0,
            y: 0,
            val: GridCell::Path,
            f: Cell::new(i64::MAX),
            g: Cell::new(i64::MAX),
        };

        let b = Location {
            x: 1,
            y: 0,
            val: GridCell::Path,
            f: Cell::new(i64::MAX),
            g: Cell::new(i64::MAX),
        };

        let c = Location {
            x: 0,
            y: 0,
            val: GridCell::Path,
            f: Cell::new(i64::MAX),
            g: Cell::new(i64::MAX),
        };

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
    fn location_ordering() {
        let origin = Location {
            x: 0,
            y: 0,
            val: GridCell::Path,
            f: Cell::new(0),
            g: Cell::new(0),
        };

        let origin_duplicate = Location {
            x: 0,
            y: 0,
            val: GridCell::Path,
            f: Cell::new(5),
            g: Cell::new(5),
        };

        let random_point = Location {
            x: 10,
            y: 10,
            val: GridCell::Path,
            f: Cell::new(10),
            g: Cell::new(10),
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
}
