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

use aoc2019::geometry::*;
use aoc2019::input::get_numbers;
use intcode::*;
use joinery::JoinableIterator;
use stats::Frequencies;
use std::convert::TryFrom;

type Input = Vec<Vec<bool>>;
type Output = u64;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let row = line.trim().chars().map(|ch| ch == '#').collect_vec();
        result.push(row);
    }

    assert!(result
        .iter()
        .tuple_windows()
        .all(|(a, b)| a.len() == b.len()));

    Ok(result)
}

fn solve(input_matrix: &Input) -> Result<Output, Error> {
    let rows = input_matrix.len();
    let cols = input_matrix[0].len();
    let asteroids = (0..rows)
        .cartesian_product(0..cols)
        .flat_map(|(row, col)| {
            if input_matrix[row][col] {
                Some(Point2::new(row as f64 + 0.5, col as f64 + 0.5))
            } else {
                None
            }
        })
        .collect::<Vec<Point2>>();

    use decorum::N64;

    let hits = asteroids
        .iter()
        .map(|&center| {
            let angles = asteroids
                .iter()
                .flat_map(|&other| {
                    if center == other {
                        None
                    } else {
                        Some(
                            N64::try_from(LineSegment::new(center, other).angle())
                                .expect("No NaN angles"),
                        )
                    }
                })
                .sorted()
                .collect_vec();
            let mut hits = if angles.is_empty() { 0 } else { 1 };
            for (a, b) in angles.iter().cloned().tuple_windows() {
                let a: f64 = a.into();
                let b: f64 = b.into();
                if (a - b).abs() > 1e-5 {
                    hits += 1;
                }
            }
            hits
        })
        .collect_vec();

    Ok(*hits.iter().max().unwrap())
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
