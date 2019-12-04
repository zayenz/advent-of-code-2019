use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;
use strum_macros::EnumString;

pub trait Step<T: Copy>
where
    Self: Copy + Clone,
{
    fn step(&self, direction: T) -> Self;
}

pub type Scalar = i32;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: Scalar,
    pub y: Scalar,
}

impl Position {
    pub fn new(x: Scalar, y: Scalar) -> Position {
        Position { x, y }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Step<Cardinal> for Position {
    fn step(&self, direction: Cardinal) -> Self {
        use crate::position::Cardinal::*;
        let (x, y) = match direction {
            North => (self.x, self.y - 1),
            South => (self.x, self.y + 1),
            West => (self.x - 1, self.y),
            East => (self.x + 1, self.y),
        };
        Position { x, y }
    }
}

impl Step<Direction> for Position {
    fn step(&self, direction: Direction) -> Self {
        use crate::position::Direction::*;
        let (x, y) = match direction {
            Up => (self.x, self.y - 1),
            Down => (self.x, self.y + 1),
            Right => (self.x + 1, self.y),
            Left => (self.x - 1, self.y),
        };
        Position { x, y }
    }
}

impl From<(Scalar, Scalar)> for Position {
    fn from(pos: (Scalar, Scalar)) -> Self {
        Position { x: pos.0, y: pos.1 }
    }
}

impl From<&(Scalar, Scalar)> for Position {
    fn from(pos: &(Scalar, Scalar)) -> Self {
        Position { x: pos.0, y: pos.1 }
    }
}

impl From<(usize, usize)> for Position {
    fn from(pos: (usize, usize)) -> Self {
        Position {
            x: pos.0 as Scalar,
            y: pos.1 as Scalar,
        }
    }
}

impl From<&(usize, usize)> for Position {
    fn from(pos: &(usize, usize)) -> Self {
        Position {
            x: pos.0 as Scalar,
            y: pos.1 as Scalar,
        }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        (self.x + rhs.x, self.y + rhs.y).into()
    }
}

impl Add for &Position {
    type Output = Position;

    fn add(self, rhs: &Position) -> Position {
        (self.x + rhs.x, self.y + rhs.y).into()
    }
}

impl Add<Position> for &Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        (self.x + rhs.x, self.y + rhs.y).into()
    }
}

impl Add<&Position> for Position {
    type Output = Position;

    fn add(self, rhs: &Position) -> Position {
        (self.x + rhs.x, self.y + rhs.y).into()
    }
}

#[derive(EnumString, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Turn {
    Left,
    Right,
}

#[derive(EnumString, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Cardinal {
    North,
    South,
    East,
    West,
}

impl Cardinal {
    pub fn turn(self, turn: Turn) -> Cardinal {
        use crate::position::Cardinal::*;
        use crate::position::Turn::*;
        match (self, turn) {
            (North, Left) => West,
            (North, Right) => East,
            (South, Left) => East,
            (South, Right) => West,
            (East, Left) => North,
            (East, Right) => South,
            (West, Left) => South,
            (West, Right) => North,
        }
    }
}

#[derive(EnumString, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn turn(self, turn: Turn) -> Direction {
        use crate::position::Direction::*;
        match (self, turn) {
            (Up, Turn::Left) => Left,
            (Up, Turn::Right) => Right,
            (Down, Turn::Left) => Right,
            (Down, Turn::Right) => Left,
            (Right, Turn::Left) => Up,
            (Right, Turn::Right) => Down,
            (Left, Turn::Left) => Down,
            (Left, Turn::Right) => Up,
        }
    }
}

impl From<Cardinal> for Direction {
    fn from(cardinal: Cardinal) -> Self {
        use crate::position::Cardinal::*;
        use crate::position::Direction::*;
        match cardinal {
            North => Up,
            South => Down,
            East => Right,
            West => Left,
        }
    }
}

impl From<Direction> for Cardinal {
    fn from(direction: Direction) -> Self {
        use crate::position::Cardinal::*;
        use crate::position::Direction::*;
        match direction {
            Up => North,
            Down => South,
            Right => East,
            Left => West,
        }
    }
}

pub fn connect<P>(position: P) -> impl Iterator<Item = Position>
where
    P: Into<Position>,
{
    static OFFSETS: [Position; 4] = [
        Position { x: 0, y: -1 },
        Position { x: 1, y: 0 },
        Position { x: 0, y: 1 },
        Position { x: -1, y: 0 },
    ];
    let position: Position = position.into();
    OFFSETS.iter().map(move |&offset| position + offset)
}

pub fn connect8<P>(position: P) -> impl Iterator<Item = Position>
where
    P: Into<Position>,
{
    static OFFSETS: [Position; 8] = [
        Position { x: 0, y: -1 },
        Position { x: 1, y: -1 },
        Position { x: 1, y: 0 },
        Position { x: 1, y: 1 },
        Position { x: 0, y: 1 },
        Position { x: -1, y: 1 },
        Position { x: -1, y: 0 },
        Position { x: -1, y: -1 },
    ];
    let position: Position = position.into();
    OFFSETS.iter().map(move |&offset| position + offset)
}
