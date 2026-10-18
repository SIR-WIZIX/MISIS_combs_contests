use core::num;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut line = input.trim().split_whitespace();
    let n: usize = line.next().unwrap().parse().expect("");
    let m: usize = line.next().unwrap().parse().expect("");

    let mut data: Vec<Vec<i32>> = Vec::new();
    data.resize(n, Vec::new());

    let mut color: Vec<i32> = Vec::new();
    color.resize(n as usize, 0);

    for i in 0..n {
        data[i].resize(n, 0);
    }

    for _ in 0..m {
        input = String::new();
        io::stdin().read_line(&mut input).expect("");

        let mut line = input.trim().split_whitespace();
        let a: usize = line.next().unwrap().parse().expect("");
        let b: usize = line.next().unwrap().parse().expect("");

        data[a - 1][b - 1] = 1;
        data[b - 1][a - 1] = 1;
    }
    let t = BFS(0, &mut color, &data);

    println!("{}", t);
    for i in 0..color.len() {
        if color[i] == 1 {
            print!("{} ", i + 1);
        }
    }
    println!();
}

fn BFS(v: usize, mut colors: &mut Vec<i32>, numbers: &Vec<Vec<i32>>) -> i32 {
    colors[v] = 1;
    let mut total = 1;
    for next in 0..colors.len() {
        if numbers[v][next] == 0 {
            continue;
        }
        if colors[next] == 0 {
            total += BFS(next, &mut colors, &numbers);
        }
    }
    total
}
