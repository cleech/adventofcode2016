#![feature(slice_patterns)]

#[macro_use]
extern crate lazy_static;

extern crate arrayvec;
use arrayvec::ArrayVec;

extern crate fnv;
use fnv::FnvHashMap;

use std::cmp;
use std::str::FromStr;
use std::sync::Mutex;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

static DATA: &'static str = include_str!("day10.txt");

#[derive(Debug, Copy, Clone)]
enum Sink {
    Bot(u32),
    Bin(u32),
}

#[derive(Debug)]
struct Bot {
    id: u32,
    chips: ArrayVec<[u32; 2]>,
    low: Sink,
    high: Sink,
}

impl Bot {
    fn push(&mut self, v: u32) {
        self.chips.push(v);
        if self.chips.len() == 2 {
            let v1 = cmp::min(self.chips[0], self.chips[1]);
            let v2 = cmp::max(self.chips[0], self.chips[1]);
            self.chips.clear();
            let queue = CHAN.0.lock().unwrap();
            let _ = queue.send(Msg::Cmp {
                bot: self.id,
                low: v1,
                high: v2,
            });
            let _ = queue.send(Msg::Move(v1, self.low));
            let _ = queue.send(Msg::Move(v2, self.high));
        }
    }
}

#[derive(Debug)]
enum ParseElem {
    Source { value: u32, bot: u32 },
    Bot(Bot),
}

impl FromStr for ParseElem {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<ParseElem, Self::Err> {
        let ss = s.split_whitespace().collect::<Vec<_>>();
        match ss[..] {
            ["value", v, "goes", "to", "bot", b] => {
                let source = v.parse::<u32>().unwrap();
                let bot = b.parse::<u32>().unwrap();
                Ok(ParseElem::Source {
                    value: source,
                    bot: bot,
                })
            }
            ["bot", b, "gives", "low", "to", d1, d1id, "and", "high", "to", d2, d2id] => {
                let bot = b.parse::<u32>().unwrap();
                let d1 = match d1 {
                    "bot" => Sink::Bot(d1id.parse::<u32>().unwrap()),
                    "output" => Sink::Bin(d1id.parse::<u32>().unwrap()),
                    _ => panic!(),
                };
                let d2 = match d2 {
                    "bot" => Sink::Bot(d2id.parse::<u32>().unwrap()),
                    "output" => Sink::Bin(d2id.parse::<u32>().unwrap()),
                    _ => panic!(),
                };
                Ok(ParseElem::Bot(Bot {
                    id: bot,
                    chips: ArrayVec::default(),
                    low: d1,
                    high: d2,
                }))
            }
            _ => panic!("{:?}", ss),
        }
    }
}

enum Msg {
    // send chips to other bots or output bins
    Move(u32, Sink),
    // trace comparisons for part 1
    Cmp { bot: u32, low: u32, high: u32 },
}

lazy_static! {
    static ref CHAN: (Mutex<Sender<Msg>>, Mutex<Receiver<Msg>>) = {
        let (tx, rx) = mpsc::channel();
        (Mutex::new(tx), Mutex::new(rx))
    };
}

fn main() {
    let mut bots: FnvHashMap<u32, Bot> = FnvHashMap::default();
    let mut bins: FnvHashMap<u32, u32> = FnvHashMap::default();

    for l in DATA.lines() {
        let tx = CHAN.0.lock().unwrap();
        match l.parse::<ParseElem>() {
            Ok(ParseElem::Source { value, bot }) => {
                let _ = tx.send(Msg::Move(value, Sink::Bot(bot)));
            }
            Ok(ParseElem::Bot(b @ Bot { .. })) => {
                bots.insert(b.id, b);
            }
            Err(_) => panic!(),
        };
    }

    let rx = CHAN.1.lock().unwrap();
    while let Ok(m) = rx.try_recv() {
        match m {
            Msg::Cmp { bot: b, low: v1, high: v2 } => {
                if v1 == 17 && v2 == 61 {
                    println!("part1: bot {}", b);
                }
            }
            Msg::Move(v, Sink::Bot(b)) => {
                if let Some(be @ &mut Bot { .. }) = bots.get_mut(&b) {
                    be.push(v);
                } else {
                    panic!();
                }
            }
            Msg::Move(v, Sink::Bin(o)) => {
                bins.insert(o, v);
            }            
        }
    }
    println!("part2: {:?}", bins[&0] * bins[&1] * bins[&2]);
}
