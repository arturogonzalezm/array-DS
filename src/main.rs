use std::io::{self, BufRead};

/*
 * Complete the 'reverse_array' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

fn reverse_array(a: &[i32]) -> Vec<i32> {
    // Instead of converting to a vector and then reversing, directly collect into a reversed iterator.
    a.iter().rev().copied().collect()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let res = reverse_array(&arr);

    // Join the elements with a space and print them in one go to avoid multiple prints
    println!("{}", res.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
