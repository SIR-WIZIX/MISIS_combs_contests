use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut line = input.trim().split_whitespace();
    let n: usize = line.next().unwrap().parse().expect("");
    let m: usize = line.next().unwrap().parse().expect("");

    input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut line = input.trim().split_whitespace();
    let s: usize = line.next().unwrap().parse().expect("");
    let e: usize = line.next().unwrap().parse().expect("");

    let mut table: Vec<Vec<f64>> = Vec::new();
    table.resize(n, Vec::new());
    table.iter_mut().for_each(|x| x.resize(n, 0.0));

    for _ in 0..m {
        input = String::new();
        io::stdin().read_line(&mut input).expect("");

        let mut line = input.trim().split_whitespace();
        let si: i32 = line.next().unwrap().parse().expect("");
        let ei: i32 = line.next().unwrap().parse().expect("");
        let pi: i32 = line.next().unwrap().parse().expect("");

        table[si as usize - 1][ei as usize - 1] = 1.0 - (pi as f64 * 0.01);
        table[ei as usize - 1][si as usize - 1] = 1.0 - (pi as f64 * 0.01);
    }

    println!("{:.7}", 1.0 - floyd(s - 1, e - 1, &mut table));
}

fn floyd(s: usize, f: usize, table: &mut Vec<Vec<f64>>) -> f64 {
    for k in 0..table.len() {
        for i in 0..table.len() {
            for j in 0..table.len() {
                table[i][j] = table[i][j].max(table[i][k] * table[k][j])
            }
        }
    }
    table[s][f]
}
