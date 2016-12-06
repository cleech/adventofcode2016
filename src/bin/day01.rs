const DATA: &'static str = include_str!("day01.txt");

extern crate nalgebra;
use self::nalgebra::{Vector2, Matrix2};
extern crate itertools;
use self::itertools::unfold;
use std::collections::HashSet;

// Left and Right rotation matrices

const L: Matrix2<i32> = Matrix2 {
    m11: 0,
    m12: -1,
    m21: 1,
    m22: 0,
};

const R: Matrix2<i32> = Matrix2 {
    m11: 0,
    m12: 1,
    m21: -1,
    m22: 0,
};

pub fn main() {
    let mkitr = || {
        DATA.trim()
            .split(", ")
            .scan((Vector2::new(0, 1), Vector2::new(0, 0)), |state, s| {
                let (rot, dist) = match s.split_at(1) {
                    ("R", d) => (R, d.parse::<usize>().unwrap()),
                    ("L", d) => (L, d.parse::<usize>().unwrap()),
                    (_, _) => panic!(),
                };
                let oldp = state.1;
                let f = rot * state.0;
                let p = state.1 + f * (dist as i32);
                *state = (f, p);
                Some(unfold((f, oldp), |state| {
                        let (f, mut p) = *state;
                        p += f;
                        *state = (f, p);
                        Some(p)
                    })
                    .take(dist))
            })
            .flat_map(|i| i)
    };

    let ebhq = mkitr().last().unwrap();
    let part1 = ebhq.x.abs() + ebhq.y.abs();

    let mut visited = HashSet::new();
    let ebhq = mkitr().find(|&p| !visited.insert(p)).unwrap();
    let part2 = ebhq.x.abs() + ebhq.y.abs();

    println!("{}", part1.to_string());
    println!("{}", part2.to_string());
}

#[cfg(test)]
mod test {
    #[test]
    fn examples() {}
}
