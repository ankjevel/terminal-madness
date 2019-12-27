use crate::lib::map::{Direction, Map, Tile};
use std::{
    io::{stdout, Read},
    thread,
    time::Duration,
};
use termion::{async_stdin, clear, cursor, raw::IntoRawMode};

pub struct Game {
    map: Map,
}

impl Game {
    pub fn test(input: &Vec<Vec<u8>>) -> Game {
        Game {
            map: Map::test(input),
        }
    }

    fn move_player(&mut self, input: &u8) {
        let mut direction = self.map.direction.to_owned();
        let current = self.map.current.to_owned();
        let mut point = current.clone();
        match input {
            65 => {
                if direction == Direction::Up && point.y > 0 {
                    point.y -= 1
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

        if let Some(tile) = self.map.grid.get_mut(&point) {
            if tile == &Tile::Empty {
                *tile = Tile::Current;

                *self.map.grid.get_mut(&current).unwrap() = Tile::Empty;
                self.map.current = point.to_owned();
            }
        }

        self.map.print_grid();
    }

    fn interact(&mut self) {
        print!("space!\r\n");
    }

    pub fn run(&mut self) {
        let mut stdin = async_stdin().bytes();
        let _stdout = stdout().into_raw_mode().unwrap();

        println!("{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Hide);

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

        println!("{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Restore);
    }
}
