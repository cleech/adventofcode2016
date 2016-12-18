#[macro_use]
extern crate scan_fmt;

use std::str::FromStr;
use std::fmt;

static DATA: &'static str = include_str!("day08.txt");
const WIDTH: usize = 50;
const HEIGHT: usize = 6;

#[derive(Debug)]
enum Cmnd {
    Rect { x: usize, y: usize },
    RRow { y: usize, n: usize },
    RCol { x: usize, n: usize },
}
use Cmnd::*;

impl FromStr for Cmnd {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Cmnd, &'static str> {
        let mut words = line.split_whitespace();

        match words.next() {
            Some("rect") => {
                if let(Some(x), Some(y)) = scan_fmt!(&words.next().unwrap(), "{d}x{d}", usize, usize) {
                    Ok(Rect {x: x, y: y})
                } else {
                    Err("invalid input")
                }
            },
            Some("rotate") => {
                match words.next() {
                    Some("column") => {
                        if let(Some(x), Some(n)) = scan_fmt!(&words.collect::<String>(), "x={d} by {d}", usize, usize) {
                            Ok(RCol {x: x, n: n})
                        } else {
                            Err("invalid input")
                        }
                    },
                    Some("row") => {
                        if let(Some(y), Some(n)) = scan_fmt!(&words.collect::<String>(), "y={d} by {d}", usize, usize) {
                            Ok(RRow {y: y, n: n})
                        } else {
                            Err("invalid input")
                        }
                    },
                    _ => Err("invalid input")
                }
            },
            _ => Err("invalid input")
        }
    }
}

#[derive(Clone)]
struct Screen ([[bool; WIDTH]; HEIGHT]);

impl Screen {
    fn rect(&mut self, x: usize, y: usize) {
        for iy in 0..y {
            for ix in 0..x {
                self.0[iy][ix] = true;
            }
        }
    }
    fn rrow(&mut self, y: usize, n: usize) {
        let mut tmp = self.0[y];
        for i in 0..WIDTH {
            tmp[i] = self.0[y][(i + WIDTH - n) % WIDTH];
        }
        self.0[y] = tmp;
    }
    fn rcol(&mut self, x: usize, n: usize) {
        let mut tmp = [false; HEIGHT];
        for i in 0..HEIGHT {
            tmp[i] = self.0[i][x];
        }
        for i in 0..HEIGHT {
            self.0[i][x] = tmp[(i + HEIGHT - n) % HEIGHT];
        }
    }
    fn cmnd(&mut self, cmnd: Cmnd) {
        match cmnd {
            Rect {x, y} => self.rect(x, y),
            RRow {y, n} => self.rrow(y, n),
            RCol {x, n} => self.rcol(x, n),
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().map(|row| {
            writeln!(f, "{}",
                row.iter().map(|b| match *b {
                    true => '\u{2588}',
                    false => '.',
                }).collect::<String>()
            )
        }).last().unwrap()
    }
}

fn main () {
    let mut s = Screen([[false; WIDTH]; HEIGHT]);
    for cmnd in DATA.lines().map(Cmnd::from_str) {
        // println!("{:?}", cmnd);
        s.cmnd(cmnd.unwrap());
        // println!("{}", s);
    }
    let count = s.0.iter().flat_map(|row| row.iter()).filter(|cel| **cel).count();
    println!("{}", count);
    println!("{}", s);
}
