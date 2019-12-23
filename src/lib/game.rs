use crate::lib::map::{Direction, Map, Tile};
use std::io::{stdin, stdout, Write};
use termion::{clear, cursor, event::Key, input::TermRead, raw::IntoRawMode};

pub struct Game {
    map: Map,
}

impl Game {
    pub fn test(input: &Vec<Vec<u8>>) -> Game {
        Game {
            map: Map::test(input),
        }
    }

    fn move_player(&mut self, input: Key) {
        let mut direction = self.map.direction.to_owned();
        let current = self.map.current.to_owned();
        let mut point = current.clone();
        match input {
            Key::Left => {
                if direction == Direction::Left && point.x > 0 {
                    point.x -= 1;
                }
                direction = Direction::Left;
            }
            Key::Up => {
                if direction == Direction::Up && point.y > 0 {
                    point.y -= 1
                }
                direction = Direction::Up;
            }
            Key::Right => {
                if direction == Direction::Right {
                    point.x += 1;
                }
                direction = Direction::Right;
            }
            Key::Down => {
                if direction == Direction::Down {
                    point.y += 1;
                }
                direction = Direction::Down;
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

        println!("{}{}", self.map.print_grid(), cursor::Goto(1, 1));
    }

    pub fn run(&mut self) {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        println!("{}{}", self.map.print_grid(), cursor::Goto(1, 1));

        stdout.flush().unwrap();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') | Key::Esc | Key::Char('c') => break,
                Key::Char(' ') => println!("space"),
                c => self.move_player(c),
            }

            stdout.flush().unwrap();
        }

        println!("{}{}", clear::All, cursor::Goto(1, 1));
    }
}
