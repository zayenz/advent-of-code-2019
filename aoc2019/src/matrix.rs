use failure::Error;

use std::fmt;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    data: Vec<bool>,
}

impl Matrix {
    pub fn new(width: usize, height: usize) -> Matrix {
        Matrix {
            width,
            height,
            data: vec![false; width * height],
        }
    }

    pub fn count_true(&self) -> usize {
        self.data.iter().filter(|&&v| v).count()
    }
    pub fn count_false(&self) -> usize {
        self.data.iter().filter(|&&v| !v).count()
    }

    fn pos(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width && y < self.height);
        x + (y * self.width)
    }

    pub fn slice(&self, start_x: usize, start_y: usize, width: usize, height: usize) -> Matrix {
        let mut data = Vec::with_capacity(width * height);
        for x in start_x..(start_x + width) {
            for y in start_y..(start_y + height) {
                data.push(self[(x, y)])
            }
        }
        Matrix {
            width,
            height,
            data,
        }
    }

    pub fn fill_with(
        &mut self,
        start_x: usize,
        start_y: usize,
        width: usize,
        height: usize,
        value: bool,
    ) {
        for x_offset in 0..width {
            for y_offset in 0..height {
                self[(start_x + x_offset, start_y + y_offset)] = value;
            }
        }
    }

    pub fn fill_true(&mut self, start_x: usize, start_y: usize, width: usize, height: usize) {
        self.fill_with(start_x, start_y, width, height, true);
    }

    pub fn fill_false(&mut self, start_x: usize, start_y: usize, width: usize, height: usize) {
        self.fill_with(start_x, start_y, width, height, false);
    }

    pub fn fill_from(&mut self, x: usize, y: usize, source: &Matrix) {
        for source_x in 0..source.width {
            for source_y in 0..source.height {
                self[(x + source_x, y + source_y)] = source[(source_x, source_y)];
            }
        }
    }

    pub fn invert(&mut self, start_x: usize, start_y: usize, width: usize, height: usize) {
        for x_offset in 0..width {
            for y_offset in 0..height {
                let previous_value = self[(start_x + x_offset, start_y + y_offset)];
                self[(start_x + x_offset, start_y + y_offset)] = !previous_value;
            }
        }
    }

    pub fn row(&self, y: usize) -> Matrix {
        let mut result = Matrix::new(self.width, 1);
        for x in 0..self.width {
            result[(x, 1)] = self[(x, y)];
        }
        result
    }

    pub fn col(&self, x: usize) -> Matrix {
        let mut result = Matrix::new(1, self.height);
        for y in 0..self.height {
            result[(1, y)] = self[(x, y)];
        }
        result
    }

    pub fn rotate_row(&mut self, y: usize, steps: usize) {
        let row = self.row(y);
        let rot_row: Vec<bool> = row.iter().cycle().skip(steps).take(self.width).collect();
        for (offset, value) in rot_row.into_iter().enumerate() {
            self[(offset, y)] = value;
        }
    }

    pub fn rotate_col(&mut self, x: usize, steps: usize) {
        let col = self.col(x);
        let rot_col: Vec<bool> = col.iter().cycle().skip(steps).take(self.height).collect();
        for (offset, value) in rot_col.into_iter().enumerate() {
            self[(x, offset)] = value;
        }
    }

    pub fn rot90(&self) -> Matrix {
        let mut result = Matrix::new(self.height, self.width);

        for x in 0..self.width {
            for y in 0..self.height {
                let xr90 = y;
                let yr90 = self.height - x - 1;
                result[(xr90, yr90)] = self[(x, y)];
            }
        }

        result
    }

    pub fn flip(&self) -> Matrix {
        let mut result = Matrix::new(self.width, self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                result[(self.width - x - 1, y)] = self[(x, y)];
            }
        }

        result
    }

    pub fn iter(&self) -> MatrixIterator<'_> {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a Matrix {
    type Item = bool;
    type IntoIter = MatrixIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MatrixIterator {
            matrix: self,
            index: 0,
        }
    }
}

#[derive(Clone)]
pub struct MatrixIterator<'a> {
    matrix: &'a Matrix,
    index: usize,
}

impl<'a> Iterator for MatrixIterator<'a> {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        if self.index < self.matrix.data.len() {
            let result = self.matrix.data[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.data[self.pos(x, y)]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        let position = self.pos(x, y);
        &mut self.data[position]
    }
}

impl FromStr for Matrix {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let rows: Vec<&str> = s.split('/').collect();
        if rows.is_empty() {
            return Ok(Matrix::new(0, 0));
        }
        let width = rows[0].len();
        let height = rows.len();
        let mut result = Matrix::new(width, height);
        for (y, row) in rows.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                result[(x, y)] = ch == '#';
            }
        }
        Ok(result)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", if self[(x, y)] { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
