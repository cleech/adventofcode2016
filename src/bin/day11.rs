#[macro_use]
extern crate lazy_static;
use std::fmt;
use std::fmt::Display;
use std::collections::VecDeque;
use std::collections::HashSet;
extern crate arrayvec;
use arrayvec::ArrayVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Stuff(usize, usize, char);

#[derive(Clone)]
struct WorldState {
    moves: usize,
    elevator: usize,
    stuff: ArrayVec<[Stuff; 8]>,
}

impl Display for WorldState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "Elevator on {}", self.elevator + 1);
        let _ = writeln!(f, "has taken {} moves", self.moves);
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
            let _ = writeln!(f, "F{} {:?} {:?}", floor + 1, m, g);
        }
        Ok(())
    }
}

impl WorldState {
    fn new() -> Self {
        WorldState {
            moves: 0,
            elevator: 0,
            stuff: ArrayVec::new(),
        }
    }

    fn key(&self) -> ArrayVec<[usize; 16]> {
        let mut key = ArrayVec::new();
        key.push(self.elevator);
        for &Stuff(m, g, _) in self.stuff.iter() {
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

    fn adjacent_floors(&self) -> ArrayVec<[usize; 2]> {
        let mut av = ArrayVec::new();
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

    fn solve(&self, mut w: WorldState) -> WorldState {
        let mut q = VecDeque::new();
        let mut seen = HashSet::new();
        seen.insert(w.key());
        while !w.complete(self.dest) {
            for mut w in w.valid_moves() {
                // make sure stuff is sorted before generating key
                w.stuff.sort();
                if !seen.contains(&w.key()) {
                    seen.insert(w.key());
                    q.push_back(w);
                }
            }
            w = q.pop_front().unwrap();            
        }
        w
    }
}

#[allow(dead_code)]
fn load_test(world: &mut WorldState) {
    world.stuff.clear();
    world.stuff.push(Stuff(0, 1, 'H'));
    world.stuff.push(Stuff(0, 2, 'L'));
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
    world.stuff.sort();
}

#[allow(dead_code)]
fn load_part_two(world: &mut WorldState) {
    load_part_one(world);
    world.stuff.insert(0, Stuff(0, 0, 'D'));
    world.stuff.insert(1, Stuff(0, 0, 'E'));
    world.stuff.sort();
}

fn main() {
    let mut world = WorldState::new();
    load_part_one(&mut world);
    let part1 = Solver::new(3);
    world = part1.solve(world);
    println!("part 1 {}", world);

    let mut world = WorldState::new();
    load_part_two(&mut world);
    let part2 = Solver::new(3);
    world = part2.solve(world);
    println!("part 2 {}", world);
}
