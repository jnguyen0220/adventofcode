// Import the shared `common.rs` module (relative path) so this file
// can be compiled directly with `rustc` (no Cargo required).
#[path = "../common.rs"]
mod common;
use common::factors;
use common::is_unique;
use common::read_integers_from_file;
use std::collections::HashSet;

fn main() {
    let file_name = "temp.txt";
    // let file_name = "input.txt";

    match read_integers_from_file(file_name) {
        Ok(charges) => {
            let result: i64 = charges
                .iter()
                .map(|line| find_max_joltage(line))
                .collect::<Vec<i64>>()
                .iter()
                .sum();
            println!("Sum of invalid product IDs: {}", result);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}

fn find_max_joltage(line: &str) -> i64 {
    let mut numbers: Vec<i64> = Vec::new();
    for n in line.chars() {
        let num = n.to_digit(10).unwrap() as i64;
        // if !numbers.contains(&num) {
        numbers.push(num);
        // }
    }

    let mut combo: Vec<i64> = Vec::new();
    for i in 0..numbers.len() {
        for j in i+1..numbers.len() {
            let ab = format!("{}{}", numbers[i], numbers[j]).parse::<i64>().unwrap();
            combo.push(ab)
        }
    }
    // println!("{:?}", combo);
    // println!("{:?}", numbers);
    let max = combo.iter().max().unwrap();
    println!("{:?}", max);
    *max

    // combo.iter().max()
}