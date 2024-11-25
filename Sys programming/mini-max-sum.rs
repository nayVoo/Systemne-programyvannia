use std::io;

fn mini_max_sum(arr: Vec<i64>) {
    let sum: i64 = arr.iter().sum();
    let min = sum - *arr.iter().max().unwrap();
    let max = sum - *arr.iter().min().unwrap();
    println!("{} {}", min, max);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i64> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    mini_max_sum(arr);
}
