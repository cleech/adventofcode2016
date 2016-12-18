#![feature(slice_patterns)]
use std::str::FromStr;

static DATA: &'static str = include_str!("day12.txt");

pub fn main() {
    let mut instructions = DATA.parse::<Program>().unwrap();
    instructions.optimize();

    let mut machine = Machine::new();
    machine.run(&instructions);
    let s1 = machine.regs[A].to_string();
    println!("{}", s1);

    let mut machine = Machine { regs: [0, 0, 1, 0], ..Machine::new() };
    machine.run(&instructions);
    let s2 = machine.regs[A].to_string();
    println!("{}", s2);
}

#[derive(Clone,Copy,Debug,PartialEq)]
enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}
use Register::*;

impl std::ops::Index<Register> for [isize] {
    type Output = isize;
    fn index(&self, idx: Register) -> &isize {
        &self[idx as usize]
    }
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Register, ()> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            "c" => Ok(Register::C),
            "d" => Ok(Register::D),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum Instr {
    Cpy(Register, Register),
    CpyI(isize, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Register, isize),
    JnzI(isize, isize),
    Add(Register, Register),
    NoOp,
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Instr, ()> {
        match s.split_whitespace().collect::<Vec<_>>()[..] {
            ["inc", r] => r.parse::<Register>().map(Instr::Inc),
            ["dec", r] => r.parse::<Register>().map(Instr::Dec),
            ["cpy", x, y] => {
                x.parse::<Register>()
                    .and_then(|x| {
                        y.parse::<Register>()
                            .map(|y| Instr::Cpy(x, y))
                    })
                    .or_else(|_| {
                        x.parse::<isize>()
                            .map_err(|_| ())
                            .and_then(|i| {
                                y.parse::<Register>()
                                    .map(|y| Instr::CpyI(i, y))
                            })
                    })
            }
            ["jnz", x, y] => {
                x.parse::<Register>()
                    .and_then(|x| {
                        y.parse::<isize>()
                            .map_err(|_| ())
                            .map(|y| Instr::Jnz(x, y))
                    })
                    .or_else(|_| {
                        x.parse::<isize>()
                            .map_err(|_| ())
                            .and_then(|i| {
                                y.parse::<isize>()
                                    .map_err(|_| ())
                                    .map(|y| Instr::JnzI(i, y))
                            })
                    })
            }
            _ => Err(()),
        }
    }
}

struct Program(Vec<Instr>);

impl std::ops::Deref for Program {
    type Target = Vec<Instr>;

    fn deref(&self) -> &Self::Target {
        let &Program(ref iv) = self;
        iv
    }
}

impl std::ops::DerefMut for Program {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let &mut Program(ref mut iv) = self;
        iv
    }
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Program, ()> {
        s.lines()
            .map(|l| l.parse::<Instr>())
            .collect::<Result<Vec<_>, _>>()
            .map(Program)
    }
}

impl Program {
    fn optimize(&mut self) {
        for i in 0..(self.len() - 2) {
            // replace inc,dec,jnz-2 loops with new Add instruction
            if let [Instr::Inc(x), Instr::Dec(y), Instr::Jnz(j, o)] = self[i..i + 3] {
                if x != y && y == j && o == -2 {
                    self[i] = Instr::Add(y, x);
                    self[i + 1] = Instr::CpyI(0, y);
                    self[i + 2] = Instr::NoOp;
                }
            }
        }
    }
}

#[derive(Debug)]
struct Machine {
    pc: isize,
    regs: [isize; 4],
}

impl Machine {
    fn new() -> Machine {
        Machine {
            pc: 0,
            regs: [0; 4],
        }
    }

    // apply a function to a register
    fn rmap<F, T>(&mut self, r: Register, f: F) -> T
        where F: Fn(&mut isize) -> T
    {
        match r {
            Register::A => f(&mut self.regs[0]),
            Register::B => f(&mut self.regs[1]),
            Register::C => f(&mut self.regs[2]),
            Register::D => f(&mut self.regs[3]),
        }
    }

    fn run(&mut self, instr: &[Instr]) {
        while let Some(i) = instr.get(self.pc as usize) {
            match *i {
                Instr::Cpy(x, y) => {
                    let tmp = self.rmap(x, |a| *a);
                    self.rmap(y, |b| *b = tmp);
                }
                Instr::CpyI(i, y) => self.rmap(y, |y| *y = i),
                Instr::Inc(r) => self.rmap(r, |r| *r += 1),
                Instr::Dec(r) => self.rmap(r, |r| *r -= 1),
                Instr::Jnz(r, o) => {
                    if self.rmap(r, |r| *r != 0) {
                        self.pc += o - 1;
                    }
                }
                Instr::JnzI(x, o) => {
                    if x != 0 {
                        self.pc += o - 1;
                    }
                }
                Instr::Add(x, y) => {
                    let tmp = self.rmap(x, |a| *a);
                    self.rmap(y, |b| *b += tmp);
                }
                Instr::NoOp => {}
            }
            self.pc += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Machine, parse_program};

    #[test]
    fn examples() {
        let source = ["cpy 41 a", "inc a", "inc a", "dec a", "jnz a 2", "dec a"].join("\n");
        let instructions = parse_program(&source).unwrap();
        println!("{:?}", instructions);
        let mut machine = Machine::new();
        println!("{:?}", machine);
        machine.run(&instructions);
        assert_eq!(machine.a, 42);
    }
}
