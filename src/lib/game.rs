use crate::lib::{
    map::Map,
    pathfinding::find_path,
    shared::{rand_range, Direction, ParsedMap, Point, Tile},
};
use std::{
    collections::HashMap,
    ops::RangeInclusive,
    sync::{Arc, RwLock},
};

#[derive(Clone)]
struct MapMeta {
    grid: HashMap<Point, (u8, (u8, u8))>,
    max: (usize, usize),
    player: (usize, usize, u8),
    props: HashMap<u8, (RangeInclusive<u8>, RangeInclusive<u8>)>,
}

type Range = (RangeInclusive<u8>, RangeInclusive<u8>);

#[allow(dead_code)]
#[derive(Clone)]
pub struct Game {
    current_map: (u8, u8),
    dialogue: Option<u8>,
    entries: HashMap<(u8, u8), Point>,
    pub map: Map,
    maps: HashMap<(u8, u8), MapMeta>,
    splash: Option<u8>,
    pub pathfinding: Arc<RwLock<HashMap<(u8, u8, u8), Vec<Point>>>>,
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

        let mut game = Game {
            current_map,
            dialogue: None,
            entries: HashMap::new(),
            map: Map::parse_map(&map.grid, &map.max, &map.player, &map.props),
            maps,
            splash: None,
            pathfinding: Arc::new(RwLock::new(HashMap::new())),
        };

        game.new_path_for_npc();

        game
    }

    fn gen_point(&self, range: &Range, iteration: usize) -> Point {
        let point = Point {
            x: rand_range(&range.0) as usize,
            y: rand_range(&range.1) as usize,
        };

        if self.map.grid.get(&point) == Some(&Tile::Empty) || iteration > 10 {
            return point;
        }

        self.gen_point(range, iteration + 1)
    }

    pub fn new_path_for_npc(&mut self) {
        let mut pathfinding = self.pathfinding.write().unwrap();
        for npc in &self.map.npc {
            let key = (npc.0.to_owned(), self.current_map.0, self.current_map.1);
            if let Some(val) = pathfinding.get(&key) {
                if !val.is_empty() {
                    continue;
                }
            }

            if let Some(meta) = self.map.props.get(npc.0) {
                let point = self.gen_point(&meta, 0);
                let path = find_path(&self.map.grid, npc.1.to_owned(), point.to_owned());

                if path.is_empty() {
                    continue;
                }

                pathfinding.insert(key, path.to_owned());
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
                            let map = Map::parse_map(
                                &new_map_meta.grid,
                                &new_map_meta.max,
                                &player,
                                &new_map_meta.props,
                            );

                            let mut pathfinding = self.pathfinding.write().unwrap();
                            pathfinding.clear();
                            drop(pathfinding);

                            self.entries.insert(self.current_map, self.map.current);
                            self.current_map = meta.to_owned();
                            self.map = map;
                            self.new_path_for_npc();
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

    pub fn move_npc(&mut self, meta: &(u8, u8, u8), point: &Point) {
        if self.current_map != (meta.1, meta.2) {
            return;
        }

        if let Some(npc) = self.map.npc.get_mut(&meta.0) {
            let is_not_occupied = self.map.grid.get(point).unwrap() == &Tile::Empty;

            let key = meta.to_owned();

            if is_not_occupied {
                let tile = self.map.grid.get_mut(npc).unwrap();
                *tile = Tile::Empty;
                let tile = self.map.grid.get_mut(point).unwrap();
                *tile = Tile::NPC;
                *npc = point.to_owned();
                self.map.print_grid();

                let mut calculate_new = false;

                if let Ok(pathfinding) = self.pathfinding.read() {
                    if let Some(npc) = pathfinding.get(&key) {
                        if npc.len() == 0 {
                            calculate_new = true;
                        }
                    }
                }

                if calculate_new {
                    self.new_path_for_npc();
                }

                return;
            }

            drop(npc);
            drop(is_not_occupied);

            if let Ok(guard) = self.pathfinding.write() {
                let mut pathfinding = guard;
                if let Some(npc) = pathfinding.get_mut(&key) {
                    npc.clear()
                }
            }

            self.new_path_for_npc();
        }
    }
}
