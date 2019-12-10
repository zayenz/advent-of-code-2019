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

type Input = (usize, usize);
type Output = usize;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut numbers = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let mut line_numbers = get_numbers::<usize>(&line)?;
        numbers.append(&mut line_numbers);
    }

    Ok((numbers[0], numbers[1]))
}

fn solve(&(min, max): &Input) -> Result<Output, Error> {
    let count = (min..=max).filter(|&num| test_number(num)).count();
    Ok(count)
}

fn test_number(num: usize) -> bool {
    let digits = num
        .to_string()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect_vec();
    let repeated_pair = digits.iter().tuple_windows().any(|(a, b)| a == b);
    if !repeated_pair {
        return false;
    }
    let increasing = digits.iter().tuple_windows().all(|(a, b)| a <= b);
    if !increasing {
        return false;
    }
    let length_two_group = digits
        .iter()
        .group_by(|&&d| d)
        .into_iter()
        .any(|(_d, group)| group.count() == 2);
    if !length_two_group {
        return false;
    }

    true
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
    use crate::test_number;

    #[test]
    fn test_samples() {
        assert!(test_number(112233));
        assert!(!test_number(123444));
        assert!(test_number(111122));
    }
}
