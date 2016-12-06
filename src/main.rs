// #![feature(box_syntax)]
// #![feature(slice_patterns)]
// #![feature(advanced_slice_patterns)]
// #![feature(associated_consts)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

// #![feature(conservative_impl_trait)]

use std::io;
use std::time;

#[macro_use]
extern crate scan_fmt;

extern crate clap;
use clap::{Arg, App};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

const LATEST: u8 = 5;

fn main() {
    let args = App::new("AdventOfCode")
        .arg(Arg::with_name("DAY")
            .short("d")
            .long("day")
            .takes_value(true))
        .get_matches();

    let day = args.value_of("DAY")
        .unwrap_or("0")
        .parse::<u8>()
        .expect("Invalid value for day");

    try_main(day).unwrap()
}

fn try_main(day: u8) -> io::Result<()> {
    match day {
        0 => {
            for day in 1..(LATEST + 1) {
                try!(run_one(day));
            }
        }
        1...LATEST => {
            try!(run_one(day));
        }
        _ => {}
    };
    Ok(())
}

fn run_one(day: u8) -> io::Result<Vec<String>> {
    let f: fn() -> Vec<String> = match day {
        0 => panic!("don't do that"),
        1 => day01::main,
        2 => day02::main,
        3 => day03::main,
        4 => day04::main,
        5 => day05::main,
        _ => panic!("not there yet"),
    };

    let start = time::Instant::now();
    let results = f();
    let time = start.elapsed();
    println!("Day {}\t{:?}", day, time);
    for output in &results {
        println!("  {}", output);
    }
    Ok(results)
}

#[test]
fn verify_my_answers() {
    assert_eq!(run_one(1).unwrap(), ["287", "133"]);
}
