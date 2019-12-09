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
use stats::Frequencies;

type Input = (usize, usize, Vec<u8>);
type Output = u64;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let sizes_line = lines.next().unwrap()?;
    let sizes = get_numbers::<usize>(&sizes_line)?;
    let data_line = lines.next().unwrap()?;
    let data = data_line
        .trim()
        .chars()
        .map(|ch: char| ch.to_digit(10).unwrap() as u8)
        .collect_vec();

    Ok((sizes[0], sizes[1], data))
}

fn solve((width, height, data): &Input) -> Result<Output, Error> {
    let layer_size = width * height;
    let checksum = data
        .chunks(layer_size)
        .map(|w| w.iter().cloned().collect::<Frequencies<u8>>())
        .min_by_key(|f| f.count(&0))
        .map(|f| f.count(&1) * f.count(&2))
        .expect("Result must exist");

    Ok(checksum)
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
