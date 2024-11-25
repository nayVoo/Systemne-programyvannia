fn birthday(s: Vec<i32>, d: i32, m: i32) -> i32 {
    let mut count = 0;
    for i in 0..=s.len() - m as usize {
        if s[i..i + m as usize].iter().sum::<i32>() == d {
            count += 1;
        }
    }
    count
}

fn main() {
    let s0 = vec![1, 2, 1, 3, 2];
    let (d0, m0) = (3, 2);
    println!("{}", birthday(s0, d0, m0)); 

    let s1 = vec![1, 1, 1, 1, 1, 1];
    let (d1, m1) = (3, 2);
    println!("{}", birthday(s1, d1, m1)); 

    let s2 = vec![4];
    let (d2, m2) = (4, 1);
    println!("{}", birthday(s2, d2, m2)); 
}
