#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate float_cmp;
extern crate rand;

mod lib;

use lib::{game::Game, helper::parse_maps};
use std::{
    io::{stdout, Read},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use termion::{async_stdin, clear, cursor, raw::IntoRawMode};

fn main() {
    let maps = parse_maps(&include_str!("../lib/maps"));
    let game = Arc::new(Mutex::new(Game::new(maps)));
    let mut stdin = async_stdin().bytes();
    let _stdout = stdout().into_raw_mode().unwrap();

    game.lock().unwrap().map.print_grid();

    let pathfinding = game.lock().unwrap().pathfinding.clone();

    let in_thread = game.clone();

    thread::spawn(move || loop {
        for path in pathfinding.lock().unwrap().iter_mut() {
            let point = path.1.pop();
            if point.is_none() {
                continue;
            }
            let point = point.unwrap();
            let mut game = in_thread.lock().unwrap();
            game.move_actor(path.0, &point);
        }

        thread::sleep(Duration::from_millis(2500));
    });

    'stdin: loop {
        if let Some(Ok(val)) = stdin.next() {
            match val {
                // arrow sequence = 27+91+(65-68)
                27 => {
                    if let Some(Ok(val)) = stdin.next() {
                        if val == 91 {
                            let mut this = game.lock().unwrap();
                            this.move_player(&stdin.next().unwrap_or(Ok(0)).unwrap_or(0));
                            continue 'stdin;
                        }
                    }
                }
                // ctrl+c
                3 => break 'stdin,
                // enter
                13 => print!("enter\r\n"),
                // space
                32 => {
                    let mut this = game.lock().unwrap();
                    this.interact()
                }
                // n
                110 => {
                    let mut this = game.lock().unwrap();
                    this.new_path_for_npc()
                }
                _ => print!("{}\r\n", val),
            }
        }

        thread::sleep(Duration::from_millis(100));
    }

    println!("{}{}{}", clear::All, cursor::Show, cursor::Goto(1, 1));
}
