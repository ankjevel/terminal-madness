use crate::lib::{
    map::{Direction, Map, Tile},
    shared::{MapStruct, Point},
};
use std::{
    collections::HashMap,
    io::{stdout, Read},
    thread,
    time::Duration,
};
use termion::{async_stdin, clear, cursor, raw::IntoRawMode};

#[derive(Clone, Eq, PartialEq, Debug)]
struct MapMeta {
    grid: HashMap<Point, (u8, (u8, u8))>,
    max: (usize, usize),
    player: (usize, usize, u8),
}

#[allow(dead_code)]
pub struct Game {
    map: Map,
    maps: HashMap<(u8, u8), MapMeta>,
}

impl Game {
    pub fn new(input: Vec<MapStruct>) -> Game {
        let mut maps = HashMap::new();
        for (area_and_part, grid, max, player) in input.into_iter() {
            maps.insert(
                area_and_part.to_owned(),
                MapMeta {
                    max: max.to_owned(),
                    grid: grid.to_owned(),
                    player: player.to_owned(),
                },
            );
        }

        let no_match = MapMeta {
            max: (0, 0),
            grid: HashMap::new(),
            player: (0, 0, 0),
        };

        let map = match maps.get(&(0, 0)) {
            Some(meta) => meta,
            None => &no_match,
        }
        .to_owned();

        Game {
            map: Map::parse_map(&map.grid, &map.max, &map.player),
            maps,
        }
    }

    fn move_player(&mut self, input: &u8) {
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
                    let mut map = None;

                    if let Some(meta) = self.map.meta.get(&point) {
                        if let Some(new_map_meta) = self.maps.get(&meta) {
                            map = Some(Map::parse_map(
                                &new_map_meta.grid,
                                &new_map_meta.max,
                                &new_map_meta.player,
                            ))
                        }
                    }

                    if map.is_some() {
                        self.map = map.unwrap();
                    }
                }
                _ => {}
            }
        }

        self.map.print_grid();
    }

    fn interact(&mut self) {
        let mut looking_at = self.map.current.to_owned();

        match self.map.direction {
            Direction::Down => looking_at.y += 1,
            Direction::Left => looking_at.x -= 1,
            Direction::Right => looking_at.x += 1,
            Direction::Up => looking_at.y -= 1,
        };

        if let Some(tile) = self.map.grid.get(&looking_at) {
            print!("you are looking at \"{:?}\"\r\n", tile);
        }
    }

    pub fn run(&mut self) {
        let mut stdin = async_stdin().bytes();
        let _stdout = stdout().into_raw_mode().unwrap();

        self.map.print_grid();

        'stdin: loop {
            if let Some(Ok(val)) = stdin.next() {
                match val {
                    // arrow sequence = 27+91+(65-68)
                    27 => {
                        if let Some(Ok(val)) = stdin.next() {
                            if val == 91 {
                                self.move_player(&stdin.next().unwrap_or(Ok(0)).unwrap_or(0));
                                continue 'stdin;
                            }
                        }
                    }
                    // ctrl+c
                    3 => break 'stdin,
                    // space
                    32 => self.interact(),
                    _ => {}
                }
            }

            thread::sleep(Duration::from_millis(100));
        }

        println!("{}{}{}", clear::All, cursor::Show, cursor::Goto(1, 1));
    }
}
