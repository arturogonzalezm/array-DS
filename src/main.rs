use std::io::{self, BufRead};

/*
 * Complete the 'reverse_array' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

fn reverse_array(a: &[i32]) -> Vec<i32> {
    let mut reversed = a.to_vec();
    reversed.reverse();
    reversed
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let res = reverse_array(&arr);

    for i in 0..res.len() {
        print!("{}", res[i]);

        if i != res.len() - 1 {
            print!(" ");
        }
    }

    println!();
}
