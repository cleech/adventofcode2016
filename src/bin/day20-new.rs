use std::collections::BTreeSet;

static DATA: &'static str = include_str!("day20.txt");

fn main() {
    let pairs = DATA.lines()
        .map(|l| {
            let mut i = l.split('-');
            (i.next().and_then(|n| n.parse::<u32>().ok()).unwrap(),
             i.next().and_then(|n| n.parse::<u32>().ok()).unwrap())
        })
        .collect::<BTreeSet<_>>();

    let mut uncovered: BTreeSet<(u32, u32)> = BTreeSet::new();
    uncovered.insert((0, !0));

    for (l, h) in pairs {
        uncovered = uncovered.into_iter()
            .flat_map(|(a, b)| {
                if h < a || l > b {
                    vec![(a, b)]
                } else if l <= a && h >= b {
                    vec![]
                } else if l <= a && h >= a && h < b {
                    vec![(h + 1, b)]
                } else if l > a && l <= b && h >= b {
                    vec![(a, l - 1)]
                } else {
                    vec![(a, l - 1), (h + 1, b)]
                }
            })
            .collect();
    }
    println!("{}", uncovered.iter().next().unwrap().0);
    let count: u32 = uncovered.iter().map(|&(a, b)| b - a + 1).sum();
    println!("{}", count);
}
