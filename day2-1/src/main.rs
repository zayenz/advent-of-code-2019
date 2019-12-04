#![allow(dead_code, unused_imports)]

use failure::bail;
use failure::Error;
use rayon::prelude::*;
use strum_macros::EnumString;

use std::char;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::BufRead;
use std::ops::*;
use std::str;
use std::str::FromStr;
use std::{io, process};

use aoc2019::input::get_numbers;

type Input = Vec<i32>;
type Output = i32;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let mut numbers = get_numbers::<i32>(&line)?;
        result.append(&mut numbers);
    }

    Ok(result)
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
struct OpArgs {
    in_addr1: i32,
    in_addr2: i32,
    out_addr: i32,
}

impl OpArgs {
    fn triple_from(store: &[i32], pc: usize) -> OpArgs {
        OpArgs {
            in_addr1: store[pc],
            in_addr2: store[pc + 1],
            out_addr: store[pc + 2],
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
enum Op {
    Add(OpArgs),
    Multiply(OpArgs),
    Halt,
}

impl Op {
    fn decode(store: &[i32], pc: usize) -> Result<Op, Error> {
        match store[pc] {
            1 => Ok(Op::Add(OpArgs::triple_from(store, pc + 1))),
            2 => Ok(Op::Multiply(OpArgs::triple_from(store, pc + 1))),
            99 => Ok(Op::Halt),
            code => bail!("Unknown op code {} at {}", code, pc),
        }
    }

    fn run(&self, store: &mut [i32], pc: usize) -> Status {
        match *self {
            Op::Add(OpArgs {
                in_addr1,
                in_addr2,
                out_addr,
            }) => {
                let in1 = store[in_addr1 as usize];
                let in2 = store[in_addr2 as usize];
                let result = in1 + in2;
                store[out_addr as usize] = result;
                //                eprintln!("Added {} and {} to get {}, read from {:?}", in1, in2, result, (in_addr1, in_addr2, out_addr));
                Status::Continue(pc + 4)
            }
            Op::Multiply(OpArgs {
                in_addr1,
                in_addr2,
                out_addr,
            }) => {
                let in1 = store[in_addr1 as usize];
                let in2 = store[in_addr2 as usize];
                let result = in1 * in2;
                store[out_addr as usize] = result;
                Status::Continue(pc + 4)
            }
            Op::Halt => Status::Halt,
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
enum Status {
    Continue(usize),
    Halt,
}

fn debug_print(store: &[i32], pc: usize) {
    for (pos, value) in store.iter().enumerate() {
        if pos == pc {
            print!(">");
        }
        print!("{} ", value);
    }
    println!();
}

fn solve(store: &mut Input) -> Result<Output, Error> {
    let mut pc = 0;
    //    debug_print(store, pc);
    loop {
        let op = Op::decode(store, pc)?;
        match op.run(store, pc) {
            Status::Continue(next_pc) => {
                pc = next_pc;
            }
            Status::Halt => {
                break;
            }
        }
        //        debug_print(store, pc);
    }
    //    debug_print(store, pc);
    Ok(store[0])
}

fn run() -> Result<(), Error> {
    let mut input = read_input()?;

    let output = solve(&mut input)?;

    println!("{}", output);
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => process::exit(0),
        Err(error) => {
            eprintln!("Error while solving problem: {}", error);
            for cause in error.iter_causes() {
                eprintln!("{}", cause)
            }
            process::exit(1)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn sample1() {
        let mut store = vec![1, 0, 0, 0, 99];
        let result = solve(&mut store).unwrap();
        assert_eq!(result, 2);
    }
}
