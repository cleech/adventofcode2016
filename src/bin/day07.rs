use std::str;
extern crate itertools;
use itertools::Itertools;
extern crate fnv;
use fnv::FnvHashSet;

static DATA: &'static str = include_str!("day07.txt");

#[inline]
fn aba(w: &[u8]) -> bool {
    w[0] == w[2] && w[0] != w[1]
}

#[inline]
fn abba(w: &[u8]) -> bool {
    w[0] == w[3] && w[1] == w[2] && w[0] != w[1]
}

pub fn main() {
    let part1 = DATA.lines()
        .filter(|l| {
            let mut supernet = l.split(|c| c == '[' || c == ']').step(2);
            let mut hypernet = l.split(|c| c == '[' || c == ']').skip(1).step(2);
            supernet.any(|sn| sn.as_bytes().windows(4).any(abba)) &&
            hypernet.all(|hn| hn.as_bytes().windows(4).all(|w| !abba(w)))
        })
        .count();
    println!("{}", part1);

    let part2 = DATA.lines()
        .filter(|l| {
            let supernet = l.split(|c| c == '[' || c == ']').step(2);
            let mut hypernet = l.split(|c| c == '[' || c == ']').skip(1).step(2);

            let abas: FnvHashSet<&[u8]> = supernet.flat_map(|sn| {
                sn.as_bytes().windows(3).filter(|&w| aba(w))
            }).collect();

            hypernet.any(|hn| {
                hn.as_bytes().windows(3).any(|w| {
                    let bab: &[u8] = &[w[1], w[0], w[1]];
                    aba(w) && abas.contains(&bab)
                })
            })
        })
        .count();
    println!("{}", part2);
}
