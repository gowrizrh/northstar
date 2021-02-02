mod core;

use self::core::path_finder::Astar;

fn main() {
    let start = core::Location::new(0, 0, 0);
    let goal = core::Location::new(5, 7, 0);

    let path_finder = Astar::new(&start, &goal);

    path_finder.find();
}
