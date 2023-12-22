use std::fmt;
use std::ops::{Add, Sub};
use crate::domain::point::Point;
use crate::tools::parse_numbers;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Vector3 {
    pub fn parse(input_line: &str) -> Self {
        let [x,y,z] = input_line.split(",").map(|val| parse_numbers(val).unwrap().1).next_chunk().unwrap();
        Vector3 { x, y, z }
    }

    pub fn new(x:i32, y: i32, z: i32) -> Self {
        Vector3 { x, y, z }
    }
}
impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}



impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}
