use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let _: i32 = input.trim().parse().expect("");

    input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect(""))
        .collect();
    let mut ans = String::new();
    let swaps = heap(0, &mut numbers, &mut ans);
    println!("{swaps}");
    print!("{}", ans);
}

fn heap(root: usize, vec: &mut Vec<i32>, ans: &mut String) -> i32 {
    if root >= vec.len() {
        return 0;
    }
    let mut swaps = 0;
    swaps += heap(root * 2 + 1, vec, ans);
    swaps += heap(root * 2 + 2, vec, ans);

    if root * 2 + 1 >= vec.len() {
        return swaps;
    } else if root * 2 + 2 >= vec.len() {
        if vec[root] > vec[root * 2 + 1] {
            vec[root] += vec[root * 2 + 1];
            vec[root * 2 + 1] = vec[root] - vec[root * 2 + 1];
            vec[root] -= vec[root * 2 + 1];
            swaps += 1;
            //println!("{root} {}", root * 2 + 1);
            ans.insert_str(ans.len(), &format!("{} {}\n", root, root * 2 + 1));
            swaps += heap(root * 2 + 1, vec, ans);
        }
        return swaps;
    } else if vec[root] < vec[root * 2 + 1] && vec[root] < vec[root * 2 + 2] {
        return swaps;
    }
    if vec[root * 2 + 1] < vec[root * 2 + 2] {
        vec[root] += vec[root * 2 + 1];
        vec[root * 2 + 1] = vec[root] - vec[root * 2 + 1];
        vec[root] -= vec[root * 2 + 1];
        swaps += 1;
        //println!("{root} {}", root * 2 + 1);
        ans.insert_str(ans.len(), &format!("{} {}\n", root, root * 2 + 1));
        swaps += heap(root * 2 + 1, vec, ans);
    } else {
        vec[root] += vec[root * 2 + 2];
        vec[root * 2 + 2] = vec[root] - vec[root * 2 + 2];
        vec[root] -= vec[root * 2 + 2];
        swaps += 1;
        //println!("{root} {}", root * 2 + 2);
        ans.insert_str(ans.len(), &format!("{} {}\n", root, root * 2 + 2));
        swaps += heap(root * 2 + 2, vec, ans);
    }
    return swaps;
}
