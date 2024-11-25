use std::io;

fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: Vec<i32>, oranges: Vec<i32>) {
    let apple_count = apples.iter().filter(|&&d| {
        let position = a + d;
        position >= s && position <= t
    }).count();
    
    let orange_count = oranges.iter().filter(|&&d| {
        let position = b + d;
        position >= s && position <= t
    }).count();

    println!("{}", apple_count);
    println!("{}", orange_count);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut values: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let s = values[0];
    let t = values[1];
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    values = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let a = values[0];
    let b = values[1];
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let _n = values[2];
    let apples: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let oranges: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    count_apples_and_oranges(s, t, a, b, apples, oranges);
}
