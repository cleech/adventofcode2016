#![feature(slice_patterns)]
use std::str::FromStr;

static DATA: &'static str = include_str!("day21.txt");

enum Op {
    SwapP(usize, usize),
    SwapL(u8, u8),
    Rev(usize, usize),
    RotL(usize),
    RotR(usize),
    RotP(u8),
    Mov(usize, usize),
}
use Op::*;

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace().collect::<Vec<_>>();
        match words[..] {
            ["swap", "position", a, "with", "position", b] => {
                let x = a.parse::<usize>().unwrap();
                let y = b.parse::<usize>().unwrap();
                Ok(SwapP(x, y))
            }
            ["swap", "letter", a, "with", "letter", b] => {
                let x = a.as_bytes()[0];
                let y = b.as_bytes()[0];
                Ok(SwapL(x, y))
            }
            ["reverse", "positions", a, "through", b] => {
                let x = a.parse::<usize>().unwrap();
                let y = b.parse::<usize>().unwrap();
                Ok(Rev(x, y))
            }
            ["rotate", "left", a, "steps"] |
            ["rotate", "left", a, "step"] => {
                let x = a.parse::<usize>().unwrap();
                Ok(RotL(x))
            }
            ["rotate", "right", a, "steps"] |
            ["rotate", "right", a, "step"] => {
                let x = a.parse::<usize>().unwrap();
                Ok(RotR(x))
            }
            ["rotate", "based", "on", "position", "of", "letter", a] => {
                let x = a.as_bytes()[0];
                Ok(RotP(x))
            }
            ["move", "position", a, "to", "position", b] => {
                let x = a.parse::<usize>().unwrap();
                let y = b.parse::<usize>().unwrap();
                Ok(Mov(x, y))
            }
            _ => Err(()),
        }
    }
}

fn scramble(password: &mut Vec<u8>, instr: &[Op]) {
    for i in instr {
        match *i {
            SwapP(x, y) => {
                password.swap(x, y);
            }
            SwapL(x, y) => {
                let i = password.iter().position(|&c| c == x).unwrap();
                let j = password.iter().position(|&c| c == y).unwrap();
                password.swap(i, j);
            }
            Rev(x, y) => {
                password[x..y + 1].reverse();
            }
            RotL(x) => {
                let y = x % password.len();
                if y > 0 {
                    password[..y].reverse();
                    password[y..].reverse();
                    password[..].reverse();
                }
            }
            RotR(x) => {
                let y = password.len() - (x % password.len());
                if y > 0 {
                    password[..y].reverse();
                    password[y..].reverse();
                    password[..].reverse();
                }
            }
            RotP(x) => {
                let i = password.iter().position(|&c| c == x).unwrap();
                let j = {
                    if i <= 3 {
                        password.len() - ((i + 1) % password.len())
                    } else {
                        password.len() - ((i + 2) % password.len())
                    }
                };
                if j > 0 {
                    password[..j].reverse();
                    password[j..].reverse();
                    password[..].reverse();
                }
            }
            Mov(x, y) => {
                let c = password.remove(x);
                password.insert(y, c);
            }
        }
    }
}

fn reverse(password: &mut Vec<u8>, instr: &[Op]) {
    for i in instr.iter().rev() {
        match *i {
            SwapP(x, y) => {
                password.swap(x, y);
            }
            SwapL(x, y) => {
                let i = password.iter().position(|&c| c == x).unwrap();
                let j = password.iter().position(|&c| c == y).unwrap();
                password.swap(i, j);
            }
            Rev(x, y) => {
                password[x..y + 1].reverse();
            }
            RotL(x) => {
                let y = password.len() - (x % password.len());
                if y > 0 {
                    password[..y].reverse();
                    password[y..].reverse();
                    password[..].reverse();
                }
            }
            RotR(x) => {
                let y = x % password.len();
                if y > 0 {
                    password[..y].reverse();
                    password[y..].reverse();
                    password[..].reverse();
                }
            }
            RotP(x) => {
                // this only works for passwords of size 8
                assert!(password.len() == 8);
                let i = password.iter().position(|&c| c == x).unwrap();
                let j = match i {
                    0 | 1 => 1,
                    2 => 6,
                    3 => 2,
                    4 => 7,
                    5 => 3,
                    6 => 0,
                    7 => 4,
                    _ => panic!(),
                };
                if j > 0 {
                    password[..j].reverse();
                    password[j..].reverse();
                    password[..].reverse();
                }
            }
            Mov(x, y) => {
                let c = password.remove(y);
                password.insert(x, c);
            }
        }
    }
}

fn main() {
    let instr = DATA.lines().map(|l| l.parse::<Op>().unwrap()).collect::<Vec<_>>();

    let mut password = b"abcdefgh".iter().cloned().collect::<Vec<_>>();
    scramble(&mut password, &instr);
    println!("{}",
             password.into_iter().map(|c| c as char).collect::<String>());

    let mut password = b"fbgdceah".iter().cloned().collect::<Vec<_>>();
    reverse(&mut password, &instr);
    println!("{}",
             password.into_iter().map(|c| c as char).collect::<String>());
}

// brute force solution for part 2 without needing the reverse function
extern crate permutohedron;
#[allow(dead_code)]
fn part2_brute(instr: &[Op]) {
    let mut password = b"abcdefgh".iter().cloned().collect::<Vec<_>>();
    let heap = permutohedron::Heap::new(&mut password);
    for password in heap {
        let mut pw = password.clone();
        let pt = pw.iter().map(|&c| c as char).collect::<String>();
        scramble(&mut pw, instr);
        let ct = pw.iter().map(|&c| c as char).collect::<String>();
        if ct == "fbgdceah" {
            println!("{}", pt);
            break;
        }
    }
}
