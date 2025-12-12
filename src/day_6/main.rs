// Import the shared `common.rs` module (relative path) so this file
// can be compiled directly with `rustc` (no Cargo required).
#[path = "../common.rs"]
mod common;
use common::parse_math_problem_vertical;
use std::collections::HashMap;

fn main() {
    let file_name = "temp.txt";
    // let file_name = "input.txt";
    parse_math_problem_vertical(file_name);

    // match parse_math_problem_vertical(file_name) {
    //     Ok((matrix, operator)) => solve_math(matrix, operator),
    //     Err(e) => {
    //         eprintln!("Error reading file: {}", e);
    //     }
    // }
}

// fn solve_math(matrix: Vec<i64>, operator: Vec<String>) {
//     let mut map: HashMap<i64, Vec<i64>> = HashMap::new();
//     for (idx, value) in matrix.iter().enumerate() {
//         let map_id: i64 = (idx % operator.len()) as i64;
//         map.entry(map_id).or_insert(Vec::new()).push(*value);
//     }

//     // for (key, value) in &map {
//     //     let mut map_str: HashMap<i64, Vec<String>> = HashMap::new();
//     //     if let Some(max) = value.iter().max() {
//     //         let max_str = max.to_string();
//     //         for val in value.iter() {
//     //             let val_str = val.to_string();
//     //             for (idx, num) in val_str.chars().rev().enumerate() {
//     //                 map_str
//     //                     .entry(max_str.len() - val_str.len() + idx )
//     //                     .or_insert(Vec::new())
//     //                     .push(num.to_string());
//     //             }
//     //         }
//     //     }
//     //     println!("{:?}", map_str);
//     // }
//     for (key, value) in &map {
//         let mut counts: HashMap<usize, usize> = HashMap::new();

//         for n in value {
//             // Get digit length (string-based for simplicity)
//             let len = n.to_string().len();
//             // Increment count in the HashMap
//             *counts.entry(len).or_insert(0) += 1;
//         }

//         println!("{:?}", counts);

//         // if let Some(max) = value.iter()
//         //     let max_str = max.to_string();
//         //     for val in value.iter() {
//         //         let i = val.to_string();
//         //         let fill_len = max_str.len() - i.len();
//         //         map_str
//         //             .entry(*key)
//         //             .or_insert(Vec::new())
//         //             .push("0".repeat(fill_len) + &i);
//         //     }
//         // }
//     }

//     // println!("{:?}", map_str);

//     let mut sum: i64 = 0;
//     for (idx, oper) in operator.iter().enumerate() {
//         if let Some(value) = map.get(&(idx as i64)) {
//             match oper.as_str() {
//                 "*" => sum += value.iter().fold(1, |acc, &x| acc * x),
//                 "+" => sum += value.iter().sum::<i64>(),
//                 _ => {
//                     println!("Unknown operator: {}", oper);
//                 }
//             }
//         }
//     }

//     // println!("{:?} {:?}", sum, map);
// }
