use rand::Rng;
use std::{
    cmp::Ordering::{self, Equal, Greater, Less},
    collections::HashMap,
    ops::RangeInclusive,
};

pub struct ParsedMap {
    pub area: u8,
    pub part: u8,
    pub grid: HashMap<Point, (u8, (u8, u8))>,
    pub max: (usize, usize),
    pub player: (usize, usize, u8),
    pub props: HashMap<u8, (RangeInclusive<u8>, RangeInclusive<u8>)>,
}

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

pub fn rand_range(input: &RangeInclusive<u8>) -> u8 {
    rand::thread_rng().gen_range(input.start(), input.end())
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Tile {
    Wall,
    Empty,
    Current,
    Warp,
    NPC,
    Unknown,
}

impl Tile {
    pub fn from_str(input: &str) -> Tile {
        match input {
            "0" => Tile::Wall,
            "1" => Tile::Empty,
            "2" => Tile::Current,
            "3" => Tile::Warp,
            "4" => Tile::NPC,
            _ => Tile::Unknown,
        }
    }

    pub fn from_u8(input: &u8) -> Tile {
        Tile::from_str(&input.to_string())
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
#[allow(dead_code)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn to_u8(&self) -> u8 {
        match self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Right => 2,
            Direction::Left => 3,
        }
    }
}
