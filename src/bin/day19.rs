use std::u32;
use std::collections::VecDeque;
use std::iter::FromIterator;
extern crate num;
use num::integer::Integer;

#[allow(dead_code)]
fn part1 (count: u32) -> u32 {
        let mut a = (1..(count +1)).collect::<Vec<_>>();
        while a.len() > 1 {
            let b = a.chunks(2).map(|w| w[0]).collect::<Vec<_>>();
            if a.len() & 1 == 1 {
                a = b[1..].iter().cloned().collect();
            } else {
                a = b;
            }
        }
        a[0]
}

#[allow(dead_code)]
fn part1_queue(count: u32) -> u32 {
    let mut q = VecDeque::from_iter((1..count+1));
    while q.len() > 1 {
        let front = q.pop_front().unwrap();
        q.push_back(front);
        q.pop_front();
    }
    q[0]
}

#[allow(dead_code)]
pub fn part1_wtf(count: u32) -> u32 {
    let ffs = count.leading_zeros();
    (count << 1) ^ ((1 << (32 - ffs)) | 1)
}

#[allow(dead_code)]
fn part2_queue(count :usize) -> usize {
    let mut q1 = (1..count+1).collect::<VecDeque<_>>();
    let mut q2 = q1.split_off(count / 2 + 1);

    while !q1.is_empty() {
        q2.pop_front();
        if q1.len() == q2.len() {
            q1.push_back(q2.pop_front().unwrap());
        }
        q2.push_back(q1.pop_front().unwrap());
    }
    q2[0]
}

#[allow(dead_code)]
fn part2_1queue(count :usize) -> usize {
    let mut qt = (1..count+1).collect::<VecDeque<_>>();
    let mut q = qt.split_off(count / 2);
    q.extend(qt);

    while q.len() > 1 {
        q.pop_front();
        if q.len().is_even() {
            let tmp = q.pop_front().unwrap();
            q.push_back(tmp);
        }
    }
    q[0]
}

fn main () {
    println!("{}", part1_queue(3001330));
    // println!("{}", part2_queue(3001330));
    println!("{}", part2_1queue(3001330));
}
