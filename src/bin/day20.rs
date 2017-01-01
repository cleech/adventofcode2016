static DATA: &'static str = include_str!("day20.txt");

fn main () {
    let mut pairs = DATA.lines()
        .map(|l| {
            let mut i = l.split('-');
            (i.next().unwrap(), i.next().unwrap())
        })
        .map(|(l,h)| (l.parse::<u32>().unwrap(), h.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();
    pairs.sort();

    let mut lowest: u32 = 0;
    for &(l,h) in &pairs {
        if lowest >= l && lowest <= h {
            // println!("{:?} covers {} => new lowest might be {}", (l,h), lowest, h+1);
            lowest = h + 1;
        }
    }
    println!("{}", lowest);

    let mut uncovered: Vec<(u32,u32)> = vec![(0,!0)];
    for &(l,h) in &pairs {
        uncovered = uncovered.into_iter().flat_map(|(a,b)| {
            if h < a || l > b {
                vec![(a,b)]
            } else if l <= a && h >= b {
                vec![]
            } else if l <= a && h >= a && h < b {
                vec![(h+1,b)]
            } else if l > a && l <= b && h >= b {
                vec![(a,l-1)]
            } else {
                vec![(a,l-1), (h+1, b)]
            }
        }).collect();
    }
    let count: u32 = uncovered.iter().map(|&(a,b)| b - a + 1).sum();
    println!("{:?}", count);
}
