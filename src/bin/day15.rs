#![feature(step_by)]

extern crate num;
use num::Integer;

struct Disc {
    idx: usize,
    size: usize,
    start: usize,
}

impl Disc {
    fn pos(&self, time: usize) -> usize {
        (self.start + time) % self.size
    }
}

fn main() {
    let _test = vec![Disc { idx: 1, size: 5, start: 4, },
                     Disc { idx: 2, size: 2, start: 1, }];
    let part1 = vec![Disc { idx: 1, size: 13, start: 1, },
                     Disc { idx: 2, size: 19, start: 10, },
                     Disc { idx: 3, size: 3, start: 2, },
                     Disc { idx: 4, size: 7, start: 1, },
                     Disc { idx: 5, size: 5, start: 3, },
                     Disc { idx: 6, size: 17, start: 5, }];
    let part2 = vec![Disc { idx: 7, size: 11, start: 0, }];

    fn sync_discs((t0, step): (usize, usize), disc: &Disc) -> (usize, usize) {
        for t in (t0..).step_by(step) {
            if disc.pos(t + disc.idx) == 0 {
                return (t, step.lcm(&disc.size));
            }
        }
        panic!()
    };

    let t = part1.iter().fold((0, 1), sync_discs);
    println!("{}", t.0);
    let t = part2.iter().fold(t, sync_discs);
    println!("{}", t.0);
}
