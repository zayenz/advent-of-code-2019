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
type Output = Word;

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

fn solve(code: &Input) -> Result<Output, Error> {
    let programs = (0..5).map(|_| IntCode::new(code)).collect_vec();
    let result = (5..10)
        .permutations(5)
        .map(|phase_settings| {
            let mut inputs = phase_settings
                .iter()
                .map(|phase| {
                    let mut stream = VecDeque::new();
                    stream.push_back(*phase);
                    stream
                })
                .collect_vec();
            inputs[0].push_back(0);
            let mut programs = programs.clone();
            loop {
                for p in 0..5 {
                    let result = programs[p].run(&mut inputs[p]).ok().unwrap();
                    if p == 4 && result == Status::Done {
                        return programs[p].pop_output().ok().unwrap();
                    }
                    inputs[(p + 1) % 5].push_back(programs[p].pop_output().ok().unwrap());
                }
            }
        })
        .max()
        .expect("Must exist");

    Ok(result)
}

fn run() -> Result<(), Error> {
    let input = read_input()?;

    let output = solve(&input)?;

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
