use std::{cmp::Reverse, collections::BinaryHeap, io};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut line = input.trim().split_whitespace();
    let n: usize = line.next().unwrap().parse().expect("");
    let s: usize = line.next().unwrap().parse().expect("");
    let f: usize = line.next().unwrap().parse().expect("");

    let mut table: Vec<Vec<i32>> = Vec::new();
    table.resize(n as usize, Vec::new());

    let mut distances: Vec<i32> = Vec::new();
    distances.resize(n as usize, i32::MAX);

    for i in 0..n {
        input = String::new();
        io::stdin().read_line(&mut input).expect("");
        table[i] = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Please type a number!"))
            .collect();
    }

    djikstra(s - 1, &table, &mut distances);

    println!(
        "{}",
        if distances[f - 1] as i32 == i32::MAX {
            -1
        } else {
            distances[f - 1] as i32
        }
    );
}

fn djikstra(v1: usize, table: &Vec<Vec<i32>>, distances: &mut Vec<i32>) {
    let mut heap: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
    distances[v1] = 0;
    heap.push((std::cmp::Reverse(0), v1));
    while let Some((Reverse(dist), target)) = heap.pop() {
        if dist > distances[target] as i32 {
            continue;
        }

        for i in 0..table[target].len() {
            if table[target][i] != -1 {
                if dist + table[target][i] < distances[i] as i32 {
                    distances[i] = dist + table[target][i];
                    heap.push((Reverse(distances[i] as i32), i));
                }
            }
        }
    }
}
