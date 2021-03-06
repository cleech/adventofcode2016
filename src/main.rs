use std::io;
use std::time;
use std::process::Command;

#[macro_use]
extern crate scan_fmt;

extern crate clap;
use clap::{Arg, App};

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
            for day in 1..(25 + 1) {
                try!(run_one(day));
            }
        }
        1...25=> {
            try!(run_one(day));
        }
        _ => {}
    };
    Ok(())
}

fn run_one(day: u8) -> io::Result<Vec<String>> {
    let f = match day {
        0 => panic!("don't do that"),
        1...25 => {
            || {
                let _ = Command::new("cargo")
                    .args(&["run", "--release", "--bin", &format!("day{:02}", day)])
                    .status();
                vec![]
            }
        }
        _ => panic!("not there yet"),
    };

    let start = time::Instant::now();
    let results = f();
    let time = start.elapsed();
    println!("Day {}\t{:?}", day, time);
    //for output in &results {
    //    println!("  {}", output);
    //}
    Ok(results)
}

#[test]
fn verify_my_answers() {}
