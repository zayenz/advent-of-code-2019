#![allow(dead_code, unused_imports, clippy::ptr_arg, clippy::let_and_return)]

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

use aoc2019::input::get_numbers;
use aoc2019::position::*;
use aoc2019::sparse_grid::*;
use intcode::*;
use std::convert::TryInto;
use std::sync::atomic::AtomicU8;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Planet {
    id: u8,
    x: i64,
    y: i64,
    z: i64,
    dx: i64,
    dy: i64,
    dz: i64,
}

fn compute_change(current: i64, other: i64) -> i64 {
    match current.cmp(&other) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
    }
}

impl Planet {
    fn new(x: i64, y: i64, z: i64) -> Planet {
        static COUNTER: AtomicU8 = AtomicU8::new(0);
        let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Planet {
            id,
            x,
            y,
            z,
            dx: 0,
            dy: 0,
            dz: 0,
        }
    }

    fn change_velocity(&mut self, previous: &Planet, other: &Planet) {
        assert_eq!(self.id, previous.id);
        assert_ne!(self.id, other.id);
        self.dx += compute_change(previous.x, other.x);
        self.dy += compute_change(previous.y, other.y);
        self.dz += compute_change(previous.z, other.z);
    }

    fn step(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.z += self.dz;
    }

    fn energy(&self) -> i64 {
        let potential = self.x.abs() + self.y.abs() + self.z.abs();
        let kinetic = self.dx.abs() + self.dy.abs() + self.dz.abs();
        potential * kinetic
    }
}

type Input = Vec<Planet>;
type Output = i64;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut planets = Vec::new();
    for line in stdin.lock().lines() {
        let planet_line = line?;
        println!("Read {}", planet_line);
        let coordinates = get_numbers::<i64>(&planet_line)?;
        planets.push(Planet::new(coordinates[0], coordinates[1], coordinates[2]));
    }

    Ok(planets)
}

fn debug_print(planets: &[Planet]) {
    for planet in planets {
        println!(
            "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>",
            planet.x, planet.y, planet.z, planet.dx, planet.dy, planet.dz,
        );
    }
    println!();
}

fn compress(planets: &[Planet]) -> Result<[i16; 12], Error> {
    assert_eq!(planets.len(), 4);

    Ok([
        planets[0].x.try_into()?,
        planets[0].y.try_into()?,
        planets[0].z.try_into()?,
        planets[1].x.try_into()?,
        planets[1].y.try_into()?,
        planets[1].z.try_into()?,
        planets[2].x.try_into()?,
        planets[2].y.try_into()?,
        planets[2].z.try_into()?,
        planets[3].x.try_into()?,
        planets[3].y.try_into()?,
        planets[3].z.try_into()?,
    ])
}

fn solve(start_planets: Input) -> Result<Output, Error> {
    let planet_count = start_planets.len();
    let mut states = HashSet::new();
    let mut planets = start_planets.clone();
    states.insert(compress(&planets)?);
    //    debug_print(&planets);
    for i in 0..2_000_000_000 {
        let mut next_planets = planets.clone();
        for to_modify in 0..planet_count {
            for modify_from in 0..planet_count {
                if to_modify != modify_from {
                    next_planets[to_modify]
                        .change_velocity(&planets[to_modify], &planets[modify_from]);
                }
            }
        }
        for planet in &mut next_planets {
            planet.step();
        }
        planets = next_planets;
        if !states.insert(compress(&planets)?) {
            return Ok(i);
        }
    }
    Err(err_msg("Found no result in 2 billion steps"))
}

fn run() -> Result<(), Error> {
    let input = read_input()?;

    let output = solve(input)?;

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
    use super::*;
    #[test]
    fn t() {
        let planets = vec![
            Planet::new(-1, 0, 2),
            Planet::new(2, -10, -7),
            Planet::new(4, -8, 8),
            Planet::new(3, 5, -1),
        ];

        solve(planets).ok().unwrap();
    }
}
