use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut n: i32 = input.trim().parse().expect("");

    input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please type a number!"))
        .collect();
}
