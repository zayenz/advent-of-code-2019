#![allow(dead_code, unused_imports, clippy::needless_range_loop)]

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

use aoc2019::input::{get_numbers, get_words};

type Input = (HashMap<String, usize>, Vec<(usize, usize)>);
type Output = usize;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut id = 0;
    let mut symbol_table = HashMap::new();
    let mut orbits = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let words = line.split(')').collect_vec();
        if !words.is_empty() {
            assert_eq!(words.len(), 2);
            let stationary = *symbol_table.entry(words[0].to_string()).or_insert_with(|| {
                let result = id;
                id += 1;
                result
            });
            let in_orbit = *symbol_table.entry(words[1].to_string()).or_insert_with(|| {
                let result = id;
                id += 1;
                result
            });
            orbits.push((stationary, in_orbit));
        }
    }

    Ok((symbol_table, orbits))
}

fn solve((symbol_table, orbits): &Input) -> Result<Output, Error> {
    let max_id = orbits
        .iter()
        .map(|&(a, b)| max(a, b))
        .max()
        .expect("Some orbit must exist");
    let id_to_name = symbol_table
        .iter()
        .sorted_by_key(|(_k, v)| **v)
        .map(|(k, _v)| k.clone())
        .collect_vec();
    #[derive(Debug)]
    struct Object {
        id: usize,
        name: String,
        outgoing: Vec<usize>,
        incoming: Vec<usize>,
    };
    let mut objects = Vec::new();
    for id in 0..=max_id {
        objects.push(Object {
            id,
            name: id_to_name[id].clone(),
            outgoing: vec![],
            incoming: vec![],
        })
    }
    for &(stationary, in_orbit) in orbits {
        objects[stationary].outgoing.push(in_orbit);
        objects[in_orbit].incoming.push(stationary);
    }

    let start_position = symbol_table["YOU"];
    let end_position = symbol_table["SAN"];

    let mut queue = VecDeque::new();
    queue.push_back((0, start_position));

    let mut visited = vec![false; max_id + 1];
    visited[start_position] = true;

    while let Some((steps, id)) = queue.pop_front() {
        if id == end_position {
            return Ok(steps);
        }
        queue.extend(
            objects[id]
                .incoming
                .iter()
                .chain(objects[id].outgoing.iter())
                .flat_map(|&next| {
                    if visited[next] {
                        None
                    } else {
                        visited[next] = true;
                        Some((steps + 1, next))
                    }
                }),
        );
    }
    Err(err_msg("Should have found some path by now..."))
}

fn run() -> Result<(), Error> {
    let input = read_input()?;

    let output = solve(&input)?;

    println!("{}", output - 2);
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
