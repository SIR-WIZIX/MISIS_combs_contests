use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut line = input.trim().split_whitespace();
    let n: usize = line.next().unwrap().parse().expect("");
    let m: usize = line.next().unwrap().parse().expect("");

    let mut data: Vec<Vec<i32>> = Vec::new();
    data.resize(n + 1, Vec::new());

    let mut colors: Vec<i32> = Vec::new();
    colors.resize(n + 1, 0);

    for _ in 0..m {
        input = String::new();
        io::stdin().read_line(&mut input).expect("");

        let mut line = input.trim().split_whitespace();
        let a: i32 = line.next().unwrap().parse().expect("");
        let b: i32 = line.next().unwrap().parse().expect("");
        data[a as usize].push(b);
        data[b as usize].push(a);
    }

    let mut possible = true;
    for i in 1..n + 1 {
        if colors[i] == 0 {
            possible = possible && DFS(i, &mut colors, &data, 1);
        }
    }
    if possible {
        let first_table = (1..=n)
            .filter(|&i| colors[i] == 1)
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        println!("YES");
        println!("{first_table}");
    } else {
        println!("NO");
    }
    println!();
}

fn DFS(v: usize, mut colors: &mut Vec<i32>, numbers: &Vec<Vec<i32>>, color: i32) -> bool {
    colors[v] = color;
    let mut possible = true;
    for next in 0..numbers[v].len() {
        if colors[numbers[v][next] as usize] == 0 {
            possible = possible && DFS(numbers[v][next] as usize, &mut colors, &numbers, -color);
        } else if colors[numbers[v][next] as usize] == color {
            possible = false;
            return possible;
        }
    }
    possible
}
