fn migratory_birds(arr: Vec<i32>) -> i32 {
    let mut freq = vec![0; 6];
    
    for &bird in &arr {
        freq[bird as usize] += 1;
    }

    let max_freq = *freq.iter().max().unwrap();
    
    for i in 1..freq.len() {
        if freq[i] == max_freq {
            return i as i32;
        }
    }
    
    0
}

fn main() {
    let arr = vec![1, 4, 4, 4, 5, 3];
    println!("{}", migratory_birds(arr)); 

    let arr = vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4];
    println!("{}", migratory_birds(arr)); 
}
