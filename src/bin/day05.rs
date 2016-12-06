use std::io;
use std::io::Write;

extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::md5::Md5;

extern crate itertools;
use self::itertools::unfold;

extern crate rustc_serialize;
pub use self::rustc_serialize::hex::*;

const DATA: &'static str = include_str!("day05.txt");

pub fn main() {
    let input = DATA.trim();

    let code = unfold(0, |mut idx| Some(find_interesting_hash(input, &mut idx, 5)))
        .map(|hash| hash.chars().skip(5).next().unwrap())
        .take(8)
        .collect::<String>();

    println!("day05 part1: {}", code);

    let mut hashes = unfold(0, |mut idx| Some(find_interesting_hash(input, &mut idx, 5)));
    let mut c2: Vec<u8> = vec![b'_'; 8];
    print!("day05 part2: {}", unsafe { String::from_utf8_unchecked(c2.clone()) });
    io::stdout().flush().unwrap();
    loop {
        let hash = hashes.next().unwrap();
        let mut ch = hash.chars().skip(5);
        let n = ch.next().unwrap().to_digit(16).unwrap() as usize;
        let b = ch.next().unwrap().to_digit(16).unwrap() as u8;

        if n < 8 && c2[n] == b'_' {
            c2[n] = [b].to_hex().bytes().skip(1).next().unwrap() as u8;
            print!("\rday05 part2: {}", unsafe { String::from_utf8_unchecked(c2.clone()) });
            io::stdout().flush().unwrap();

            if c2.iter().all(|b| *b != b'_') {
                break;
            }
        }
    }
    println!("\rday05 part2: {}", unsafe { String::from_utf8_unchecked(c2.clone()) });
}

fn leading_zeros(buf: &[u8], count: usize) -> bool {
    let (bytes, nibble) = (count / 2, count % 2);
    buf[..bytes].iter().all(|b| *b == 0) &&
    if nibble != 0 {
        (buf[bytes] & 0xf0) == 0
    } else {
        true
    }
}

fn find_interesting_hash(key: &str, start: &mut u32, zeros: usize) -> String {
    let mut md5 = Md5::new();
    let k = key.as_bytes();
    let mut buf = Vec::with_capacity(256);
    let mut output = [0u8; 16];

    *start = (*start..)
        .find(|n| {
            md5.reset();
            md5.input(k);
            buf.clear();
            write!(&mut buf, "{}", n).unwrap();
            md5.input(&buf);
            md5.result(&mut output);
            leading_zeros(&output, zeros)
        })
        .unwrap();
    // println!("\t{}", start);
    *start += 1;

    output.to_hex()
}

#[cfg(test)]
mod test {
    use super::find_interesting_hash;

    #[test]
    fn examples() {}
}
