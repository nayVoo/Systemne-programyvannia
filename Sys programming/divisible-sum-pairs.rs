fn divisible_sum_pairs(n: usize, k: i32, ar: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 0..n {
        for j in i+1..n {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let n = 6;
    let k = 3;
    let ar = vec![1, 3, 2, 6, 1, 2];
    println!("{}", divisible_sum_pairs(n, k, ar));
}
