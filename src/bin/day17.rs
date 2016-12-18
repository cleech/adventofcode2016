use std::collections::VecDeque;

extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

fn possible_moves((x, y, key): (isize, isize, String)) -> Vec<(isize, isize, String)> {
    let mut md5 = Md5::new();
    md5.input_str(&key);
    let hash = md5.result_str();
    let h = hash.as_bytes();

    [(x, y - 1, h[0], 'U'), (x, y + 1, h[1], 'D'), (x - 1, y, h[2], 'L'), (x + 1, y, h[3], 'R')]
        .iter()
        .filter(|&&(a, b, c, _)| (a >= 0 && a < 4) && (b >= 0 && b < 4) && (c > b'a' && c <= b'f'))
        .map(|&(a, b, _, d)| (a, b, format!("{}{}", key, d)))
        .collect()
}

// static KEY: &'static str = "ihgpwlah";
// static KEY: &'static str = "kglvqrro";
// static KEY: &'static str = "ulqzkmiv";
static KEY: &'static str = "pxxbnzuo";

fn main() {
    let start = (0, 0, KEY.to_string());
    let mut q = VecDeque::new();
    let mut longest = None;
    let mut shortest = None;

    q.push_back(start);
    while let Some(loc) = q.pop_front() {
        if let (3, 3, _) = loc {
            shortest = shortest.or(Some(loc.clone()));
            longest = Some(loc.clone());
            continue;
        }
        for m in possible_moves(loc) {
            q.push_back(m);
        }
    }
    if let Some(short) = shortest {
        println!("shortest: {:?}", &short.2[8..]);
    }
    if let Some(long) = longest {
        println!("longest: {:?}", long.2[8..].len());
    }
}
