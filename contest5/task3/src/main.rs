use std::io;

fn main() {
    let mut heap = Heap::new();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut line = input.trim().split_whitespace();
    let n: usize = line.next().unwrap().parse().expect("");

    for _ in 0..n {
        input = String::new();
        io::stdin().read_line(&mut input).expect("");

        let mut line = input.trim().split_whitespace();
        let c: i32 = line.next().unwrap().parse().expect("");
        if c == 1 {
            println!("{}", heap.extract());
        } else {
            let item: i32 = line.next().unwrap().parse().expect("");
            heap.insert(item);
        }
    }
}

struct Heap {
    data: [Option<i32>; 1e5 as usize],
    len: usize,
}
impl Heap {
    pub fn new() -> Self {
        Heap {
            data: [None; 1e5 as usize],
            len: 0,
        }
    }

    pub fn insert(&mut self, item: i32) {
        self.len += 1;
        let mut cur = self.len;
        self.data[cur] = Some(item);

        while cur > 1 {
            let parent = cur / 2;
            if self.data[parent] >= self.data[cur] {
                break;
            }

            self.data.swap(parent, cur);
            cur = parent;
        }
    }

    pub fn extract(&mut self) -> i32 {
        self.data.swap(1, self.len);
        let ans: i32 = self.data[self.len].take().unwrap();
        self.len -= 1;

        let mut cur = 1;
        while cur * 2 <= self.len {
            let mut child = cur * 2;
            let right = child + 1;

            if right <= self.len && self.data[right] > self.data[child] {
                child = right;
            }

            if self.data[cur] >= self.data[child] {
                break;
            }

            self.data.swap(cur, child);
            cur = child;
        }

        ans
    }
}
