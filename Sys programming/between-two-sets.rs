fn getTotalX(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut count = 0;
    let max_a = *a.iter().max().unwrap();
    let min_b = *b.iter().min().unwrap();
    for x in (max_a..=min_b).step_by(max_a as usize) {
        let mut valid = true;
        for &num in &a {
            if x % num != 0 {
                valid = false;
                break;
            }
        }
        if valid {
            for &num in &b {
                if num % x != 0 {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            count += 1;
        }
    }

    count
}

fn main() {
    let a = vec![2, 4];
    let b = vec![16, 32, 96];
    
    let result = getTotalX(a, b);
    println!("{}", result);
}
