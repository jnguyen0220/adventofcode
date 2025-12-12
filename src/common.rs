use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

pub fn read_integers_from_file(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?; // Open the file, handle potential errors
    let reader = BufReader::new(file); // Create a buffered reader for efficient line-by-line reading

    let mut results = Vec::new(); // Initialize an empty vector to store the integers

    for line_result in reader.lines() {
        let line = line_result?; // Get the line as a String, handling potential errors during reading
        let trimmed_line = line.trim(); // Remove leading/trailing whitespace

        if trimmed_line.is_empty() {
            continue; // Skip empty lines
        }

        let result: String = trimmed_line.parse().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse '{}' as integer: {}", trimmed_line, e),
            )
        })?;
        // Attempt to parse the string into an i32, mapping parsing errors to io::Error
        results.push(result); // Add the parsed integer to the vector
    }
    Ok(results) // Return the vector of integers
}

pub fn read_and_print_comma_separated_values(
    file_path: &str,
) -> Result<Vec<(i64, i64)>, io::Error> {
    // Open the file
    let mut file = File::open(file_path)?;
    let mut contents = String::new();

    // Read the entire file into a string
    file.read_to_string(&mut contents)?;

    // Split by comma and collect into a vector
    let ranges: Result<Vec<(i64, i64)>, io::Error> = contents
        .split(',')
        .map(|pair| {
            let mut parts = pair.split('-');
            let start = parts
                .next()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing start value"))?
                .trim()
                .parse::<i64>()
                .map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Failed to parse start: {}", e),
                    )
                })?;
            let end = parts
                .next()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing end value"))?
                .trim()
                .parse::<i64>()
                .map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Failed to parse end: {}", e),
                    )
                })?;
            Ok((start, end))
        })
        .collect();
    ranges
}

pub fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as i64;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn factors(n: i64) -> Vec<i64> {
    let mut result = Vec::new();
    let limit = (n as f64).sqrt() as i64;

    for i in 1..=limit {
        if n % i == 0 {
            result.push(i);
            if i != n / i {
                result.push(n / i);
            }
        }
    }

    result.sort(); // optional: sort ascending
    result
}

pub fn read_range_and_ingredents(
    file_path: &str,
) -> Result<(Vec<(i64, i64, HashSet<i64>)>, Vec<i64>), io::Error> {
    let content = fs::read_to_string(file_path);
    let binding = content?;
    let parts: Vec<&str> = binding.split("\n\n").collect();
    let mut ranges: Vec<(i64, i64, HashSet<i64>)> = Vec::new();

    for line in parts[0].lines() {
        let partial: Vec<&str> = line.split("-").collect();
        let mut set: HashSet<i64> = HashSet::new();
        let start = partial[0].trim().parse().unwrap();
        let end = partial[1].trim().parse().unwrap();
        set.insert(partial[0].trim().len() as i64);
        set.insert(partial[1].trim().len() as i64);
        ranges.push((start, end, set));
    }

    let ingredents: Vec<i64> = parts[1]
        .lines()
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    Ok((ranges, ingredents))
}

// pub fn parse_math_problem(file_path: &str) -> Result<(Vec<i64>, Vec<String>), io::Error> {
//     let mut file = File::open(file_path)?;
//     let mut contents = String::new();
//     let mut matrix: HashMap<usize, Vec<i64>> = HashMap::new();
//     let mut operator: Vec<String> = Vec::new();
//     file.read_to_string(&mut contents)?;

//     let mut lines = contents.lines().collect::<Vec<_>>();

//     if let Some(last) = lines.pop() {
//         let mut current_line: String = "".to_string();
//         for position in last.chars() {
//             if position != ' ' {
//                 current_line += &position.to_string();
//             }

//             if position == ' ' && current_line.len() > 0 {
//                 operator.push(current_line);
//                 current_line = "".to_string();
//             }
//         }
//         if current_line.len() > 0 {
//             operator.push(current_line);
//         }
//     }

//     for line in lines {
//         let mut current_line: String = "".to_string();
//         for position in line.chars() {
//             if position != ' ' {
//                 current_line += &position.to_string();
//             }

//             if position == ' ' && current_line.len() > 0 {
//                 let value: i64 = current_line.parse().unwrap();
//                 matrix.push(value);
//                 current_line = "".to_string();
//             }
//         }
//         if current_line.len() > 0 {
//             let value: i64 = current_line.parse().unwrap();
//             matrix.push(value);
//         }
//     }

//     Ok((matrix, operator))
// }

pub fn parse_math_problem_vertical(file_path: &str) -> Result<(), io::Error> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let mut values: HashMap<usize, Vec<i64>> = HashMap::new();
    let reader = io::BufReader::new(file);
    let mut operator: Vec<char> = Vec::new();

    let mut lines: Vec<Vec<char>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    if lines.is_empty() {
        return Ok(());
    }

    if let Some(last) = lines.pop() {
        println!("{:?}", last);
    }

    let line_len = lines[0].len();
    let mut column: HashMap<usize, Vec<char>> = HashMap::new();
    let mut block: usize = 0;
    for i in 0..line_len - 1 {
        for line in &lines {
            column.entry(i).or_insert(Vec::new()).push(line[i]);
        }
        if let Some(value) = column.get(&i) {
            let total_spaces: usize = value.iter().filter(|&&c| c == ' ').count();
            if total_spaces == lines.len() {
                // let filter_values: Vec<char> = value.iter().filter(|&c| c != ' ').collect();
                let filtered: Vec<char> = value.iter().filter(|&&c| c != ' ').collect();
                println!("{:?}", column);
                column = HashMap::new();
                block += 1;
            }
        }
    }

    println!("{:?}", column);

    Ok(())
}
