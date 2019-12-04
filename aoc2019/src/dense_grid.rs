#![allow(
dead_code,
unused_imports,
clippy::needless_range_loop,
clippy::ptr_arg,
clippy::char_lit_as_u8
)]

use crate::position::*;
use hashbrown::HashMap;
use std::cmp::max;
use std::cmp::min;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Index;
use std::ops::IndexMut;
use strum_macros::EnumString;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grid<T>
where
    T: Debug + Clone + Eq + Default,
{
    pub values: Vec<T>,
    pub min_x: Scalar,
    pub min_y: Scalar,
    pub max_x: Scalar,
    pub max_y: Scalar,
    pub height: usize,
    pub width: usize,
}

impl<T> Grid<T>
where
    T: Debug + Clone + Eq + Default,
{
    pub fn new(min_x: Scalar, min_y: Scalar, max_x: Scalar, max_y: Scalar) -> Grid<T> {
        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;
        let positions = width * height;
        let values = vec![T::default(); positions];
        Grid {
            values,
            min_x,
            min_y,
            max_x,
            max_y,
            height,
            width,
        }
    }

    pub fn from_origo(width: usize, height: usize) -> Grid<T> {
        let positions = width * height;
        let values = vec![T::default(); positions];
        Grid {
            values,
            min_x: 0,
            min_y: 0,
            max_x: (width - 1) as i32,
            max_y: (height - 1) as i32,
            height,
            width,
        }
    }

    pub fn in_bounds(&self, position: Position) -> bool {
        self.min_x <= position.x
            && position.x <= self.max_x
            && self.min_y <= position.y
            && position.y <= self.max_y
    }

    fn index(&self, position: Position) -> usize {
        if self.min_x == 0 && self.min_y == 0 {
            position.y as usize * self.width + position.x as usize
        } else {
            panic!("Unhandled case right now");
        }
    }

    pub fn insert(&mut self, position: Position, value: T) {
        if !self.in_bounds(position) {
            panic!(format!(
                "Position {} is not in bounds {}, {}, {}, {}",
                position, self.min_x, self.max_x, self.min_y, self.max_y
            ));
        }
        let index = self.index(position);
        self.values[index] = value;
    }

    pub fn get<I>(&self, position: I) -> Option<&T>
    where
        I: Into<Position>,
    {
        self.values.get(self.index(position.into()))
    }

    pub fn get_mut<I>(&mut self, position: I) -> Option<&mut T>
    where
        I: Into<Position>,
    {
        let index = self.index(position.into());
        self.values.get_mut(index)
    }
}

impl<T> Index<Position> for Grid<T>
where
    T: Debug + Clone + Eq + Default,
{
    type Output = T;

    fn index(&self, position: Position) -> &T {
        self.get(position).unwrap()
    }
}

impl<T> IndexMut<Position> for Grid<T>
where
    T: Debug + Clone + Eq + Default,
{
    fn index_mut(&mut self, position: Position) -> &mut T {
        self.get_mut(position).unwrap()
    }
}

impl<T> Index<(Scalar, Scalar)> for Grid<T>
where
    T: Debug + Clone + Eq + Default,
{
    type Output = T;

    fn index(&self, position: (Scalar, Scalar)) -> &T {
        self.get(position).unwrap()
    }
}

impl<T> IndexMut<(Scalar, Scalar)> for Grid<T>
where
    T: Debug + Clone + Eq + Default,
{
    fn index_mut(&mut self, position: (Scalar, Scalar)) -> &mut T {
        self.get_mut(position).unwrap()
    }
}

impl<T> Index<(usize, usize)> for Grid<T>
where
    T: Debug + Clone + Eq + Default,
{
    type Output = T;

    fn index(&self, position: (usize, usize)) -> &T {
        self.get(position).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T>
where
    T: Debug + Clone + Eq + Default,
{
    fn index_mut(&mut self, position: (usize, usize)) -> &mut T {
        self.get_mut(position).unwrap()
    }
}

impl<T> Display for Grid<T>
where
    T: Display + Debug + Clone + Eq + Default,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let width = self
            .values
            .iter()
            .map(|v| format!("{}", v).len())
            .max()
            .unwrap_or(1);
        let filler = " ".repeat(width);
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                if let Some(v) = self.get((x, y)) {
                    write!(f, "{:width$}", v, width = width)?;
                } else {
                    write!(f, "{}", filler)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
