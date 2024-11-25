fn breaking_records(scores: Vec<i32>) -> Vec<i32> {
    let mut max_count = 0;
    let mut min_count = 0;
    let mut max_score = scores[0];
    let mut min_score = scores[0];

    for &score in &scores[1..] {
        if score > max_score {
            max_score = score;
            max_count += 1;
        } else if score < min_score {
            min_score = score;
            min_count += 1;
        }
    }

    vec![max_count, min_count]
}

fn main() {
    let scores0 = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
    let result0 = breaking_records(scores0);
    println!("{} {}", result0[0], result0[1]);

    let scores1 = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
    let result1 = breaking_records(scores1);
    println!("{} {}", result1[0], result1[1]);
}
