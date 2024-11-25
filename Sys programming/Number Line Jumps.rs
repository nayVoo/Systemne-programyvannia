fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        if x1 == x2 {
            return "YES".to_string();
        } else {
            return "NO".to_string();
        }
    }

    if (x1 - x2) % (v2 - v1) == 0 && (x1 - x2) / (v2 - v1) >= 0 {
        return "YES".to_string();
    }

    "NO".to_string()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let values: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let result = kangaroo(values[0], values[1], values[2], values[3]);
    println!("{}", result);
}
