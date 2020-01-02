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
    rand::thread_rng().gen_range(input.start(), input.end() + 1)
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

    pub fn from_u8(input: u8) -> Direction {
        match input {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Right,
            3 | _ => Direction::Left,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_render_expected_random_numbers() {
        let range = 0..=10;
        for _ in 0..1000 {
            let rand = rand_range(&range);
            assert!(range.contains(&rand));
        }
    }
}
