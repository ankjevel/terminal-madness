use crate::lib::shared::Point;
use std::collections::{BTreeMap, HashMap};
use termion::{clear, color, cursor};

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

fn grid_as_tree_map(
    grid: &HashMap<Point, Tile>,
    max_x: &usize,
    max_y: &usize,
) -> BTreeMap<Point, Tile> {
    let mut tree_map = BTreeMap::new();
    for y in 0..=(max_y + 1) {
        for x in 0..=(max_x + 1) {
            let point = Point { x, y };
            let key = grid.get(&point).unwrap_or(&Tile::Unknown);
            tree_map.insert(point, key.to_owned());
        }
    }
    tree_map
}

impl Map {
    pub fn parse_map(
        input: &HashMap<Point, u8>,
        map: &(usize, usize),
        player: &(usize, usize, u8),
    ) -> Map {
        let mut grid = HashMap::new();
        let current = Point {
            x: player.0,
            y: player.1,
        };

        for y in 0..map.1 {
            for x in 0..map.0 {
                let point = Point { x, y };
                let tile = Tile::Empty;
                grid.insert(point, tile);
            }
        }

        for (point, tile) in input {
            if let Some(grid) = grid.get_mut(&point) {
                *grid = Tile::from_u8(tile);
            }
        }

        grid.insert(current.to_owned(), Tile::Current);

        Map {
            grid,
            current,
            direction: match player.2 {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Right,
                3 | _ => Direction::Left,
            },
        }
    }

    pub fn print_grid(&mut self) {
        let (max_x, max_y) = self.get_grid();
        let mut current = 0;
        let mut string = "".to_string();
        let mut out = Vec::new();
        for (point, tile) in grid_as_tree_map(&self.grid, &max_x, &max_y) {
            if point.y != current {
                out.push(string.to_owned());
                string = "".to_string();
                current = point.y.to_owned();
            }

            string = [
                string,
                match tile {
                    Tile::Current => format!(
                        "{}{}{}",
                        color::Fg(color::Green),
                        match self.direction {
                            Direction::Left => "←",
                            Direction::Right => "→",
                            Direction::Up => "↑",
                            Direction::Down => "↓",
                        },
                        color::Fg(color::Reset)
                    )
                    .to_string(),
                    Tile::Wall => "█".to_string(),
                    Tile::Empty => " ".to_string(),
                    // Tile::Enemy => "░",
                    _ => " ".to_string(),
                },
            ]
            .concat()
        }

        print!(
            "{}{}{}{}\r\n",
            clear::All,
            cursor::Goto(1, 1),
            cursor::Hide,
            out.join("\r\n")
        );
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
