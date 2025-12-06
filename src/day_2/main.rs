// Import the shared `common.rs` module (relative path) so this file
// can be compiled directly with `rustc` (no Cargo required).
#[path = "../common.rs"]
mod common;
use common::factors;
use common::is_unique;
use common::read_and_print_comma_separated_values;
use std::collections::HashSet;

fn main() {
    // let file_name = "temp.txt";
    let file_name = "input.txt";

    match read_and_print_comma_separated_values(file_name) {
        Ok(ranges) => {
            let result: i64 = ranges
                .iter()
                .map(|(start, end)| find_invalid_product_id_part2(*start, *end))
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

fn find_invalid_product_id(start: i64, end: i64) -> i64 {
    let mut sum = 0;
    for i in start..=end {
        let number_string = i.to_string();
        if number_string.len() % 2 == 0 {
            let (left, right) = number_string.split_at(number_string.len() / 2);
            if left == right {
                sum += i;
            }
        }
    }
    sum
}

fn find_invalid_product_id_part2(start: i64, end: i64) -> i64 {
    let mut sum = 0;
    for i in start..=end {
        let number_string = i.to_string();

        // Skip single digit numbers
        if number_string.len() == 1 {
            continue;
        }

        // Check if palindrome
        let unique_count = is_unique(&number_string);
        if unique_count == 1 {
            sum += i;
            println!("{:?}", i);
            continue;
        }

        let factors: Vec<i64> = factors(number_string.len() as i64);
        let middle = &factors[1..factors.len() - 1];

        if middle.is_empty() {
            continue;
        }

        for &interval in middle {
            let mut hash = HashSet::new();
            for i in (0..number_string.len()).step_by(interval as usize) {
                let end = (i + interval as usize).min(number_string.len());
                hash.insert(&number_string[i..end]);
            }
            if hash.len() == 1 {
                sum += i;
                println!("{:?}", i);
                break;
            }
        }
    }
    sum
}
