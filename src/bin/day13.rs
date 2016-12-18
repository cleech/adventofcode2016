use std::collections::VecDeque;
use std::collections::HashSet;

const KEY: isize = 1364;

fn f(x: isize, y: isize) -> bool {
    let z = x * x + 3 * x + 2 * x * y + y + y * y + KEY;
    z.count_ones() & 1 == 0
}

fn moves(loc: (isize, isize)) -> Vec<(isize, isize)> {
    let (x, y) = loc;
    vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
        .into_iter()
        .filter(|&(x, y)| x >= 0 && y >= 0 && f(x, y))
        .collect()
}

fn part1() {
    let loc = ((1, 1), 0);
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();

    q.push_front(loc);
    seen.insert(loc.0);

    let mut lc = 0;
    while let Some(loc) = q.pop_front() {
        lc += 1;
        if loc.0 == (31, 39) {
            println!("{:?}", loc);
            break;
        }
        for m in moves(loc.0) {
            if !seen.contains(&m) {
                seen.insert(m);
                q.push_back((m, loc.1 + 1));
            }
        }
    }
    println!("loops {}", lc);
}

fn part2() {
    let mut loc = ((1, 1), 0);
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();

    while loc.1 < 50 {
        for m in moves(loc.0) {
            if !seen.contains(&m) {
                seen.insert(m);
                q.push_back((m, loc.1 + 1));
            }
        }
        loc = q.pop_front().unwrap();
    }
    println!("{:?}", seen.len())
}

fn main() {
    part1();
    part2();
}
