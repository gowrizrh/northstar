use std::collections::HashSet;

use super::{
    heap::BinaryHeap,
    location::{GridCell, Location},
    map::Map,
};

use std::convert::TryInto;

pub trait PathFinder {
    fn find(&self) -> Option<i32>;

    fn heuristic(&self, location: &Location) -> i64;
}

pub struct Astar<'a> {
    start: &'a Location,
    end: &'a Location,
    map: &'a Map,
}

impl<'a> Astar<'a> {
    pub fn new(map: &'a Map, start: &'a Location, end: &'a Location) -> Self {
        Astar { map, start, end }
    }
}

impl PathFinder for Astar<'_> {
    fn find(&self) -> Option<i32> {
        let mut open: BinaryHeap<&Location> = BinaryHeap::new();
        let mut closed: HashSet<&Location> = HashSet::new();

        let mut current = self.start;
        current.f.set(self.heuristic(current));
        open.push(current);

        while !open.is_empty() {
            current = open.pop().unwrap();
            closed.insert(current);

            if current.eq(self.end) {
                println!("Target!");
                break;
            }

            for neighbour in self.map.neighbours(current) {
                match neighbour.val {
                    GridCell::Path => {
                        if closed.contains(neighbour) {
                            continue;
                        }

                        ()
                    }
                    _ => continue,
                }

                let new_g = current.g.get() + 1;

                if new_g < neighbour.g.get() {
                    let f = new_g + self.heuristic(neighbour);
                    neighbour.g.set(new_g);
                    neighbour.f.set(f);

                    if !open.contains(neighbour) {
                        open.push(neighbour)
                    }
                }
            }
        }

        Some(0)
    }

    /// Manhattan heuristic
    fn heuristic(&self, location: &Location) -> i64 {
        let lx: i64 = location.x.try_into().unwrap();
        let ly: i64 = location.y.try_into().unwrap();

        let gx: i64 = self.end.x.try_into().unwrap();
        let gy: i64 = self.end.y.try_into().unwrap();

        let dx = lx - gx;
        let dy = ly - gy;

        dx.abs() + dy.abs()
    }
}
