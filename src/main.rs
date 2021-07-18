use std::error::Error;

mod core;

fn main() -> Result<(), Box<dyn Error>> {
    let map = core::init();

    core::astar(&map, (1, 5), (9, 9));

    Ok(())
}
