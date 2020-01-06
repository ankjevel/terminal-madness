#[macro_use]
extern crate lazy_static;
extern crate rand;

mod lib;

use lib::{game::Game, helper::parse_maps, shared::Point};
use rand::Rng;
use std::{
    cmp::max,
    io::{stdout, Read},
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use termion::{async_stdin, clear, cursor, raw::IntoRawMode};

enum Message {
    MoveNPC { meta: (u8, u8, u8), point: Point },
    MovePlayer { key: u8 },
    Interact,
}

fn main() {
    let (tx, rx) = mpsc::channel();

    let maps = parse_maps(&include_str!("../lib/maps"));
    let game = Arc::new(Mutex::new(Game::new(maps)));

    let mut stdin = async_stdin().bytes();
    let _stdout = stdout().into_raw_mode().unwrap();

    game.lock().unwrap().map.print_grid();

    let pathfinding = game.lock().unwrap().pathfinding.clone();
    let in_thread_tx = tx.clone();

    thread::spawn(move || loop {
        let start = Instant::now();
        let mut movement = Vec::new();
        if let Ok(guard) = pathfinding.write() {
            let mut guard = guard;
            for (meta, points) in guard.iter_mut() {
                if points.is_empty() {
                    continue;
                }

                let point = points.pop();

                if point.is_none() {
                    continue;
                }

                let point = point.unwrap();
                movement.push((meta.to_owned(), point.to_owned()));
            }
        };

        for (meta, point) in movement {
            let range = rand::thread_rng().gen_range(50, 250);
            thread::sleep(Duration::from_millis(range as u64));

            in_thread_tx
                .send(Message::MoveNPC {
                    meta: meta.to_owned(),
                    point: point.to_owned(),
                })
                .unwrap();
        }

        let duration = start.elapsed().as_millis();

        let diff = if duration < 1000 { 1000 - duration } else { 0 };

        thread::sleep(Duration::from_millis(diff as u64));
    });

    thread::spawn(move || loop {
        let msg = rx.recv().unwrap();
        if let Ok(guard) = game.try_lock() {
            let mut this = guard;
            match msg {
                Message::MoveNPC { meta, point } => this.move_npc(&meta, &point),
                Message::MovePlayer { key } => this.move_player(&key),
                Message::Interact => this.interact(),
            }
        }
    });

    let input_loop_tx = tx.clone();

    'stdin: loop {
        let start = Instant::now();
        if let Some(Ok(val)) = stdin.next() {
            match val {
                // arrow sequence = 27+91+(65-68)
                27 => {
                    if let Some(Ok(val)) = stdin.next() {
                        if val == 91 {
                            input_loop_tx
                                .send(Message::MovePlayer {
                                    key: stdin.next().unwrap_or(Ok(0)).unwrap_or(0),
                                })
                                .unwrap();
                            continue 'stdin;
                        }
                    }
                }
                // ctrl+c
                3 => break 'stdin,
                // enter
                13 => print!("enter\r\n"),
                // space
                32 => input_loop_tx.send(Message::Interact).unwrap(),
                _ => {}
            }
        }

        let duration = start.elapsed().as_millis();

        thread::sleep(Duration::from_millis(max(100 - duration, 0) as u64));
    }

    println!("{}{}{}", clear::All, cursor::Show, cursor::Goto(1, 1));
}
