fn birthday(s: Vec<i32>, d: i32, m: i32) -> i32 {
    let mut count = 0;

    for i in 0..=s.len() - m as usize {
        let sum: i32 = s[i..(i + m as usize)].iter().sum();
        if sum == d {
            count += 1;
        }
    }

    count
}

fn main() {
    let s = vec![1, 2, 1, 3, 2];
    let d = 3;
    let m = 2;
    println!("{}", birthday(s, d, m)); 

    let s = vec![1, 1, 1, 1, 1, 1];
    let d = 3;
    let m = 2;
    println!("{}", birthday(s, d, m)); 

    let s = vec![4];
    let d = 4;
    let m = 1;
    println!("{}", birthday(s, d, m)); 
}
