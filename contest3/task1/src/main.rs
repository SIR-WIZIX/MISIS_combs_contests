use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let n: usize = input.trim().parse().expect("");

    let mut numbers: Vec<Vec<i32>> = Vec::new();
    numbers.resize(n as usize, Vec::new());

    let mut color: Vec<i32> = Vec::new();
    color.resize(n as usize, 0);

    for i in 0..n {
        input = String::new();
        io::stdin().read_line(&mut input).expect("");
        numbers[i as usize] = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Please type a number!"))
            .collect();
    }

    let mut ans: Vec<usize>;
    let mut found;
    for v in 0..n {
        if color[v] == 0 {
            (ans, found) = DFS(v, &mut color, &numbers, 1);
            if found != 0 {
                println!("YES\n{}", found + 1);
                for i in 0..found + 1 {
                    print!("{} ", ans[i] + 1);
                }
                println!();
                return;
            }
        }
    }
    println!("NO");
}

fn DFS(
    v: usize,
    mut colors: &mut Vec<i32>,
    numbers: &Vec<Vec<i32>>,
    c: i32,
) -> (Vec<usize>, usize) {
    colors[v] = c;
    let mut ans = Vec::new();
    let mut found;
    for next in 0..colors.len() {
        if numbers[v][next] == 0 {
            continue;
        }
        if c - colors[next] >= 2 && colors[next] > 0 {
            ans.push(next);
            ans.push(v);
            colors[v] = -1;
            return (ans, (c - colors[next]) as usize);
        } else if colors[next] == 0 {
            (ans, found) = DFS(next, &mut colors, &numbers, c + 1);
            if found != 0 {
                ans.push(v);
                colors[v] = -1;
                return (ans, found);
            }
        }
    }
    (Vec::new(), 0)
}
