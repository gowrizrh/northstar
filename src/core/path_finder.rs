use super::maps::RefMap;
use super::world::Location;

pub struct PathFinder<'a> {
    start: &'a Location<'a>,
    goal: &'a Location<'a>,
    map: RefMap<'a>
}

impl PathFinder<'_> {

    fn heuristic(&self, location: &Location) -> i32 {
        let dx = (i32::from(location.row) - i32::from(self.goal.row)).abs();
        let dy = (i32::from(location.col) - i32::from(self.goal.col)).abs();

        dx + dy
    }

    pub fn find(&self) {
        println!("Start location is: ({0},{1})", self.start.row, self.start.col);
        println!("Goal location is: ({0},{1})\n", self.goal.row, self.goal.col);
        self.map.print_map();
    }

    pub fn new<'a>(start: &'a Location, goal: &'a Location) -> PathFinder<'a> {
        PathFinder {
            start,
            goal,
            map: RefMap::new()
        }
    }
}

#[test]
fn heuristic_test() {
    let map: RefMap = RefMap::new();
    let start = Location::new(0, 0, 0);
    let goal = Location::new(5, 7, 0);
    let path_finder = PathFinder { start: &start, goal: &goal, map };
 
    assert_eq!(12, path_finder.heuristic(&start));
}