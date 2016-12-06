const DATA: &'static str = include_str!("day03.txt");

#[macro_use]
extern crate scan_fmt;

fn is_possible(mut sides: [u32; 3]) -> bool {
    sides.sort();
    (sides[0] + sides[1]) > sides[2]
}

pub fn main() {
    let count = DATA.lines()
        .filter(|line| {
            if let (Some(a), Some(b), Some(c)) = scan_fmt!(line, "{d} {d} {d}", u32, u32, u32) {
                is_possible([a, b, c])
            } else {
                panic!("bad input")
            }
        })
        .count();
    println!("{}", count);

    let count2 = DATA.lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|ss| ss.join(" "))
        .flat_map(|ss| {
            if let (Some(a1), Some(b1), Some(c1),
                    Some(a2), Some(b2), Some(c2),
                    Some(a3), Some(b3), Some(c3)) = 
                    scan_fmt!(&ss, "{d} {d} {d} {d} {d} {d} {d} {d} {d}",
                                          u32, u32, u32,
                                          u32, u32, u32,
                                          u32, u32, u32) {
                vec![[a1, a2, a3], [b1, b2, b3], [c1, c2, c3]]
            } else {
                panic!("bad input")
            }
        })
        .filter(|t| is_possible(*t))
        .count();
    println!("{}", count2);
}

#[cfg(test)]
mod test {
    #[test]
    fn examples() {}
}
