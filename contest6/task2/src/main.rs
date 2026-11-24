use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut line = input.trim().split_whitespace();
    let n: usize = line.next().unwrap().parse().expect("");
    let m: usize = line.next().unwrap().parse().expect("");

    let mut table: Vec<(i32, i32, i32)> = Vec::new();
    table.resize(m, (0, 0, 0));

    for i in 0..m {
        input = String::new();
        io::stdin().read_line(&mut input).expect("");

        let mut line = input.trim().split_whitespace();
        let a: i32 = line.next().unwrap().parse().expect("");
        let b: i32 = line.next().unwrap().parse().expect("");
        let c: i32 = line.next().unwrap().parse().expect("");
        table[i] = (a - 1, b - 1, c);
    }

    table.sort_unstable_by(|a, b| a.2.cmp(&b.2));

    println!("{}", min_ostov(n, &table));
}

fn min_ostov(n: usize, table: &Vec<(i32, i32, i32)>) -> i32 {
    let mut weight = 0;

    let mut components: Vec<usize> = Vec::new();
    components.resize(n, 0);
    for i in 0..components.len() {
        components[i] = i;
    }

    for i in table {
        if components[i.0 as usize] != components[i.1 as usize] {
            weight += i.2;
            let new_comp = components[i.0 as usize];
            let old_comp = components[i.1 as usize];
            components.iter_mut().for_each(|x| {
                if *x == old_comp {
                    *x = new_comp;
                }
            });
        }
    }

    weight
}
