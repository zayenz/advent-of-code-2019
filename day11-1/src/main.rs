#![allow(dead_code, unused_imports, clippy::ptr_arg)]

use failure::bail;
use failure::err_msg;
use failure::Error;
use itertools::Itertools;
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

use crate::Plate::{Black, Blank, White};
use aoc2019::input::get_numbers;
use aoc2019::position::*;
use aoc2019::sparse_grid::*;
use intcode::*;

type Input = Vec<Word>;
type Output = usize;

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

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Plate {
    Blank,
    Black,
    White,
}

impl Default for Plate {
    fn default() -> Self {
        Plate::Blank
    }
}

fn solve(code: &Input) -> Result<Output, Error> {
    let mut program = IntCode::new(code);
    let mut input = VecDeque::new();
    let mut grid = Grid::<Plate>::new();
    let mut position = Position::new(0, 0);

    let mut direction = Direction::Up;
    let mut next_is_painting = true;
    loop {
        //        debug_print(&grid, Some(position), direction);
        match program.run_to_output(&mut input)? {
            Status::Done => {
                break;
            }
            Status::NeedsInput => {
                let value = if *grid.get(position).unwrap_or(&Blank) == White {
                    1 // White is 1 as input
                } else {
                    0 // Blank and Black are both treated as black, so 0 as input
                };
                input.push_back(value);
            }
            Status::HasOutput => {
                let output = program.pop_output()?;
                if next_is_painting {
                    let paint = if output == 1 { White } else { Black };
                    grid.insert(position, paint);
                } else {
                    if output == 1 {
                        direction = direction.turn(Turn::Right);
                    } else {
                        direction = direction.turn(Turn::Left);
                    }
                    position = position.step(direction);
                };
                next_is_painting = !next_is_painting;
            }
        }
    }

    Ok(grid.values.len())
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
