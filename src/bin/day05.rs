#![feature(step_by)]

use std::io;
use std::io::Write;
use std::thread;
use std::sync::{Arc, Mutex};

extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

extern crate itertools;
use itertools::unfold;

extern crate rustc_serialize;
pub use rustc_serialize::hex::*;

extern crate arrayvec;
use arrayvec::ArrayVec;

extern crate num_cpus;

const DATA: &'static str = include_str!("day05.txt");

pub fn main() {
    let input = DATA.trim();

    let code = unfold(0, |mut idx| {
            Some(find_interesting_hash(input, &mut idx, 1))
        })
        .map(|(_, hash)| hash.chars().skip(5).next().unwrap())
        .take(8)
        .collect::<String>();

    println!("day05 part1: {}", code);

    print!("day05 part2: --------");
    io::stdout().flush().unwrap();

    let code2 = Arc::new(Mutex::new((vec![u32::max_value(); 8], vec![b'-'; 8])));
    let cpus = num_cpus::get();
    let mut threads = Vec::with_capacity(cpus);

    for cpu in 0..cpus {
        let code2 = code2.clone();

        let tid = thread::spawn(move || {
            let mut hashes = unfold(cpu as u32, |mut idx| {
                Some(find_interesting_hash(input, &mut idx, cpus as u32))
            });

            loop {
                let (idx, hash) = hashes.next().unwrap();
                let mut ch = hash.bytes().skip(5);
                let n = ch.next().unwrap().to_digit_dumb().unwrap() as usize;
                let b = ch.next().unwrap();

                let mut code2 = code2.lock().unwrap();
                // locked section from here to end of loop
                if n < 8 && idx < code2.0[n] {
                    code2.1[n] = b;
                    code2.0[n] = idx;
                    print!("\rday05 part2: {}", unsafe { String::from_utf8_unchecked(code2.1.clone()) });
                    io::stdout().flush().unwrap();
                }
                if code2.0.iter().all(|n| *n <= idx) {
                    break;
                }
            }
        });
        threads.push(tid);
    }
    for tid in threads { let _ = tid.join(); }

    let code2 = code2.lock().unwrap();
    println!("\rday05 part2: {}", unsafe { String::from_utf8_unchecked(code2.1.clone()) });
}

fn leading_zeros(buf: &[u8]) -> bool {
    buf[0] | buf[1] | (buf[2] >> 4) == 0
}

trait DumbConvert {
    fn to_digit_dumb(&self) -> Option<u8>;
}
impl DumbConvert for u8 {
    fn to_digit_dumb(&self) -> Option<u8> {
        match *self {
            b'0'...b'9' => Some(*self - b'0'),
            b'a'...b'f' => Some(*self - b'a' + 10),
            _ => None,
        }
    }
}

fn find_interesting_hash(key: &str, start: &mut u32, stride: u32) -> (u32, String) {
    let mut md5 = Md5::new();
    let mut buf = ArrayVec::<[_; 64]>::new();
    let mut output = [0u8; 16];

    *start = (*start..).step_by(stride)
        .find(|n| {
            md5.reset();
            md5.input(key.as_bytes());
            buf.clear();
            write!(&mut buf, "{}", n).unwrap();
            md5.input(&buf);
            md5.result(&mut output);
            leading_zeros(&output)
        })
        .unwrap();

    let res = (*start, output.to_hex());
    *start += stride;
    res
}
