use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use std::iter::FromIterator;
use std::collections::BinaryHeap;
use std::collections::HashSet;

static DATA: &'static str = include_str!("day22.txt");

#[derive(Debug, PartialEq, Clone)]
struct Node {
    x: usize,
    y: usize,
    space: usize,
    used: usize,
    avail: usize,
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace().collect::<Vec<_>>();
        let pos = words[0].split('-').collect::<Vec<_>>();
        let x = pos[1][1..].parse::<usize>().unwrap();
        let y = pos[2][1..].parse::<usize>().unwrap();
        let space = words[1][..words[1].len() - 1].parse::<usize>().unwrap();
        let used = words[2][..words[2].len() - 1].parse::<usize>().unwrap();
        let avail = words[3][..words[3].len() - 1].parse::<usize>().unwrap();

        Ok(Node {
            x: x,
            y: y,
            space: space,
            used: used,
            avail: avail,
        })
    }
}

#[derive(Debug)]
struct Grid {
    x: usize,
    y: usize,
    nodes: Vec<Node>,
}

type Coord = (usize, usize);

impl Display for Grid {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for y in 0..self.y {
            let s = (0..self.x)
                .map(|x| match self[(x, y)] {
                    ref n if n.used == 0 => " _",
                    ref n if n.used > 100 => " #",
                    _ => " .",
                })
                .collect::<String>();
            writeln!(fmt, "{}", s)?
        }
        Ok(())
    }
}

impl FromIterator<Node> for Grid {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = Node>
    {
        let mut nodes = Vec::from_iter(iter);
        nodes.sort_by_key(|n| (n.y, n.x));
        let last_node = nodes[nodes.len() - 1].clone();
        let (x, y) = (last_node.x + 1, last_node.y + 1);
        Grid {
            x: x,
            y: y,
            nodes: nodes,
        }
    }
}

impl std::ops::Index<Coord> for Grid {
    type Output = Node;
    fn index(&self, idx: Coord) -> &Node {
        &self.nodes[idx.1 * self.x + idx.0]
    }
}

impl std::ops::IndexMut<Coord> for Grid {
    fn index_mut(&mut self, idx: Coord) -> &mut Node {
        &mut self.nodes[idx.1 * self.x + idx.0]
    }
}

impl Grid {
    fn count_viable_pairs(&self) -> usize {
        let mut count = 0;
        for a in &self.nodes {
            if a.used == 0 {
                continue;
            }
            for b in &self.nodes {
                if a == b {
                    continue;
                }
                if a.used <= b.avail {
                    count += 1;
                }
            }
        }
        count
    }

    #[allow(dead_code)]
    fn move_data(&mut self, from: Coord, to: Coord) -> usize {
        let data = self[from].used;
        if data == 0 {
            return 0;
        }
        if data > self[to].avail {
            return 0;
        }
        self[from].used = 0;
        self[from].avail += data;
        self[to].used += data;
        self[to].avail -= data;
        data
    }

    #[allow(dead_code)]
    fn move_gap(&mut self, from: Coord, to: Coord) -> Result<usize, ()> {
        let mut pq = BinaryHeap::new();
        let mut seen = HashSet::new();

        seen.insert(from);
        pq.push(from);

        Err(())
    }
}


fn main() {
    let grid: Grid = DATA.lines().skip(2).map(|s| s.parse::<Node>().unwrap()).collect();

    println!("{}", grid);
    println!("{}", grid.count_viable_pairs());
}
