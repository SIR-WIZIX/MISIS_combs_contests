use std::{collections::HashMap, io, iter::Map, ptr::null};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut n: i32 = input.trim().parse().expect("");

    input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please type a number!"))
        .collect();

    let mut data: HashMap<i32, i32> = HashMap::new();
    for j in numbers {
        let m = data.get_mut(&j);
        if m.is_some() {
            *m.expect("") += 1;
        } else {
            data.insert(j, 1);
        }
    }

    let mut out: Vec<_> = data.iter().collect();
    out.sort_by_key(|&(key, _)| key);
    for (k, v) in out {
        for _ in 0..*v {
            print!("{} ", k);
        }
    }
    println!();
}
