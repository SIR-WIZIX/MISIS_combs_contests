use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut line = input.trim().split_whitespace();
    let n: usize = line.next().unwrap().parse().expect("");
    let m: usize = line.next().unwrap().parse().expect("");

    let mut data: Vec<Vec<usize>> = Vec::new();
    data.resize(n, Vec::new());

    for _ in 0..m {
        input = String::new();
        io::stdin().read_line(&mut input).expect("");

        line = input.trim().split_whitespace();
        let a: usize = line.next().unwrap().parse().expect("");
        let b: usize = line.next().unwrap().parse().expect("");

        data[a - 1].push(b - 1);
        data[b - 1].push(a - 1);
    }
    let mut color: Vec<usize> = Vec::new();
    color.resize(n + 1, 0);
    let mut clusters: Vec<Vec<usize>> = Vec::new();
    clusters.resize(n + 1, Vec::new());

    let mut found: usize = 0;
    for i in 0..n {
        if color[i] == 0 {
            found += 1;
            BFS(i, &data, &mut color, &mut clusters, found);
        }
    }

    println!("{found}");
    for i in 1..found + 1 {
        if !clusters[i].is_empty() {
            println!("{}", clusters[i].len());
            for j in 0..clusters[i].len() {
                print!("{} ", clusters[i][j] + 1);
            }
            println!();
        }
    }
}

fn BFS(
    v: usize,
    data: &Vec<Vec<usize>>,
    mut color: &mut Vec<usize>,
    mut clusters: &mut Vec<Vec<usize>>,
    paint: usize,
) {
    if color[v] != 0 {
        return;
    }

    color[v] = paint;
    clusters[paint].push(v);

    for i in 0..data[v].len() {
        BFS(data[v][i], &data, &mut color, &mut clusters, paint);
    }
}
