use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut line = input.trim().split_whitespace();
    let n: usize = line.next().unwrap().parse().expect("");

    let mut numbers: Vec<Vec<i32>> = Vec::new();
    numbers.resize(n as usize, Vec::new());

    for i in 0..n {
        input = String::new();
        io::stdin().read_line(&mut input).expect("");
        numbers[i as usize] = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Please type a number!"))
            .collect();
    }

    for x in 0..n {
        for y in 0..n {
            if numbers[x][y] == 1 {
                for z in 0..n {
                    if numbers[y][z] == 1 && numbers[x][z] != 1 {
                        println!("NO");
                        return;
                    }
                }
            }
        }
    }
    println!("YES");
}
