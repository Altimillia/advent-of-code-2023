use std::fmt;
use std::ops::{Add, Sub};
use crate::domain::point::Point;
use crate::tools::{parse_numbers, parse_numbers_f64};

#[derive(Debug, PartialEq, Clone, Copy, PartialOrd)]
pub struct Vector3Large {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3Large {
    pub fn parse(input_line: &str) -> Self {
        println!("{}", input_line);
        let [x,y,z] = input_line.split(",").map(|val| parse_numbers_f64(val.trim()).unwrap().1).next_chunk().unwrap();
        Vector3Large { x, y, z }
    }

    pub fn new(x:f64, y: f64, z: f64) -> Self {
        Vector3Large { x, y, z }
    }
}
impl Add for Vector3Large {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}



impl Sub for Vector3Large {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl fmt::Display for Vector3Large {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}
