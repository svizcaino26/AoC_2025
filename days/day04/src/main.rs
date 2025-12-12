use common::load_grid;

fn main() {
    let flat_grid =
        // load_grid("input/day04/sample.txt").unwrap_or_else(|e| panic!("Error loading file: {e}"));
        load_grid("input/day04/puzzle.txt").unwrap_or_else(|e| panic!("Error loading file: {e}"));

    println!("{flat_grid:?}");
}
