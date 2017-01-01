#![feature(slice_patterns)]
use std::str::FromStr;
use std::ops::{Index, IndexMut};
use std::ops::{Deref, DerefMut};

static DATA: &'static str = include_str!("day23.txt");

pub fn main() {
    let mut instructions = DATA.parse::<Program>().unwrap();
    instructions.optimize();

    let mut part1 = instructions.clone();
    let mut machine = Machine::new();
    machine.regs[A] = 7;
    machine.run(&mut part1);
    let s1 = machine.regs[A].to_string();
    println!("{}", s1);

    let mut part2 = instructions.clone();
    let mut machine = Machine::new();
    machine.regs[A] = 12;
    machine.run(&mut part2);
    let s1 = machine.regs[A].to_string();
    println!("{}", s1);
}

#[derive(Clone,Copy,Debug,PartialEq)]
enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}
use Register::*;

impl Index<Register> for [isize] {
    type Output = isize;
    fn index(&self, idx: Register) -> &isize {
        &self[idx as usize]
    }
}

impl IndexMut<Register> for [isize] {
    fn index_mut(&mut self, idx: Register) -> &mut isize {
        &mut self[idx as usize]
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

#[derive(Debug,Clone,Copy)]
enum AddrMode {
    Reg(Register),
    Imm(isize),
}
use AddrMode::*;

#[derive(Debug,Clone,Copy)]
enum Instr {
    Cpy(AddrMode, Register),
    Inc(Register),
    Dec(Register),
    Jnz(AddrMode, AddrMode),
    Tgl(AddrMode),
    // optimization ops
    NoOp,
    Add(Register, Register),
    Mul(Register, Register),
}
use Instr::*;

impl Instr {
    fn toggle(&mut self) {
        *self = match *self {
            Inc(r) | Tgl(Reg(r)) => Dec(r),
            Dec(r) => Inc(r),
            Jnz(a, Reg(r)) => Cpy(a, r),
            Cpy(a, r) => Jnz(a, Reg(r)),
            _ => panic!(),
        }
    }
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Instr, ()> {
        match s.split_whitespace().collect::<Vec<_>>()[..] {
            ["inc", r] => r.parse::<Register>().map(Inc),
            ["dec", r] => r.parse::<Register>().map(Dec),
            ["cpy", x, y] => {
                x.parse::<Register>()
                    .map(Reg)
                    .or_else(|_| {
                        x.parse::<isize>()
                            .map_err(|_| ())
                            .map(Imm)
                    })
                    .and_then(|x| {
                        y.parse::<Register>()
                            .map(|y| Cpy(x, y))
                    })
            }
            ["jnz", x, y] => {
                x.parse::<Register>()
                    .map(Reg)
                    .or_else(|_| {
                        x.parse::<isize>()
                            .map_err(|_| ())
                            .map(Imm)
                    })
                    .and_then(|x| {
                        y.parse::<Register>()
                            .map(Reg)
                            .or_else(|_| {
                                y.parse::<isize>()
                                    .map_err(|_| ())
                                    .map(Imm)
                            })
                            .map(|y| Jnz(x, y))
                    })
            }
            ["tgl", x] => {
                x.parse::<Register>()
                    .map(Reg)
                    .or_else(|_| {
                        x.parse::<isize>()
                            .map_err(|_| ())
                            .map(Imm)
                    })
                    .map(Tgl)
            }
            _ => Err(()),
        }
    }
}

struct Program(Vec<Instr>);

impl Program {
    fn optimize(&mut self) {
        for i in 0..(self.len() - 5) {
            // a += b * d (zeroing c and d)
            if let [Cpy(Reg(b), c),
                    Inc(a),
                    Dec(c_),
                    Jnz(Reg(c__), Imm(-2)),
                    Dec(d),
                    Jnz(Reg(d_), Imm(-5))]
                    = self[i..i + 6] {
                if c == c_ && c == c__  && d == d_ &&
                   a != b && a != c && a != d && b != c && b != d && c != d {
                    self[i] = Mul(b, d);
                    self[i + 1] = Add(d, a);
                    self[i + 2] = Cpy(Imm(0), c);
                    self[i + 3] = Cpy(Imm(0), d);
                    self[i + 4] = NoOp;
                    self[i + 5] = NoOp;
                }
            }
        }

        for i in 0..(self.len() - 2) {
            // a += b (zeroing b)
            if let [Inc(a), Dec(b), Jnz(Reg(b_), Imm(-2))] = self[i..i + 3] {
                if a != b && b == b_ {
                    self[i] = Add(b, a);
                    self[i + 1] = Cpy(Imm(0), b);
                    self[i + 2] = NoOp;
                }
            }
        }
    }
}

impl Deref for Program {
    type Target = Vec<Instr>;

    fn deref(&self) -> &Self::Target {
        let &Program(ref iv) = self;
        iv
    }
}

impl DerefMut for Program {
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

    fn run(&mut self, instr: &mut [Instr]) {
        while (self.pc as usize) < instr.len() {
            let i = instr[self.pc as usize];
            match i {
                Cpy(ref x, y) => {
                    let tmp = match *x {
                        Reg(r) => self.rmap(r, |r| *r),
                        Imm(i) => i,
                    };
                    self.rmap(y, |r| *r = tmp);
                }
                Inc(r) => self.rmap(r, |r| *r += 1),
                Dec(r) => self.rmap(r, |r| *r -= 1),
                Jnz(ref a, ref b) => {
                    let r = match *a {
                        Reg(r) => self.rmap(r, |r| *r),
                        Imm(i) => i,
                    };
                    let o = match *b {
                        Reg(r) => self.rmap(r, |r| *r),
                        Imm(i) => i,
                    };
                    if r != 0 {
                        self.pc += o - 1;
                    }
                }
                Tgl(ref x) => {
                    let tmp = match *x {
                        Reg(r) => self.rmap(r, |r| *r),
                        Imm(i) => i,
                    };
                    if let Some(i) = instr.get_mut((self.pc + tmp) as usize) {
                        i.toggle();
                    }
                }
                NoOp => {}
                Add(x, y) => {
                    let tmp = self.rmap(x, |a| *a);
                    self.rmap(y, |b| *b += tmp);
                }
                Mul(x, y) => {
                    let tmp = self.rmap(x, |a| *a);
                    self.rmap(y, |b| *b *= tmp);
                }
            }
            self.pc += 1;
        }
    }
}
