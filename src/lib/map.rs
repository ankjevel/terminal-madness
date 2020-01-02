use crate::lib::{
    helper::with_color,
    shared::{Direction, Point, Tile},
};
use std::{
    collections::{BTreeMap, HashMap},
    ops::RangeInclusive,
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

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Map {
    pub grid: HashMap<Point, Tile>,
    pub current: Point,
    pub npc: HashMap<u8, Point>,
    pub direction: Direction,
    pub meta: HashMap<Point, (u8, u8)>,
    pub props: HashMap<u8, (RangeInclusive<u8>, RangeInclusive<u8>)>,
}

impl Map {
    pub fn parse_map(
        input: &HashMap<Point, (u8, (u8, u8))>,
        map: &(usize, usize),
        player: &(usize, usize, u8),
        props: &HashMap<u8, (RangeInclusive<u8>, RangeInclusive<u8>)>,
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
        let mut npc = HashMap::new();

        for (point, (tile, tile_meta)) in input {
            if let Some(grid) = grid.get_mut(&point) {
                let parsed_tile = Tile::from_u8(tile);
                *grid = parsed_tile.to_owned();

                if parsed_tile == Tile::NPC && tile_meta.1 == 0 {
                    npc.insert(tile_meta.0.to_owned(), point.to_owned());
                }

                meta.insert(point.to_owned(), tile_meta.to_owned());
            }
        }

        grid.insert(current.to_owned(), Tile::Current);

        Map {
            grid,
            current,
            meta,
            npc,
            props: props.to_owned(),
            direction: Direction::from_u8(player.2),
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
