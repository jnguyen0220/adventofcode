// Import the shared `common.rs` module (relative path) so this file
// can be compiled directly with `rustc` (no Cargo required).
#[path = "../common.rs"]
mod common;
use common::read_range_and_ingredents;
use std::collections::HashSet;

fn main() {
    // let file_name = "temp.txt";
    let file_name = "input.txt";

    match read_range_and_ingredents(file_name) {
        Ok((ranges, ingredents)) => find_fresh_ingredients(ranges, ingredents),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}

fn find_fresh_ingredients(mut ranges: Vec<(i64, i64, HashSet<i64>)>, ingredents: Vec<i64>) {
    let mut fresh: HashSet<i64> = HashSet::new();
    let mut remove_idx: Vec<usize> = Vec::new();

    for (idx, (start, end, size)) in ranges.clone().into_iter().enumerate() {
        if end - start == 0 {
            remove_idx.push(idx)
        }
    }

    // for &idx in remove_idx.iter().rev() {
    //     ranges.remove(idx);
    // }

    for ingredent in &ingredents {
        let mut found: Vec<(i64, i64)> = Vec::new();
        let ingredent_str: &str = &ingredent.to_string();
        for (start, end, size) in &ranges {
            if size.contains(&(ingredent_str.len() as i64)) {
                found.push((*start, *end));
            }
        }
        for (start, end) in &found {
            if &ingredent > &start && &ingredent <= &end {
                fresh.insert(*ingredent);
            }
        }
    }

    for (idx, (start, end, size)) in ranges.clone().into_iter().enumerate() {
        if end - start == 0 {
            remove_idx.push(idx)
        }
    }

    let mut order_range: Vec<_> = ranges
        .clone()
        .into_iter()
        .map(|(start, end, _)| (start, end))
        .collect::<Vec<_>>();

    order_range.sort_by_key(|r| r.0);

    let mut merged: Vec<(i64, i64)> = Vec::new();
    for (start, end) in order_range {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                // overlap or touching → extend
                last.1 = last.1.max(end);
            } else {
                // no overlap → push new
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }
    let total: i64 = merged.iter().map(|(s, e)| e - s + 1).sum();

    println!("{:?}", total);

    // let mut intersections: i64 = 0
    // let mut ingredent_id: i64 = 0;

    // while let Some((start, end, size)) = ranges.pop() {
    //     ingredent_id += end - start + 1;
    //     let start_str = start.to_string().len() as i64;
    //     let end_str = start.to_string().len() as i64;
    //     let mut found_start: Vec<_> = ranges
    //         .iter()
    //         .filter(|(_, _, size)| size.contains(&start_str))
    //         .map(|(start, end, _)| (start, end))
    //         .collect();

    //     // let found_end: Vec<_> = ranges
    //     //     .iter()
    //     //     .filter(|(_, _, size)| size.contains(&end_str))
    //     //     .collect();
    //     found_start.sort_by_key(|r| r.0);

    //     for (min, max) in found_start {
    //         let _start = start.max(*min);
    //         let _end = end.min(*max);
    //         if _start <= _end {
    //             let count = _end - _start + 1;
    //             ingredent_id -= count;
    //         }
    //     }
    //     // println!("{:?} {:?} {:?}", start, end, ingredent_id);
    // }

    // println!("{:?} {:?}", fresh.len(), ingredent_id);
}
