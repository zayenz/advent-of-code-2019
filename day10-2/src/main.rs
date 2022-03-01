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
                Some(((row, col), Point2::new(row as f64 + 0.5, col as f64 + 0.5)))
            } else {
                None
            }
        })
        .collect_vec();

    use decorum::N64;
    #[derive(Clone)]
    struct Target {
        angle: N64,
        distance: N64,
        id: (usize, usize),
        position: Point2,
    };

    let _hit_groups = asteroids
        .iter()
        .map(|&(_id, center)| {
            let mut others = asteroids
                .iter()
                .flat_map(|&(oid, other)| {
                    if center == other {
                        None
                    } else {
                        let direction = LineSegment::new(center, other);
                        let angle = N64::try_from(direction.angle()).expect("No NaN angles");
                        let distance = N64::try_from(direction.length()).expect("No NaN angles");
                        //Some((angle, distance, oid, other))
                        Some(Target {
                            angle,
                            distance,
                            id: oid,
                            position: other,
                        })
                    }
                })
                .sorted_by_key(|target| (target.angle, target.distance))
                .collect_vec();
            let mut groups = Vec::new();
            if others.is_empty() {
                return groups;
            }
            let mut current = vec![others[0].clone()];
            let mut current_angle = current[0].angle;
            for other in others.drain(..).skip(1) {
                let angle = other.angle;
                let diff: f64 = (angle - current_angle).into();
                if diff.abs() < 1e-5 {
                    current.push(other);
                } else {
                    groups.push(
                        current
                            .iter()
                            .cloned()
                            .sorted_by_key(|other| other.distance)
                            .collect_vec(),
                    );
                    current_angle = other.angle;
                    current = vec![other];
                }
            }
            groups.push(
                current
                    .iter()
                    .cloned()
                    .sorted_by_key(|other| other.distance)
                    .collect_vec(),
            );
            groups
        })
        .collect_vec();

    Ok(0)
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
