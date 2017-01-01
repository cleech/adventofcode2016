use std::collections::VecDeque;
use std::collections::HashSet;

static DATA: &'static str = include_str!("day24.txt");

struct Map {
    x: usize,
    _y: usize,
    open: Vec<bool>,
}

fn main() {
    let mut targets: Vec<(usize, usize)> = vec![(0,0); 8];

    let mut map: Map = Map {
        x: 179,
        _y: 43,
        open: vec![],
    };

    for (y, l) in DATA.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '.' => map.open.push(true),
                '#' => map.open.push(false),
                _ if c.is_numeric() => {
                    targets[(c as usize) - ('0' as usize)] = (x, y);
                    map.open.push(true);
                }
                _ => panic!(),
            }
        }
    }

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();

    let tmp = [false; 8];
    // tmp[0] = true;  // uncomment for part 1
    q.push_back((targets[0], tmp, 0));
    visited.insert((targets[0], tmp));

    while let Some(loc) = q.pop_front() {
        if loc.1 == [true; 8] {
            println!("visited all in {} steps", loc.2);
            break;
        }
        for pos in &[((loc.0).0 + 1, (loc.0).1),
                     ((loc.0).0 - 1, (loc.0).1),
                     ((loc.0).0, (loc.0).1 + 1),
                     ((loc.0).0, (loc.0).1 - 1)] {
            if map.open[pos.1 * map.x + pos.0] && !visited.contains(&(*pos, loc.1)) {
                let mut tmp = loc.1;
                if let Some(tidx) = targets.iter().enumerate().find(|&(_, &t)| t == (pos.0, pos.1)) {
                    if tidx.0 == 0 && tmp[1..] == [true; 7] {
                        tmp[tidx.0] = true;
                    }
                    if tidx.0 != 0 {
                        tmp[tidx.0] = true;
                    }
                }
                q.push_back((*pos, tmp, loc.2 + 1));
                visited.insert((*pos, tmp));
            }
        }
    }
}
