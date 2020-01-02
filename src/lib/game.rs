use crate::lib::{
    map::Map,
    pathfinding::find_path,
    shared::{rand_range, Direction, ParsedMap, Point, Tile},
};
use std::{
    collections::HashMap,
    ops::RangeInclusive,
    sync::{Arc, Mutex},
};

#[derive(Clone, Eq, PartialEq, Debug)]
struct MapMeta {
    grid: HashMap<Point, (u8, (u8, u8))>,
    max: (usize, usize),
    player: (usize, usize, u8),
    props: HashMap<u8, (RangeInclusive<u8>, RangeInclusive<u8>)>,
}

type Range = (RangeInclusive<u8>, RangeInclusive<u8>);

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Game {
    current_map: (u8, u8),
    dialogue: Option<u8>,
    entries: HashMap<(u8, u8), Point>,
    pub map: Map,
    maps: HashMap<(u8, u8), MapMeta>,
    splash: Option<u8>,
    pub pathfinding: Arc<Mutex<HashMap<u8, Vec<Point>>>>,
}

impl Game {
    pub fn new(input: Vec<ParsedMap>) -> Game {
        let mut maps = HashMap::new();
        for parsed in input.into_iter() {
            maps.insert(
                (parsed.area, parsed.part),
                MapMeta {
                    max: parsed.max,
                    grid: parsed.grid,
                    player: parsed.player,
                    props: parsed.props,
                },
            );
        }

        let no_match = MapMeta {
            max: (0, 0),
            grid: HashMap::new(),
            player: (0, 0, 0),
            props: HashMap::new(),
        };

        let current_map = (0, 0);

        let map = match maps.get(&current_map) {
            Some(meta) => meta,
            None => &no_match,
        };

        Game {
            current_map,
            dialogue: None,
            entries: HashMap::new(),
            map: Map::parse_map(&map.grid, &map.max, &map.player, &map.props),
            maps,
            splash: None,
            pathfinding: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn new_path_for_npc(&mut self) {
        let gen_point = |range: &Range| -> Point {
            Point {
                x: rand_range(&range.0) as usize,
                y: rand_range(&range.1) as usize,
            }
        };

        let mut pathfinding = self.pathfinding.lock().unwrap();
        for npc in &self.map.npc {
            if let Some(val) = pathfinding.get(npc.0) {
                print!("val: {:?}\r\n", val);
                continue;
            }

            if let Some(meta) = self.map.props.get(npc.0) {
                let point = gen_point(&meta);
                let path = find_path(&self.map.grid, npc.1.to_owned(), point.to_owned());

                if path.is_none() {
                    continue;
                }

                let path = path.unwrap();

                pathfinding.insert(*npc.0, path.to_owned());

                print!("non yet for {:?}\r\n{:?}\r\n{:?}\r\n", npc, point, path);
            }
        }
    }

    pub fn move_player(&mut self, input: &u8) {
        let mut direction = self.map.direction.to_owned();
        let current = self.map.current.to_owned();
        let mut point = current.clone();
        match input {
            65 => {
                if direction == Direction::Up && point.y > 0 {
                    point.y -= 1;
                }
                direction = Direction::Up;
            }
            66 => {
                if direction == Direction::Down {
                    point.y += 1;
                }
                direction = Direction::Down;
            }
            67 => {
                if direction == Direction::Right {
                    point.x += 1;
                }
                direction = Direction::Right;
            }
            68 => {
                if direction == Direction::Left && point.x > 0 {
                    point.x -= 1;
                }
                direction = Direction::Left;
            }
            _ => {}
        };

        if self.map.direction != direction {
            self.map.direction = direction;
        }

        if point == current {
            self.map.print_grid();
            return;
        }

        if let Some(tile) = self.map.grid.get_mut(&point) {
            match tile {
                Tile::Empty => {
                    *tile = Tile::Current;

                    *self.map.grid.get_mut(&current).unwrap() = Tile::Empty;
                    self.map.current = point.to_owned();
                }
                Tile::Warp => {
                    if let Some(meta) = self.map.meta.get(&point) {
                        if let Some(new_map_meta) = self.maps.get(&meta) {
                            let player = match self.entries.get(&meta) {
                                Some(point) => (point.x, point.y, self.map.direction.to_u8()),
                                None => new_map_meta.player,
                            };
                            let map = Some(Map::parse_map(
                                &new_map_meta.grid,
                                &new_map_meta.max,
                                &player,
                                &new_map_meta.props,
                            ));
                            self.entries.insert(self.current_map, self.map.current);
                            self.current_map = meta.to_owned();
                            self.map = map.unwrap();
                        }
                    }
                }
                _ => {}
            }
        }

        self.map.print_grid();
    }

    pub fn interact(&mut self) {
        let mut looking_at = self.map.current.to_owned();

        match self.map.direction {
            Direction::Down => looking_at.y += 1,
            Direction::Left => looking_at.x -= 1,
            Direction::Right => looking_at.x += 1,
            Direction::Up => looking_at.y -= 1,
        };

        if let Some(tile) = self.map.grid.get(&looking_at) {
            print!("you stand next to \"{:?}\"\r\n", tile);
        }
    }

    pub fn move_actor(&mut self, id: &u8, point: &Point) {
        print!("{}={:?}\r\n", id, point);
    }
}
