use std::io::{self, BufRead};
use std::num::ParseIntError;

fn reverse_array(a: &[i32]) -> Vec<i32> {
    a.iter().rev().copied().collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr_count = stdin_iterator.next().ok_or("No input for array count")??
        .trim().parse::<usize>()?;

    let arr: Result<Vec<i32>, ParseIntError> = stdin_iterator.next().ok_or("No input for array elements")??
        .split_whitespace()
        .map(|s| s.parse())
        .collect();

    let arr = arr?;

    if arr.len() != arr_count {
        return Err("Array length does not match the specified count".into());
    }

    let res = reverse_array(&arr);

    println!("{}", res.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));

    Ok(())
}