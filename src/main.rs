use std::error::Error;

mod core;

fn main() -> Result<(), Box<dyn Error>> {
    let map = core::init();

    core::astar(&map, (0, 0), (10,2));

    Ok(())
}
