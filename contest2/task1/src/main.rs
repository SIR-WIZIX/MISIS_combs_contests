use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let n: i32 = input.trim().parse().expect("");

    for i in 1..n + 1 {
        input = String::new();
        io::stdin().read_line(&mut input).expect("");

        let numbers: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Please type a number!"))
            .collect();

        for j in 1..numbers.len() + 1 {
            if numbers[j - 1] == 1 {
                println!("{} {}", i, j);
            }
        }
    }
}
