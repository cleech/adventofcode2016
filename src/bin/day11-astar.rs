#[macro_use]
extern crate lazy_static;
use std::fmt;
use std::fmt::Display;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Stuff(usize, usize, char);

#[derive(Clone, Eq, PartialEq)]
struct WorldState {
    moves: usize,
    elevator: usize,
    stuff: Vec<Stuff>,
    cost: usize,
}

impl Display for WorldState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Elevator on {}", self.elevator + 1)?;
        writeln!(f, "has taken {} moves", self.moves)?;
        writeln!(f, "cost est is {}", self.cost)?;
        for floor in (0..4).rev() {
            let m = self.stuff
                .iter()
                .filter(|&&Stuff(m, _, _)| m == floor)
                .map(|&Stuff(_, _, c)| format!("{}M", c))
                .collect::<Vec<_>>();
            let g = self.stuff
                .iter()
                .filter(|&&Stuff(_, g, _)| g == floor)
                .map(|&Stuff(_, _, c)| format!("{}G", c))
                .collect::<Vec<_>>();
            writeln!(f, "F{} {:?} {:?}", floor + 1, m, g)?;
        }
        Ok(())
    }
}

impl Ord for WorldState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for WorldState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn cost_estimate(stuff: &[Stuff]) -> usize {
    stuff.iter()
        .map(|&Stuff(m, g, _)| if m == g {
            (3 - m)
        } else {
            3 * (3 - m) + 3 * (3 - g)
        })
        .sum()
}

impl WorldState {
    fn new() -> Self {
        WorldState {
            moves: 0,
            elevator: 0,
            stuff: Vec::with_capacity(8),
            cost: 0,
        }
    }

    fn key(&self) -> Vec<usize> {
        let mut key = Vec::with_capacity(16);
        key.push(self.elevator);
        for &Stuff(m, g, _) in &self.stuff {
            key.push(m);
            key.push(g);
        }
        key
    }

    fn ok(&self) -> bool {
        (0..4).all(|floor| {
            let mut m = self.stuff
                .iter()
                .filter(|&&Stuff(m, _, _)| m == floor)
                .filter(|&&Stuff(m, g, _)| m != g);
            let mut g = self.stuff
                .iter()
                .filter(|&&Stuff(_, g, _)| g == floor);
            m.next() == None || g.next() == None
        })
    }

    fn adjacent_floors(&self) -> Vec<usize> {
        let mut av = Vec::with_capacity(2);
        match self.elevator {
            0 => {
                av.push(1);
            }
            1 => {
                av.push(0);
                av.push(2);
            }
            2 => {
                av.push(1);
                av.push(3);
            }
            3 => {
                av.push(2);
            }
            _ => panic!(),
        }
        av
    }

    fn all_moves(&self, from: usize, to: usize) -> Vec<WorldState> {
        // fn all_moves<'a>(&'a self, from: usize, to: usize)
        // -> Box<Iterator<Item=WorldState> + 'a> {
        let m = self.stuff
            .iter()
            .enumerate()
            .filter(|&(_, &Stuff(m, _, _))| m == from)
            .map(|(i, &Stuff(_, g, c))| {
                let mut w = self.clone();
                w.stuff[i] = Stuff(to, g, c);
                // postpone sort until we know it's OK
                // w.stuff.sort();
                w.elevator = to;
                w.moves += 1;
                w.cost = w.moves + cost_estimate(&w.stuff);
                w
            });
        let g = self.stuff
            .iter()
            .enumerate()
            .filter(|&(_, &Stuff(_, g, _))| g == from)
            .map(|(i, &Stuff(m, _, c))| {
                let mut w = self.clone();
                w.stuff[i] = Stuff(m, to, c);
                // postpone sort until we know it's OK
                // w.stuff.sort();
                w.elevator = to;
                w.moves += 1;
                w.cost = w.moves + cost_estimate(&w.stuff);
                w
            });
        m.chain(g).collect()
        // Box::new(m.chain(g))
    }

    fn valid_moves(&self) -> Vec<WorldState> {
        let ones = self.adjacent_floors()
            .into_iter()
            .flat_map(|floor| {
                self.all_moves(self.elevator, floor)
                    .into_iter()
                    .filter(|w| w.ok())
            });
        let twos = self.adjacent_floors()
            .into_iter()
            .flat_map(|floor| {
                self.all_moves(self.elevator, floor)
                    .into_iter()
                    .flat_map(move |w| w.all_moves(self.elevator, floor))
                    .map(|mut w| {
                        w.moves -= 1;
                        w.cost = w.moves + cost_estimate(&w.stuff);
                        w
                    })
                    .filter(|w| w.ok())
            });
        ones.chain(twos).collect()
    }

    fn complete(&self, d: usize) -> bool {
        self.stuff.iter().all(|&Stuff(m, g, _)| m == d && g == d)
    }
}

struct Solver {
    dest: usize,
}

impl Solver {
    fn new(d: usize) -> Self {
        Solver { dest: d }
    }

    fn solve(&self, w: WorldState) -> Option<WorldState> {
        let mut pq = BinaryHeap::new();
        let mut seen = HashMap::new();

        seen.insert(w.key(), w.clone());
        pq.push(w);

        while let Some(w) = pq.pop() {
            if w.complete(self.dest) {
                return Some(w);
            }
            for mut w in w.valid_moves() {
                // make sure stuff is sorted before generating key
                w.stuff.sort();
                match seen.entry(w.key()) {
                    Entry::Occupied(w2) => {
                        let w2 = w2.into_mut();
                        if w2.moves > w.moves {
                            w2.moves = w.moves;
                            pq.push(w);
                        }
                    }
                    Entry::Vacant(w2) => {
                        w2.insert(w.clone());
                        pq.push(w);
                    }
                }
            }
        }
        None
    }
}

#[allow(dead_code)]
fn load_test(world: &mut WorldState) {
    world.stuff.clear();
    world.stuff.push(Stuff(0, 1, 'H'));
    world.stuff.push(Stuff(0, 2, 'L'));
    world.cost = cost_estimate(&world.stuff);
    world.stuff.sort();
}

#[allow(dead_code)]
fn load_part_one(world: &mut WorldState) {
    world.stuff.clear();
    world.stuff.push(Stuff(0, 0, 'P'));
    world.stuff.push(Stuff(0, 0, 'S'));
    world.stuff.push(Stuff(1, 1, 'C'));
    world.stuff.push(Stuff(1, 1, 'R'));
    world.stuff.push(Stuff(2, 1, 'T'));
    world.cost = cost_estimate(&world.stuff);
    world.stuff.sort();
}

#[allow(dead_code)]
fn load_part_two(world: &mut WorldState) {
    load_part_one(world);
    world.stuff.insert(0, Stuff(0, 0, 'D'));
    world.stuff.insert(1, Stuff(0, 0, 'E'));
    world.cost = cost_estimate(&world.stuff);
    world.stuff.sort();
}

fn main() {
    let mut world = WorldState::new();
    load_part_one(&mut world);
    let part1 = Solver::new(3);
    if let Some(world) = part1.solve(world) {
        println!("part 1 {}", world);
    }

    let mut world = WorldState::new();
    load_part_two(&mut world);
    let part2 = Solver::new(3);
    if let Some(world) = part2.solve(world) {
        println!("part 2 {}", world);
    }
}
