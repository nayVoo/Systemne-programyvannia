use std::io;

fn plus_minus(arr: Vec<i32>) {
    let n = arr.len();
    let (mut positive, mut negative, mut zero) = (0, 0, 0);

    for &num in arr.iter() {
        if num > 0 {
            positive += 1;
        } else if num < 0 {
            negative += 1;
        } else {
            zero += 1;
        }
    }

    println!("{:.6}", positive as f64 / n as f64);
    println!("{:.6}", negative as f64 / n as f64);
    println!("{:.6}", zero as f64 / n as f64);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    
    plus_minus(arr);
}
