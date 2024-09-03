use std::io::{self, BufRead};
use std::num::ParseIntError;

fn reverse_array(a: &[i32]) -> Vec<i32> {
    a.iter().rev().copied().collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr_count: usize = stdin_iterator
        .next()
        .ok_or("No input for array count")?
        .map_err(|e| format!("Failed to read array count: {}", e))?
        .trim()
        .parse()
        .map_err(|e: ParseIntError| format!("Invalid array count: {}", e))?;

    let arr: Vec<i32> = stdin_iterator
        .next()
        .ok_or("No input for array elements")?
        .map_err(|e| format!("Failed to read array elements: {}", e))?
        .split_whitespace()
        .enumerate()
        .map(|(i, s)| s.parse().map_err(|e: ParseIntError| format!("Invalid integer at position {}: {}", i, e)))
        .collect::<Result<_, _>>()?;

    if arr.len() != arr_count {
        return Err(format!("Array length ({}) does not match the specified count ({})", arr.len(), arr_count).into());
    }

    let res = reverse_array(&arr);

    println!("{}", res.iter().map(ToString::to_string).collect::<Vec<_>>().join(" "));

    Ok(())
}