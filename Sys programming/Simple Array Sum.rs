use std::io;

fn simple_array_sum(ar: Vec<i32>) -> i32 {
    ar.iter().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let ar: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = simple_array_sum(ar);
    println!("{}", result);
}
