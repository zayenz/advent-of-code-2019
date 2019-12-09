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
    let result = (0..5)
        .permutations(5)
        .map(|phase_settings| {
            let mut programs = programs.clone();
            programs.iter_mut().zip(phase_settings.iter()).fold(
                0,
                |input_signal, (program, phase_setting)| {
                    let mut input = VecDeque::new();
                    input.push_back(*phase_setting);
                    input.push_back(input_signal);
                    program.run(&mut input).ok().unwrap();
                    program.pop_output().ok().unwrap()
                },
            )
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
