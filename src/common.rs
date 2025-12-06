use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

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

pub fn is_unique(s: &str) -> i32 {
    let mut seen = HashSet::new();
    s.chars().all(|c| seen.insert(c));
    seen.len() as i32
}
