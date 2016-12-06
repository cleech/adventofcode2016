const DATA: &'static str = include_str!("input.txt");
const LEN: usize = 8;

use std::collections::HashMap;
use std::char;

pub fn main() -> Vec<String> {
    let mut hashvec = vec![];
    for _ in 0..LEN {
        hashvec.push(HashMap::new());
    }
    for line in DATA.lines() {
        let b = line.as_bytes();
        for i in 0..LEN {
            let mut counter = hashvec[i].entry(char::from_u32(b[i] as u32).unwrap()).or_insert(0);
            *counter +=1;
        }
    }
    let mut s1 = String::new();
    let mut s2 = String::new();
    for n in 0..LEN {
        let mut v = hashvec[n].iter().collect::<Vec<_>>();
        v.sort_by(|a, b| b.1.cmp(a.1));
        s1.push(*v[0].0);
        s2.push(*v[v.len() - 1].0);
    }
    println!("{}", s1);
    println!("{}", s2);
    vec![]
}

#[cfg(test)]
mod test {
    #[test]
    fn examples() {}
}
