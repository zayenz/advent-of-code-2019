#![allow(dead_code, unused_imports, clippy::ptr_arg)]

use failure::bail;
use failure::err_msg;
use failure::Error;
use itertools::*;
use rayon::prelude::*;
use strum_macros::EnumString;

use std::char;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::io::BufRead;
use std::ops::*;
use std::str;
use std::str::FromStr;
use std::{io, process};

use aoc2019::input::get_numbers;
use intcode::*;

type Input = Vec<Word>;
type Output = usize;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let mut numbers = get_numbers::<Word>(&line)?;
        result.append(&mut numbers);
    }

    Ok(result)
}

fn solve(code: &mut Input) -> Result<Output, Error> {
    code[0] = 2;
    let mut program = IntCode::new(code);
    let mut input = VecDeque::new();
    let status = program.run(&mut input)?;
    assert_eq!(status, Status::Done);
    let result = program
        .output_iter()
        .tuples()
        .filter(|&(_x, _y, id)| *id == 2)
        .count();
    Ok(result)
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
