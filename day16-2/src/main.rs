#![allow(dead_code, unused_imports, clippy::ptr_arg, clippy::let_and_return)]

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

type Input = (usize, Vec<i64>);
type Output = i64;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let steps_line = lines.next().unwrap()?;
    let steps = get_numbers::<usize>(&steps_line)?;
    let data_line = lines.next().unwrap()?;
    let data = data_line
        .trim()
        .chars()
        .map(|ch: char| ch.to_digit(10).unwrap() as i64)
        .collect_vec();

    Ok((steps[0], data))
}

fn step(data: &[i64]) -> Vec<i64> {
    let mut sums = vec![0; data.len()];
    sums[data.len() - 1] = data[data.len() - 1];
    for pos in (0..data.len() - 1).rev() {
        sums[pos] = (sums[pos + 1] + data[pos]).abs() % 10;
    }
    sums
}

fn solve((steps, data): &Input) -> Result<Output, Error> {
    let position = to_integer(&data[0..7]) as usize;

    let mut current = (0..10000)
        .flat_map(|_| data.iter().cloned())
        .skip(position)
        .collect_vec();
    for _ in 0..*steps {
        let next = step(&current);
        current = next;
    }

    let checksum = decode(&current);

    Ok(checksum)
}

fn decode(data: &[i64]) -> i64 {
    let message = to_integer(&data[0..8]);
    message
}

fn to_integer(digits: &[i64]) -> i64 {
    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(pos, &digit)| 10i64.pow(pos as u32) * digit)
        .sum()
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

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn sample0() {
        let data = vec![
            0, 3, 0, 3, 6, 7, 3, 2, 5, 7, 7, 2, 1, 2, 9, 4, 4, 0, 6, 3, 4, 9, 1, 5, 6, 5, 4, 7, 4,
            6, 6, 4,
        ];
        let steps = 100;
        let checksum = solve(&(steps, data)).ok().unwrap();
        assert_eq!(checksum, 84462026);
    }

    #[test]
    fn puzzle() {
        let data = vec![
            5, 9, 7, 0, 5, 3, 7, 9, 1, 5, 0, 2, 2, 0, 1, 8, 8, 7, 5, 3, 3, 1, 6, 4, 1, 2, 9, 2, 5,
            2, 3, 7, 0, 0, 3, 6, 2, 3, 3, 4, 1, 8, 7, 3, 5, 0, 2, 5, 6, 2, 1, 6, 5, 6, 1, 8, 6, 8,
            1, 8, 9, 5, 8, 4, 6, 8, 3, 8, 9, 5, 6, 3, 0, 6, 0, 2, 6, 9, 8, 1, 0, 9, 1, 6, 1, 8, 9,
            0, 2, 9, 6, 4, 5, 0, 5, 3, 1, 7, 5, 8, 9, 9, 7, 5, 3, 5, 3, 8, 0, 3, 8, 9, 1, 3, 4, 0,
            6, 8, 8, 7, 2, 6, 3, 1, 9, 9, 1, 2, 0, 7, 2, 7, 6, 2, 1, 9, 7, 2, 0, 8, 6, 0, 0, 5, 2,
            2, 2, 5, 6, 2, 2, 6, 2, 7, 7, 0, 4, 5, 1, 9, 6, 7, 4, 5, 2, 7, 5, 9, 2, 5, 5, 9, 5, 2,
            8, 5, 8, 4, 3, 4, 9, 0, 5, 8, 2, 2, 5, 7, 1, 9, 4, 9, 6, 3, 7, 5, 0, 5, 2, 3, 7, 8, 9,
            2, 6, 0, 2, 9, 7, 7, 3, 7, 9, 4, 7, 1, 2, 6, 7, 0, 4, 6, 6, 8, 5, 5, 5, 8, 4, 7, 1, 4,
            9, 1, 2, 5, 2, 5, 6, 1, 7, 7, 4, 2, 8, 0, 0, 7, 6, 0, 6, 3, 3, 8, 2, 6, 3, 6, 6, 0, 7,
            6, 5, 3, 3, 5, 4, 3, 4, 9, 1, 4, 9, 6, 1, 3, 2, 4, 5, 2, 6, 5, 6, 5, 7, 3, 0, 3, 0, 4,
            1, 0, 3, 8, 5, 7, 9, 8, 5, 8, 6, 0, 3, 0, 8, 9, 0, 6, 0, 0, 2, 3, 9, 4, 9, 8, 9, 4, 7,
            1, 0, 3, 1, 0, 5, 8, 2, 6, 6, 4, 3, 3, 3, 1, 7, 3, 7, 8, 3, 4, 6, 8, 8, 8, 6, 6, 2, 3,
            2, 3, 1, 9, 8, 4, 9, 9, 3, 8, 7, 3, 9, 1, 7, 5, 5, 1, 4, 0, 0, 0, 9, 8, 2, 4, 1, 8, 6,
            6, 6, 2, 9, 5, 0, 6, 9, 4, 8, 7, 9, 9, 3, 4, 5, 8, 2, 6, 6, 1, 0, 4, 8, 4, 6, 4, 3, 8,
            5, 1, 4, 1, 7, 8, 7, 3, 6, 3, 9, 4, 9, 2, 4, 2, 8, 8, 9, 6, 5, 2, 0, 9, 2, 7, 6, 1, 0,
            9, 0, 6, 5, 7, 2, 2, 4, 2, 5, 9, 1, 8, 2, 5, 8, 9, 4, 6, 9, 1, 6, 6, 8, 0, 7, 7, 8, 8,
            6, 5, 1, 5, 5, 7, 7, 4, 7, 6, 3, 1, 5, 7, 1, 3, 5, 7, 2, 0, 7, 6, 3, 7, 0, 8, 7, 1, 6,
            8, 9, 0, 4, 2, 5, 1, 9, 8, 7, 8, 8, 0, 7, 7, 6, 5, 6, 6, 3, 6, 0, 6, 8, 1, 1, 0, 8, 4,
            7, 0, 5, 8, 5, 4, 8, 8, 4, 9, 9, 8, 8, 9, 0, 4, 4, 8, 5, 1, 6, 9, 4, 0, 3, 5, 7, 6, 2,
            7, 0, 9, 0, 5, 3, 5, 8, 6, 8, 7, 7, 8, 1, 5, 1, 1, 5, 4, 4, 8, 8, 4, 9, 6, 5, 4, 6, 8,
            5, 7, 6, 3, 0, 5, 4, 4, 0, 6, 9, 1, 1, 8, 5, 5, 6, 0, 6, 2, 8, 3, 2, 4, 6, 1, 1, 8, 6,
            9, 9, 1, 8, 7, 0, 5, 9, 4, 2, 4, 0, 7, 7, 5, 6, 4, 0, 3, 7, 1, 7, 6, 7, 8, 7, 9, 7, 6,
            6, 8, 1, 3, 0, 9, 8, 7, 0, 9, 3, 1,
        ];
        let steps = 4;
        let checksum = solve(&(steps, data)).ok().unwrap();
        assert_eq!(checksum, 01029498);
    }
}
