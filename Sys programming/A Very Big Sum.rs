use std::io;

fn a_very_big_sum(ar: Vec<i64>) -> i64 {
    ar.iter().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let ar: Vec<i64> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let result = a_very_big_sum(ar);
    println!("{}", result);
}
