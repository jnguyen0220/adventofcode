// Import the shared `common.rs` module (relative path) so this file
// can be compiled directly with `rustc` (no Cargo required).
#[path = "../common.rs"]
mod common;
use common::read_integers_from_file;
use std::collections::HashMap;
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
    let mut ordered: Vec<(i64, Vec<usize>)> = Vec::new();
    let mut seen: Vec<i64> = Vec::new();
    let mut map: HashMap<i64, Vec<usize>> = HashMap::new();

    for (i, c) in line.chars().enumerate() {
        let num = c.to_digit(10).unwrap() as i64;
        map.entry(num).or_insert(Vec::new()).push(i);
    }

    let mut possible: Vec<i64> = Vec::new();
    const MAX_LENGTH: usize = 11;
    while possible.len() <= MAX_LENGTH {
        let cutoff = line.len() - (MAX_LENGTH - possible.len());
        let mut found: Vec<(i64, usize)> = Vec::new();
        for (num, indices) in &map {
            for (i, index) in indices.iter().enumerate() {
                if *index < cutoff {
                    found.push((*num, *index));
                    break;
                }
            }
        }
        if let Some(max) = found.iter().max_by_key(|&&(a, _)| a) {
            possible.push(max.0);
            for vec in map.values_mut() {
                vec.retain(|&x| x > max.1);
            }

            map.retain(|_, v| !v.is_empty());
        }
        // println!("{:?} {:?} {:?}", possible, found, map);
    }
    let num = possible.into_iter().fold(0, |acc, d| acc * 10 + d);
    num
}
