use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut line = input.trim().split_whitespace();
    let n: usize = line.next().unwrap().parse().expect("");

    let mut table: Vec<(usize, usize)> = Vec::new();

    for _ in 0..n - 1 {
        input = String::new();
        io::stdin().read_line(&mut input).expect("");

        let mut line = input.trim().split_whitespace();
        let a: usize = line.next().unwrap().parse().expect("");
        let b: usize = line.next().unwrap().parse().expect("");
        table.push((a, b));
    }

    regen_sequence(&table, n as i32)
        .iter()
        .for_each(|x| print!("{x} "));
}

fn regen_sequence(table: &Vec<(usize, usize)>, n: i32) -> Vec<usize> {
    let mut parent: Vec<i32> = vec![0; n as usize + 5];
    let mut size: Vec<i32> = vec![1; n as usize + 5];
    let mut tail: Vec<i32> = vec![0; n as usize + 5];
    let mut next: Vec<i32> = vec![0; n as usize + 5];

    for i in 1..=n as usize {
        parent[i] = i as i32;
        tail[i] = i as i32;
    }

    fn find(x: i32, parent: &Vec<i32>) -> i32 {
        if parent[x as usize] == x {
            return x;
        }
        return find(parent[x as usize], parent);
    }

    fn unite(
        mut a: i32,
        mut b: i32,
        size: &mut Vec<i32>,
        tail: &mut Vec<i32>,
        parent: &mut Vec<i32>,
        next: &mut Vec<i32>,
    ) {
        a = find(a, parent);
        b = find(b, parent);

        if size[a as usize] < size[b as usize] {
            (a, b) = (b, a);
        }
        next[tail[a as usize] as usize] = b;
        tail[a as usize] = tail[b as usize];

        parent[b as usize] = a;
        size[a as usize] += size[b as usize];
    }

    for i in table {
        unite(
            i.0 as i32,
            i.1 as i32,
            &mut size,
            &mut tail,
            &mut parent,
            &mut next,
        );
    }

    let mut answer: Vec<usize> = Vec::new();
    let mut cur = find(1, &parent);
    while cur != 0 {
        answer.push(cur as usize);
        cur = next[cur as usize];
    }

    answer
}
