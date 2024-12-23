fn page_count(n: i32, p: i32) -> i32 {
    let from_front = p / 2;
    let from_back = (n / 2) - (p / 2);
    std::cmp::min(from_front, from_back)
}

fn main() {
    let n = 6;
    let p = 2;
    let result = page_count(n, p);
    println!("{}", result);
}
