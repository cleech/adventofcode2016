const DATA: &'static str = include_str!("day04.txt");

use std::collections::HashMap;
use std::cmp::Ordering;

fn frequency(s: &str) -> HashMap<char, i32> {
    let mut h = HashMap::new();
    for ch in s.chars() {
        let counter = h.entry(ch).or_insert(0);
        *counter += 1;
    }
    h
}

pub fn main() {
    let count = DATA.lines()
        .map(|s| s.rsplitn(2,'-').collect::<Vec<_>>())
        // ["sector[csum]", "enc-name"]
        .map(|v| {
            let sc = v[0].split_terminator(|c| c == '[' || c == ']').collect::<Vec<_>>();
            let sector = sc[0];
            let csum = sc[1];
            (v[1], sector, csum)
        })
        // ("enc-name", "sector", "csum")
        .map(|t| {
            let mut hm = frequency(&t.0);
            hm.remove(&'-');
            let vec: Vec<(char, i32)> = hm.into_iter().collect();
            let mut vec = vec.into_iter().map(|(c,n)| (n,c)).collect::<Vec<_>>();
            vec.sort_by(|a, b| {
                let rc = b.0.cmp(&a.0);
                if rc == Ordering::Equal {
                    a.1.cmp(&b.1)
                } else {
                    rc
                }
            });
            let cs = vec.into_iter().map(|t| t.1).collect::<String>()[0 .. 5].to_string();
            (cs == t.2, t.0, t.1, t.2, cs)
        })
        // .inspect(|x| println!("{:?}", x))
        .filter(|t| t.0)
        .inspect(|t| {
            let sector = t.2.parse::<u32>().unwrap();
            let room = shift_name(&t.1, sector);
            println!("{} : {}", room, sector);
        })
        .map(|t| t.2.parse::<u32>().unwrap())
        .sum::<u32>();
        // .count();

    println!("{}", count);
}

fn shift_name(enc: &str, sector: u32) -> String {
    enc.bytes().map(|b| {
        let mut c = b;
        if c == b'-' {
            return ' ';
        }
        for _ in 0..sector {
            c = if c == b'z' { b'a' } else { c + 1 };
        }
        c as char
    }).collect()
}

#[cfg(test)]
mod test {
    #[test]
    fn examples() {}
}
