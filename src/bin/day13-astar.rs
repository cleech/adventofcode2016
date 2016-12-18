use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

const KEY: isize = 1364;

fn f(x: isize, y: isize) -> bool {
    let z = x * x + 3 * x + 2 * x * y + y + y * y + KEY;
    z.count_ones() & 1 == 0
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Position {
    x: isize,
    y: isize,
    steps: isize,
    cost: isize,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Position) -> Ordering {
        // flip the ordering so we get a min-heap queue
        other.cost.cmp(&self.cost)
    }
}

impl Position {
    fn neighbors(&self) -> Vec<Position> {
        let &Position { x, y, steps, .. } = self;
        [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
            .iter()
            .filter(|&&(x, y)| x >= 0 && y >= 0 && f(x, y))
            .map(|&(x, y)| {
                Position {
                    x: x,
                    y: y,
                    steps: steps + 1,
                    cost: /*steps + 1 + */(31 - x).abs() + (39 - y).abs(),
                }
            })
            .collect()
    }
}

fn moves(loc: (isize, isize)) -> Vec<(isize, isize)> {
    let (x, y) = loc;
    [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
        .iter()
        .filter(|&&(x, y)| x >= 0 && y >= 0 && f(x, y))
        .cloned()
        .collect()
}

fn part1() {
    let loc = Position {
        x: 1,
        y: 1,
        steps: 0,
        cost: 30 + 38,
    };
    let mut pq = BinaryHeap::new();
    let mut seen = HashSet::new();

    seen.insert((loc.x, loc.y));
    pq.push(loc);

    let mut lc = 0;
    while let Some(loc) = pq.pop() {
        lc += 1;
        if let Position { x: 31, y: 39, .. } = loc {
            println!("{:?}", loc);
            break;
        }
        for m in loc.neighbors() {
            if !seen.contains(&(m.x, m.y)) {
                seen.insert((m.x, m.y));
                pq.push(m);
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
