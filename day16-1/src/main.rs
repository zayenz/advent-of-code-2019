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
    (0..data.len())
        .map(|pos| {
            //        let factors = [0, 1, 0, -1].iter().flat_map(|&v| repeat_n(v, pos+1).collect_vec()).cycle().skip(1).take(data.len()).collect_vec();
            //        dbg!(factors);
            let factors = [0, 1, 0, -1]
                .iter()
                .flat_map(|&v| repeat_n(v, pos + 1).collect_vec())
                .cycle()
                .skip(1);
            let sum: i64 = data
                .iter()
                .zip(factors)
                .map(|(&value, factor)| value * factor)
                .sum();
            sum.abs() % 10
        })
        .collect_vec()
}

fn solve((steps, data): &Input) -> Result<Output, Error> {
    let mut current = data.clone();
    for _ in 0..*steps {
        let next = step(&current);
        current = next;
    }

    let checksum = current[0..8]
        .iter()
        .rev()
        .enumerate()
        .map(|(pos, &digit)| 10i64.pow(pos as u32) * digit)
        .sum();

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

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn sample0() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let steps = 4;
        let checksum = solve(&(steps, data)).ok().unwrap();
        assert_eq!(checksum, 01029498);
    }

    #[test]
    fn sample0_1() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8];
        let steps = 4;
        let checksum = solve(&(steps, data)).ok().unwrap();
        assert_eq!(checksum, 01029498);
    }
}
