use std::cmp::{min, Ordering};
use std::fmt;
use std::ops::{Add, Sub};
use itertools::max;
use num::integer::Roots;
use crate::tools::usize_to_i32;

pub const NORTH: Point = Point { x: 0, y: 1};
pub const SOUTH: Point = Point { x: 0, y: -1};
pub const EAST:Point = Point { x: 1, y: 0 };
pub const WEST:Point = Point { x: -1, y: 0 };
pub const SOUTHWEST:Point = Point { x: -1, y: -1 };
pub const NORTHWEST:Point = Point { x: -1, y: 1 };
pub const SOUTHEAST:Point = Point { x: 1, y: -1 };
pub const NORTHEAST:Point = Point { x: 1, y: 1 };

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32
}
impl Point {
    #[allow(dead_code)]
    pub fn new(x: i32, y: i32) -> Self {
        return Point { x, y }
    }
    pub fn parse(x: usize, y: usize) -> Self {
        return Point { x: usize_to_i32(x).unwrap(), y: usize_to_i32(y).unwrap() }
    }
    pub fn get_neighbors(&self) -> Vec<Point> {
        let directions = [NORTH, EAST, WEST, SOUTH, NORTHEAST, NORTHWEST, SOUTHEAST, SOUTHWEST];
        return directions.iter().map(|dir| *dir + *self).collect();
    }

    pub fn get_cardinal_neighbors(&self) -> Vec<Point> {
        let directions = [NORTH, EAST, WEST, SOUTH];
        return directions.iter().map(|dir| *dir + *self).collect();
    }

    pub fn normalize(&self) -> Point {
        let mag = self.magnitude();
        return Point { x: self.x / mag, y: self.y / mag }
    }

    pub fn magnitude(&self) -> i32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

