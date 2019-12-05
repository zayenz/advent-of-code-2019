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

use crate::ArgMode::{Immediate, Position};
use aoc2019::input::get_numbers;
use failure::_core::convert::TryFrom;
use std::convert::TryInto;

type Input = Vec<i32>;
type Output = ();

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
struct OpArgs1 {
    in1: i32,
}

impl OpArgs1 {
    fn single_from(store: &[i32], pc: usize) -> OpArgs1 {
        OpArgs1 { in1: store[pc] }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
struct OpArgs3 {
    in1: i32,
    in2: i32,
    out: i32,
}

impl OpArgs3 {
    fn triple_from(store: &[i32], pc: usize) -> OpArgs3 {
        OpArgs3 {
            in1: store[pc],
            in2: store[pc + 1],
            out: store[pc + 2],
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
enum Op {
    Add(OpArgs3, ArgModes),
    Multiply(OpArgs3, ArgModes),
    Input(OpArgs1, ArgModes),
    Output(OpArgs1, ArgModes),
    Halt,
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
enum ArgMode {
    Position,
    Immediate,
}

impl ArgMode {
    fn get(self, store: &[i32], arg: i32) -> i32 {
        match self {
            Position => store[arg as usize],
            Immediate => arg,
        }
    }
}

impl TryFrom<i32> for ArgMode {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Position),
            1 => Ok(Immediate),
            _ => bail!("Value {} is not vaild for ArgMode", value),
        }
    }
}

type ArgModes = (ArgMode, ArgMode, ArgMode);

impl Op {
    fn decode(store: &[i32], pc: usize) -> Result<Op, Error> {
        let opcode = store[pc] % 100;
        let arg_mode1: ArgMode = ((store[pc] / 100) % 10).try_into()?;
        let arg_mode2: ArgMode = ((store[pc] / 1000) % 10).try_into()?;
        let arg_mode3: ArgMode = ((store[pc] / 10000) % 10).try_into()?;
        let arg_modes = (arg_mode1, arg_mode2, arg_mode3);
        match opcode {
            1 => Ok(Op::Add(OpArgs3::triple_from(store, pc + 1), arg_modes)),
            2 => Ok(Op::Multiply(OpArgs3::triple_from(store, pc + 1), arg_modes)),
            3 => Ok(Op::Input(OpArgs1::single_from(store, pc + 1), arg_modes)),
            4 => Ok(Op::Output(OpArgs1::single_from(store, pc + 1), arg_modes)),
            99 => Ok(Op::Halt),
            code => bail!("Unknown op code {} at {}", code, pc),
        }
    }

    fn run(&self, store: &mut [i32], pc: usize, input: &mut Option<i32>) -> Status {
        match *self {
            Op::Add(OpArgs3 { in1, in2, out }, (am1, am2, _am3)) => {
                let in1 = am1.get(store, in1);
                let in2 = am2.get(store, in2);
                let result = in1 + in2;
                store[out as usize] = result;
                //                eprintln!("Added {} and {} to get {}, read from {:?}", in1, in2, result, (in_addr1, in_addr2, out_addr));
                Status::Continue(pc + 4)
            }
            Op::Multiply(OpArgs3 { in1, in2, out }, (am1, am2, _am3)) => {
                let in1 = am1.get(store, in1);
                let in2 = am2.get(store, in2);
                let result = in1 * in2;
                store[out as usize] = result;
                Status::Continue(pc + 4)
            }
            Op::Halt => Status::Halt,
            Op::Input(OpArgs1 { in1 }, (_am1, _, _)) => {
                let read_input = input.take().expect("No input to read");
                store[in1 as usize] = read_input;
                Status::Continue(pc + 2)
            }
            Op::Output(OpArgs1 { in1 }, (am1, _, _)) => {
                let in1 = am1.get(store, in1);
                print!("{} ", in1);
                Status::Continue(pc + 2)
            }
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
    let mut input = Some(1);
    //        debug_print(store, pc);
    loop {
        let op = Op::decode(store, pc)?;
        match op.run(store, pc, &mut input) {
            Status::Continue(next_pc) => {
                pc = next_pc;
            }
            Status::Halt => {
                break;
            }
        }
        //                debug_print(store, pc);
    }
    //        debug_print(store, pc);
    Ok(())
}

fn run() -> Result<(), Error> {
    let mut input = read_input()?;

    solve(&mut input)?;

    println!();
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
        let mut store = vec![
            3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1102, 7, 85, 225, 1102, 67, 12, 225,
            102, 36, 65, 224, 1001, 224, -3096, 224, 4, 224, 1002, 223, 8, 223, 101, 4, 224, 224,
            1, 224, 223, 223, 1001, 17, 31, 224, 1001, 224, -98, 224, 4, 224, 1002, 223, 8, 223,
            101, 5, 224, 224, 1, 223, 224, 223, 1101, 86, 19, 225, 1101, 5, 27, 225, 1102, 18, 37,
            225, 2, 125, 74, 224, 1001, 224, -1406, 224, 4, 224, 102, 8, 223, 223, 101, 2, 224,
            224, 1, 224, 223, 223, 1102, 13, 47, 225, 1, 99, 14, 224, 1001, 224, -98, 224, 4, 224,
            102, 8, 223, 223, 1001, 224, 2, 224, 1, 224, 223, 223, 1101, 38, 88, 225, 1102, 91, 36,
            224, 101, -3276, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 3, 224, 224, 1, 224, 223,
            223, 1101, 59, 76, 224, 1001, 224, -135, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 6,
            224, 1, 223, 224, 223, 101, 90, 195, 224, 1001, 224, -112, 224, 4, 224, 102, 8, 223,
            223, 1001, 224, 7, 224, 1, 224, 223, 223, 1102, 22, 28, 225, 1002, 69, 47, 224, 1001,
            224, -235, 224, 4, 224, 1002, 223, 8, 223, 101, 5, 224, 224, 1, 223, 224, 223, 4, 223,
            99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247,
            1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106,
            0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280,
            1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0,
            300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 107,
            226, 226, 224, 102, 2, 223, 223, 1006, 224, 329, 1001, 223, 1, 223, 1107, 677, 226,
            224, 1002, 223, 2, 223, 1005, 224, 344, 101, 1, 223, 223, 108, 677, 226, 224, 102, 2,
            223, 223, 1006, 224, 359, 101, 1, 223, 223, 7, 677, 226, 224, 102, 2, 223, 223, 1005,
            224, 374, 101, 1, 223, 223, 1008, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 389,
            1001, 223, 1, 223, 7, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 404, 101, 1, 223,
            223, 1007, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 419, 101, 1, 223, 223, 7, 226,
            226, 224, 102, 2, 223, 223, 1005, 224, 434, 1001, 223, 1, 223, 8, 226, 226, 224, 1002,
            223, 2, 223, 1006, 224, 449, 101, 1, 223, 223, 1007, 677, 677, 224, 102, 2, 223, 223,
            1006, 224, 464, 101, 1, 223, 223, 1007, 226, 677, 224, 1002, 223, 2, 223, 1006, 224,
            479, 101, 1, 223, 223, 108, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 494, 1001, 223,
            1, 223, 1108, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 509, 1001, 223, 1, 223, 107,
            226, 677, 224, 1002, 223, 2, 223, 1005, 224, 524, 101, 1, 223, 223, 1108, 677, 226,
            224, 1002, 223, 2, 223, 1005, 224, 539, 1001, 223, 1, 223, 1008, 677, 677, 224, 1002,
            223, 2, 223, 1006, 224, 554, 101, 1, 223, 223, 1008, 226, 226, 224, 102, 2, 223, 223,
            1005, 224, 569, 1001, 223, 1, 223, 8, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 584,
            101, 1, 223, 223, 107, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 599, 101, 1, 223,
            223, 8, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 614, 101, 1, 223, 223, 1107, 226,
            677, 224, 102, 2, 223, 223, 1006, 224, 629, 101, 1, 223, 223, 108, 677, 677, 224, 1002,
            223, 2, 223, 1005, 224, 644, 1001, 223, 1, 223, 1107, 226, 226, 224, 102, 2, 223, 223,
            1005, 224, 659, 101, 1, 223, 223, 1108, 226, 677, 224, 102, 2, 223, 223, 1005, 224,
            674, 101, 1, 223, 223, 4, 223, 99, 226,
        ];
        let _result = solve(&mut store).unwrap();
    }
}
