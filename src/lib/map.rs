use crate::lib::{helper::with_color, shared::Point};
use std::{
    collections::{BTreeMap, HashMap},
    string::String,
};
use termion::{clear, color, cursor};

lazy_static! {
    static ref TILE_WARP: String = with_color("░", color::Yellow);
    static ref TILE_DIRECTION_LEFT: String = with_color("←", color::Green);
    static ref TILE_DIRECTION_RIGHT: String = with_color("→", color::Green);
    static ref TILE_DIRECTION_UP: String = with_color("↑", color::Green);
    static ref TILE_DIRECTION_DOWN: String = with_color("↓", color::Green);
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

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Map {
    pub grid: HashMap<Point, Tile>,
    pub current: Point,
    pub direction: Direction,
    pub meta: HashMap<Point, (u8, u8)>,
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

fn print_tile_current(direction: &Direction) -> String {
    match direction {
        Direction::Left => TILE_DIRECTION_LEFT.to_string(),
        Direction::Right => TILE_DIRECTION_RIGHT.to_string(),
        Direction::Up => TILE_DIRECTION_UP.to_string(),
        Direction::Down => TILE_DIRECTION_DOWN.to_string(),
    }
}

fn join<'a>(a: String, b: String) -> String {
    let (a, b) = (a.to_owned(), b.to_owned());
    let c = [a, b].concat();

    c.to_string()
}

impl Map {
    pub fn parse_map(
        input: &HashMap<Point, (u8, (u8, u8))>,
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

        let mut meta = HashMap::new();

        for (point, (tile, tile_meta)) in input {
            if let Some(grid) = grid.get_mut(&point) {
                *grid = Tile::from_u8(tile);

                meta.insert(point.to_owned(), tile_meta.to_owned());
            }
        }

        grid.insert(current.to_owned(), Tile::Current);

        Map {
            grid,
            current,
            meta,
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

            string = join(
                string,
                match tile {
                    Tile::Current => print_tile_current(&self.direction),
                    Tile::Wall => "█".to_string(),
                    Tile::Empty => " ".to_string(),
                    Tile::Warp => TILE_WARP.to_string(),
                    Tile::NPC => "X".to_string(),
                    _ => " ".to_string(),
                },
            )
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
