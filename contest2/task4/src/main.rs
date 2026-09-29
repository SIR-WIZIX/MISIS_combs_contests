use std::{collections::HashMap, io};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let NM: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please type a number!"))
        .collect();
    let n = NM[0];
    let m = NM[1];

    let mut data: HashMap<i32, (i32, i32)> = HashMap::new();

    for i in 0..n {
        data.insert(i + 1, (0, 0));
    }

    for _i in 0..m {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("");

        let SD: Vec<_> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Please type a number!"))
            .collect();

        let m = data.get_mut(&SD[0]);
        m.expect("").0 += 1;
        let m = data.get_mut(&SD[1]);
        m.expect("").1 += 1;
    }

    let mut out: Vec<_> = data.iter().collect();
    out.sort_by_key(|&(key, _)| key);
    for (_k, v) in out {
        println!("{}\n{}", v.1, v.0);
    }
    println!();
}
