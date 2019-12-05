#![allow(dead_code, unused_imports)]

use failure::bail;
use failure::err_msg;
use failure::Error;
use itertools::*;
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
use intcode::*;

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

fn solve(program: &mut Input) -> Result<Output, Error> {
    const USE_PARALLELISM: bool = true;
    let result = if USE_PARALLELISM {
        (0..=99)
            .cartesian_product(0..=99)
            .collect_vec()
            .par_iter()
            .map(|(noun, verb)| {
                let mut local = program.clone();
                local[1] = *noun;
                local[2] = *verb;
                let mut ic = IntCode::new(&local);
                ic.run(&mut ()).unwrap_or(());
                let result = ic.store()[0];
                (noun, verb, result)
            })
            .find_any(|(_noun, _verb, result)| *result == 19_690_720)
            .map(|(noun, verb, _result)| 100 * noun + verb)
            .ok_or_else(|| err_msg("No valid pair found"))?
    } else {
        (0..=99)
            .cartesian_product(0..=99)
            .map(|(noun, verb)| {
                let mut local = program.clone();
                local[1] = noun;
                local[2] = verb;
                let mut ic = IntCode::new(&local);
                ic.run(&mut ()).unwrap_or(());
                let result = ic.store()[0];
                (noun, verb, result)
            })
            .find(|(_noun, _verb, result)| *result == 19_690_720)
            .map(|(noun, verb, _result)| 100 * noun + verb)
            .ok_or_else(|| err_msg("No valid pair found"))?
    };
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
