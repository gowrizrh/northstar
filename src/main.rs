mod core;

fn main() {
    let start = core::Location::new(0, 0, 0);
    let goal = core::Location::new(5, 7, 0);

    let path_finder = core::PathFinder::new(&start, &goal);
    path_finder.find();
}
