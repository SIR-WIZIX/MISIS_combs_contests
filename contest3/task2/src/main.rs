use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut line = input.trim().split_whitespace();
    let n: usize = line.next().unwrap().parse().expect("");
    let m: usize = line.next().unwrap().parse().expect("");

    let mut data: Vec<Vec<i32>> = Vec::new();
    data.resize(n, Vec::new());
    for i in 0..n {
        data[i].resize(n, 0);
    }

    let mut total: i32 = 0;
    for _ in 0..m {
        input = String::new();
        io::stdin().read_line(&mut input).expect("");

        let mut line = input.trim().split_whitespace();
        let a: usize = line.next().unwrap().parse().expect("");
        let b: usize = line.next().unwrap().parse().expect("");
        if a == b {
            continue;
        }

        if data[a - 1][b - 1] == 0 {
            data[a - 1][b - 1] = 1;
            total += 1;
        }
    }
    if total != ((n * (n - 1)) / 2) as i32 {
        print!("NO\n");
    } else {
        print!("YES\n");
    }
}
