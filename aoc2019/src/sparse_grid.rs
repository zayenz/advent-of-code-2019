use crate::position::*;
use hashbrown::HashMap;
use std::cmp::max;
use std::cmp::min;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grid<T>
where
    T: Debug + Clone + Eq,
{
    pub values: HashMap<Position, T>,
    pub min_x: Scalar,
    pub min_y: Scalar,
    pub max_x: Scalar,
    pub max_y: Scalar,
}

impl<T> Grid<T>
where
    T: Debug + Clone + Eq,
{
    pub fn new() -> Grid<T> {
        Grid {
            values: HashMap::with_capacity(256),
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
        }
    }

    pub fn filled(
        value: &T,
        min_x: Scalar,
        min_y: Scalar,
        max_x: Scalar,
        max_y: Scalar,
    ) -> Grid<T> {
        let mut grid = Grid::new();
        for x in min_x..max_x {
            for y in min_y..max_y {
                grid.insert((x, y).into(), value.clone());
            }
        }
        grid
    }

    fn update_bounds(&mut self, position: Position) {
        self.min_x = min(self.min_x, position.x);
        self.min_y = min(self.min_y, position.y);
        self.max_x = max(self.max_x, position.x);
        self.max_y = max(self.max_y, position.y);
    }

    pub fn insert(&mut self, position: Position, value: T) {
        self.update_bounds(position);
        self.values.insert(position, value);
    }

    pub fn get(&self, position: Position) -> Option<&T> {
        self.values.get(&position)
    }
}

impl<T> Default for Grid<T>
where
    T: Debug + Clone + Eq,
{
    fn default() -> Self {
        Grid::new()
    }
}

impl<T> Index<Position> for Grid<T>
where
    T: Debug + Clone + Eq,
{
    type Output = T;

    fn index(&self, index: Position) -> &T {
        self.values.get(&index).unwrap()
    }
}

impl<T> IndexMut<Position> for Grid<T>
where
    T: Debug + Clone + Eq,
{
    fn index_mut(&mut self, index: Position) -> &mut T {
        self.values.get_mut(&index).unwrap()
    }
}

impl<T> Index<(Scalar, Scalar)> for Grid<T>
where
    T: Debug + Clone + Eq,
{
    type Output = T;

    fn index(&self, index: (Scalar, Scalar)) -> &T {
        self.values.get(&index.into()).unwrap()
    }
}

impl<T> IndexMut<(Scalar, Scalar)> for Grid<T>
where
    T: Debug + Clone + Eq,
{
    fn index_mut(&mut self, index: (Scalar, Scalar)) -> &mut T {
        self.values.get_mut(&index.into()).unwrap()
    }
}

impl<T> Index<(usize, usize)> for Grid<T>
where
    T: Debug + Clone + Eq,
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &T {
        self.values.get(&index.into()).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T>
where
    T: Debug + Clone + Eq,
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        self.values.get_mut(&index.into()).unwrap()
    }
}

impl<T> Display for Grid<T>
where
    T: Display + Debug + Clone + Eq,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let width = self
            .values
            .values()
            .map(|v| format!("{}", v).len())
            .max()
            .unwrap_or(1);
        let filler = " ".repeat(width);
        for y in self.min_y..self.max_y {
            for x in self.min_x..self.max_x {
                if let Some(v) = self.get((x, y).into()) {
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
