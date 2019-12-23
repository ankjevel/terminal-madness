use crate::lib::shared::Point;
use std::collections::HashMap;
use termion::{clear, cursor};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Tile {
    Current,
    Unknown,
    Wall,
    Empty,
}

impl Tile {
    pub fn from_str(input: &str) -> Tile {
        match input {
            "0" => Tile::Wall,
            "1" => Tile::Empty,
            "2" => Tile::Current,
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

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Map {
    pub grid: HashMap<Point, Tile>,
    pub current: Point,
    pub direction: Direction,
}

impl Map {
    pub fn test(input: &Vec<Vec<u8>>) -> Map {
        let mut grid = HashMap::new();
        let mut current = Point { x: 0, y: 0 };
        for (y, row) in input.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let point = Point { x, y };
                let tile = Tile::from_u8(tile);

                if tile == Tile::Current {
                    current = point.to_owned();
                }

                grid.insert(point, tile);
            }
        }

        Map {
            grid,
            current,
            direction: Direction::Right,
        }
    }

    pub fn print_grid(&mut self) -> String {
        let (max_x, max_y) = self.get_grid();
        let mut grid = vec![vec![' '; max_x + 1]; max_y + 1];
        for (point, tile) in self.grid.clone() {
            let (x, y) = (point.x, point.y);

            if tile == Tile::Unknown {
                continue;
            }

            if let Some(elem) = grid.get_mut(y) {
                elem[x] = match tile {
                    Tile::Current => match self.direction {
                        Direction::Left => '←',
                        Direction::Right => '→',
                        Direction::Up => '↑',
                        Direction::Down => '↓',
                    },
                    Tile::Wall => '█',
                    Tile::Empty => ' ',
                    // Tile::Enemy => '░',
                    _ => ' ',
                };
            }
        }

        format!(
            "{}{}",
            clear::All,
            grid.iter()
                .enumerate()
                .map(|(y, row)| {
                    format!(
                        "{}{}",
                        cursor::Goto(1, (y as u16) + 1),
                        row.clone().iter().collect::<String>()
                    )
                })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }

    fn get_grid(&self) -> (usize, usize) {
        let grid = self.grid.clone();

        let (x, y) = grid.iter().fold((0, 0), |mut acc, (point, _tile)| {
            let (x, y) = (point.x, point.y);
            if acc.0 < x {
                acc.0 = x.to_owned();
            }
            if acc.1 < y {
                acc.1 = y.to_owned();
            }
            acc
        });

        (x, y)
    }
}
