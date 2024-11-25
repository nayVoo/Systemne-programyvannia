use std::io;

fn time_conversion(s: &str) -> String {
    let mut time = s.to_string();
    let period = time.split_off(8); 
    let mut hours: i32 = time[0..2].parse().unwrap();

    if period == "AM" {
        if hours == 12 {
            time.replace_range(0..2, "00");
        }
    } else {
        if hours != 12 {
            hours += 12;
            time.replace_range(0..2, &format!("{:02}", hours));
        }
    }

    time
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let result = time_conversion(input);
    println!("{}", result);
}
