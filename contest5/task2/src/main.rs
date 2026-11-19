use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut line = input.trim().split_whitespace();
    let n: usize = line.next().unwrap().parse().expect("");
    let m: usize = line.next().unwrap().parse().expect("");
    let k: i32 = line.next().unwrap().parse().expect("");
    let s: usize = line.next().unwrap().parse().expect("");
    let f: usize = line.next().unwrap().parse().expect("");

    let mut races: Vec<(usize, usize, i32)> = Vec::new();
    races.resize(m, (0, 0, 0));

    let mut distances: Vec<i32> = Vec::new();
    distances.resize(n as usize, i32::MAX);

    for i in 0..m {
        input = String::new();
        io::stdin().read_line(&mut input).expect("");

        let mut line = input.trim().split_whitespace();
        let si: i32 = line.next().unwrap().parse().expect("");
        let fi: i32 = line.next().unwrap().parse().expect("");
        let pi: i32 = line.next().unwrap().parse().expect("");
        races[i] = ((si - 1) as usize, (fi - 1) as usize, pi);
    }

    ford_bellman(s - 1, races, &mut distances, k);

    if distances[f - 1] == i32::MAX {
        println!("-1");
    } else {
        println!("{}", distances[f - 1]);
    }
}

fn ford_bellman(s: usize, races: Vec<(usize, usize, i32)>, distances: &mut Vec<i32>, k: i32) {
    distances[s] = 0;

    for _ in 0..k {
        let mut next: Vec<i32> = distances.clone();

        races.iter().for_each(|race| {
            if distances[race.0] != i32::MAX {
                next[race.1] = std::cmp::min(next[race.1], distances[race.0] + race.2)
            }
        });

        distances.copy_from_slice(&next);
    }
}
