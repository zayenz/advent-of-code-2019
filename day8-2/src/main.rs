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
use joinery::JoinableIterator;
use stats::Frequencies;

type Input = (usize, usize, Vec<u8>);
type Output = String;

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

    let layers = data.chunks(layer_size).collect_vec();

    let output = (0..layer_size)
        .map(|pos| {
            layers
                .iter()
                .map(|&layer| layer[pos])
                .find(|&v| v != 2)
                .expect("No fully transparent pixel should exist")
        })
        .collect_vec();

    let output_string = output
        .chunks(*width)
        .map(|line| {
            line.iter()
                .map(|&v| match v {
                    0 => '·',
                    1 => '#',
                    _ => unreachable!("Only 0 and 1 in output"),
                })
                .collect::<String>()
        })
        .join_with('\n')
        .to_string();

    Ok(output_string)
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
