// Import the shared `common.rs` module (relative path) so this file
// can be compiled directly with `rustc` (no Cargo required).
#[path = "../common.rs"]
mod common;
use common::read_integers_from_file;

fn main() {
    let file_name = "temp.txt";
    // let file_name = "input.txt";

    match read_integers_from_file(file_name) {
        Ok(rolls) => find_number_rolls(rolls),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}

fn find_number_rolls(lines: Vec<String>) {
    let mut rolls: Vec<usize> = Vec::new();
    let mut rolls_total: usize = 0;
    let x: usize = lines[0].len();
    let y: usize = lines.len();

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '@' {
                rolls.push(i * x + j);
            }
        }
    }

    let mut can_remove_rolls: bool = true;
    while can_remove_rolls {
        let mut rolls_count: usize = 0;
        for position in &rolls.clone() {
            let mut rolls_found: usize = 0;
            rolls_found += check_north(rolls.clone(), *position, x);
            rolls_found += check_south(rolls.clone(), *position, x, y);
            rolls_found += check_west(rolls.clone(), *position, x);
            rolls_found += check_east(rolls.clone(), *position, x);

            if rolls_found < 4 {
                rolls_count += 1;
                rolls_total += 1;
                if let Some(pos) = rolls.iter().position(|&x| x == *position) {
                    rolls.remove(pos);
                }
            }
        }
        if rolls_count == 0 {
            can_remove_rolls = false;
        }
        // println!("{}", rolls_count);
    }
    println!("{}", rolls_total);
}

fn check_west(idx: Vec<usize>, grid_position: usize, x: usize) -> usize {
    let mut results: usize = 0;
    let line_position = grid_position % x;
    if line_position.checked_sub(1).is_some() {
        if idx.contains(&(grid_position - 1)) {
            results += 1;
        }
    }
    results
}

fn check_east(idx: Vec<usize>, grid_position: usize, x: usize) -> usize {
    let mut results: usize = 0;
    let line_position = grid_position % x;
    if line_position + 1 < x {
        if idx.contains(&(grid_position + 1)) {
            results += 1;
        }
    }
    results
}

fn check_north(idx: Vec<usize>, grid_position: usize, x: usize) -> usize {
    let mut results: usize = 0;
    if grid_position.checked_sub(x).is_some() {
        let location = grid_position - x;
        if idx.contains(&location) {
            results += 1;
        }
        results += check_west(idx.clone(), location, x);
        results += check_east(idx.clone(), location, x);
    }
    results
}

fn check_south(idx: Vec<usize>, grid_position: usize, x: usize, y: usize) -> usize {
    let mut results: usize = 0;
    let line_position = grid_position + x;
    if line_position < x * y {
        if idx.contains(&line_position) {
            results += 1;
        }
        results += check_west(idx.clone(), line_position, x);
        results += check_east(idx.clone(), line_position, x);
    }
    results
}
