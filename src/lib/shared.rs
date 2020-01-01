use std::{
    cmp::Ordering::{self, Equal, Greater, Less},
    collections::HashMap,
};

pub type MapStruct = (
    (u8, u8),
    HashMap<Point, (u8, (u8, u8))>,
    (usize, usize),
    (usize, usize, u8),
);

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.y == other.y {
            if self.x < other.x {
                Less
            } else if self.x > other.x {
                Greater
            } else {
                Equal
            }
        } else if self.y < other.y {
            Less
        } else if self.y > other.y {
            Greater
        } else {
            if self.x < other.x {
                Less
            } else if self.x > other.x {
                Greater
            } else {
                Equal
            }
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
