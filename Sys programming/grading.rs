use std::io;

fn grading_students(grades: Vec<i32>) -> Vec<i32> {
    grades.iter().map(|&grade| {
        if grade < 38 {
            grade
        } else {
            let next_multiple_of_5 = (grade / 5 + 1) * 5;
            if next_multiple_of_5 - grade < 5 {
                next_multiple_of_5
            } else {
                grade
            }
        }
    }).collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    
    let mut grades = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let grade: i32 = input.trim().parse().unwrap();
        grades.push(grade);
    }
    
    let result = grading_students(grades);
    for grade in result {
        println!("{}", grade);
    }
}
