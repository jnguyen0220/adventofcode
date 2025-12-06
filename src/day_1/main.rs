// Import the shared `common.rs` module (relative path) so this file
// can be compiled directly with `rustc` (no Cargo required).
#[path = "../common.rs"]
mod common;

use common::read_integers_from_file;

fn main() {
    // let file_name = "temp.txt";
    let file_name = "input.txt";

    match read_integers_from_file(file_name) {
        Ok(numbers) => {
            find_combo(numbers);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}

fn find_combo(combo: Vec<String>) {
    let mut current_dial: i32 = 50;
    let mut count: i32 = 0;
    println!("The dial starts by pointing at {}.", current_dial);
    combo.iter().for_each(|idx| {
        let direction = &idx[0..1];
        let val: i32 = idx[1..].parse().unwrap();
        let cycle: i32 = val / 100;
        let reminder: i32 = val % 100;
        match direction {
            "L" => {
                if reminder > current_dial {
                    if current_dial != 0 {
                        count += 1;
                        println!("Full rotation added at index {}", idx);
                    }
                    current_dial = 100 - (reminder - current_dial).abs();
                } else {
                    current_dial = current_dial - reminder;
                };
            }
            "R" => {
                let total = reminder + current_dial;
                if total > 100 {
                    count += 1;
                    println!("Full rotation added at index {}", idx);
                }
                current_dial = total % 100;
            }
            _ => println!("Invalid direction"),
        }
        // println!("Count: {}, index: {}", count, idx);
        if current_dial == 0 {
            count += 1;
        }
        // println!("{}", cycle);
        count += cycle;
        println!("The dial is rotated {} to point at {}", idx, current_dial);
    });
    println!("The lock opened after {} full rotations.", count);
}
