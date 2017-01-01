use std::collections::BTreeSet;
use std::cmp::max;

static DATA: &'static str = include_str!("day20.txt");

fn main() {
    let blocked = DATA.lines()
        .map(|l| {
            let mut i = l.split('-');
            (i.next().and_then(|s| s.parse::<u32>().ok()).unwrap(),
             i.next().and_then(|s| s.parse::<u32>().ok()).unwrap())
        })
        .collect::<BTreeSet<_>>();

    // lowest uncovered
    let mut lowest = 0;
    // highest covered
    let mut highest = 0;
    // count of uncovered
    let mut count = 0;

    for &(a, b) in &blocked {
        if a <= lowest && lowest <= b {
            // covered
            lowest = b + 1; // possible overflow if everything is blocked ...
        }
        if highest < a {
            // uncovered range detected
            // this conditional enters on the adjoining case [_, h][a, b] as well
            // but will result in count += 0
            // tracking to either side to avoid can result in overflow/underflow
            count += a - highest - 1;
        }
        highest = max(highest, b);
    }
    println!("{}", lowest);
    println!("{}", count);
}
