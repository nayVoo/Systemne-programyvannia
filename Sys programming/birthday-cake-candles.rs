use std::io;

fn birthday_cake_candles(candles: Vec<i32>) -> i32 {
    let max_height = *candles.iter().max().unwrap();
    candles.iter().filter(|&&c| c == max_height).count() as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let candles: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    
    let result = birthday_cake_candles(candles);
    println!("{}", result);
}
