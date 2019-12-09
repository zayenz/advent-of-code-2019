#![allow(dead_code, unused_imports, clippy::collapsible_if, clippy::map_entry)]

use failure::bail;
use failure::err_msg;
use failure::Error;
use itertools::Itertools;
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

use aoc2019::input::{get_numbers, get_words};
use aoc2019::position::*;
use intcode::*;

type Input = Vec<Vec<(Direction, usize)>>;
type Output = usize;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let words = get_words(&line);
        let mut numbers = get_numbers::<usize>(&line)?;
        let mut directions = words
            .iter()
            .map(|word| match *word {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!(format!("Unknown word {}", word)),
            })
            .collect_vec();
        if !directions.is_empty() {
            result.push(directions.drain(..).zip_eq(numbers.drain(..)).collect_vec());
        }
    }

    Ok(result)
}

fn solve(directions: &mut Input) -> Result<Output, Error> {
    use aoc2019::position::Step;
    let size = {
        let origin = Position::new(0, 0);
        let (min_x, max_x, min_y, max_y) = directions
            .iter()
            .map(|directions| {
                let mut min_x = 0;
                let mut max_x = 0;
                let mut min_y = 0;
                let mut max_y = 0;
                let mut position = origin;
                for &(direction, steps) in directions {
                    position = position.step_by(direction, steps as Scalar);
                    min_x = min(min_x, position.x);
                    max_x = max(max_x, position.x);
                    min_y = min(min_y, position.y);
                    max_y = max(max_y, position.y);
                }
                (min_x, max_x, min_y, max_y)
            })
            .fold(
                (0, 0, 0, 0),
                |(min_x1, max_x1, min_y1, max_y1), (min_x2, max_x2, min_y2, max_y2)| {
                    (
                        min(min_x1, min_x2),
                        max(max_x1, max_x2),
                        min(min_y1, min_y2),
                        max(max_y1, max_y2),
                    )
                },
            );
        (max(max_x - min_x, max_y - min_y) + 2) * 2
    } as usize;

    let origin: Position = (size / 2, size / 2).into();
    let paths = directions
        .iter()
        .map(|steps| {
            let mut grid = vec![vec![false; size]; size];
            let mut pos = origin;
            grid[pos.x as usize][pos.y as usize] = true;
            for (direction, steps) in steps.iter() {
                for _ in 0..*steps {
                    pos = pos.step(*direction);
                    grid[pos.x as usize][pos.y as usize] = true;
                }
            }
            grid
        })
        .collect::<Vec<_>>();
    let crossings: HashSet<Position> = (0..size)
        .cartesian_product(0..size)
        .filter(|&(x, y)| paths.iter().all(|grid| grid[x][y]))
        .filter(|pos| origin != pos.into())
        .map(|pos| pos.into())
        .collect();

    let distances: Vec<HashMap<Position, usize>> = directions
        .iter()
        .map(|directions| {
            let mut distances = HashMap::new();
            let mut position = origin;
            let mut total_steps = 0;
            for &(direction, steps) in directions {
                for _ in 0..steps {
                    total_steps += 1;
                    position = position.step(direction);
                    if crossings.contains(&position) {
                        if !distances.contains_key(&position) {
                            distances.insert(position, total_steps);
                        }
                    }
                }
            }
            distances
        })
        .collect();

    let result = crossings
        .iter()
        .map(|crossing| distances.iter().map(|m| m[crossing]).sum())
        .min()
        .expect("Must exist a crossing reachable from all");

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
