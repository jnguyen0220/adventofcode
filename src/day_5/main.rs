// Import the shared `common.rs` module (relative path) so this file
// can be compiled directly with `rustc` (no Cargo required).
#[path = "../common.rs"]
mod common;
use common::read_range_and_ingredents;
use std::collections::HashSet;

fn main() {
    let file_name = "temp.txt";
    // let file_name = "input.txt";

    match read_range_and_ingredents(file_name) {
        Ok((ranges, ingredents)) => find_fresh_ingredients(ranges, ingredents),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}

fn find_fresh_ingredients(ranges: Vec<(i64, i64, HashSet<usize>)>, ingredents: Vec<i64>) {
    println!("{:?} {:?}", ranges, ingredents)
}
