extern crate num;
use num::Integer;

const SEED: &'static [u8] = b"10111100110001111";
const LEN_1: usize = 272;
const LEN_2: usize = 35_651_584;

fn expand(a: &mut Vec<u8>) {
    let mut b = Vec::with_capacity(a.len());
    b.extend(a.iter().rev().map(|x| 1 - x));
    // a.reserve(b.len() + 1);
    a.push(0);
    a.extend(b);
}

fn checksum(a: &[u8]) -> Vec<u8> {
    let cs = a.chunks(2)
        .map(|c| if c[0] == c[1] { 1 } else { 0 })
        .collect::<Vec<u8>>();
    if cs.len().is_even() {
        checksum(&cs)
    } else {
        cs
    }
}

fn main() {
    for &len in &[LEN_1, LEN_2] {
        let mut data = SEED.iter().map(|x| x - b'0').collect::<Vec<_>>();
        data.reserve(len * 2);
        while data.len() < len {
            expand(&mut data);
        }
        let mut cs = checksum(&data[..len]);
        for c in &mut cs {
            *c += b'0';
        }
        if let Ok(s) = std::str::from_utf8(&cs) {
            println!("{}", s);
        }
    }
}
