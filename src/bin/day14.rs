extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

extern crate arrayvec;
use arrayvec::ArrayVec;

use std::io::Write;
use std::collections::HashMap;

// const SALT: &'static [u8] = b"abc";
const SALT: &'static [u8] = b"qzyelonm";

struct HashCache {
    md5: Md5,
    map: HashMap<usize, String>,
    rounds: usize,
}

impl HashCache {
    fn new(rounds: usize) -> HashCache {
        HashCache {
            md5: Md5::new(),
            map: HashMap::new(),
            rounds: rounds,
        }
    }

    fn entry(&mut self, n: usize) -> String {
        let mut buf = ArrayVec::<[_; 32]>::new();
        let map = &mut self.map;
        let md5 = &mut self.md5;
        let rounds = self.rounds;
        map.entry(n)
            .or_insert_with(|| {
                md5.reset();
                md5.input(SALT);
                write!(buf, "{}", n).unwrap();
                md5.input(&buf);
                for _ in 0..rounds {                    
                    let s = md5.result_str();
                    md5.reset();
                    md5.input_str(&s);
                }
                md5.result_str()
            })
            .to_owned()
    }
}

fn find_triplet(s: &str) -> Option<u8> {
    s.as_bytes()
        .windows(3)
        .find(|w| w[0] == w[1] && w[0] == w[2])
        .map(|w| w[0])
}

fn has_quintuplet(s: &str, c: u8) -> bool {
    s.as_bytes()
        .windows(5)
        .any(|w| w.iter().all(|&b| b == c))
}

fn find_key(hc: &mut HashCache, t: usize) -> usize {
    (0..)
        .filter(|&n| {
            let s = hc.entry(n);
            if let Some(c) = find_triplet(&s) {
                return (n + 1..n + 1001).any(|m| has_quintuplet(&hc.entry(m), c));
            }
            false
        })
        .nth(t)
        .unwrap()
}

fn main() {
    let mut hc = HashCache::new(0);
    let part1 = find_key(&mut hc, 64 - 1);
    println!("{}", part1);

    let mut hc = HashCache::new(2016);
    let part2 = find_key(&mut hc, 64 - 1);
    println!("{}", part2);
}
