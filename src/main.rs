use std::cell::Cell;

fn main() {
    println!("Hello, world!");
}

struct Location<'a> {
    row: i32,
    col: i32,
    f: Cell<i32>,
    g: Cell<i32>,
    parent: Option<&'a Location<'a>>
}
