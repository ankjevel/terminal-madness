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
    MoveNPC { id: u8, point: Point },
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
            for path in guard.iter_mut() {
                if path.1.is_empty() {
                    continue;
                }

                let point = path.1.pop();

                if point.is_none() {
                    continue;
                }

                let point = point.unwrap();
                movement.push((path.0.to_owned(), point.to_owned()));
            }
        };

        for (id, point) in movement {
            let range = rand::thread_rng().gen_range(50, 250);
            thread::sleep(Duration::from_millis(range as u64));
            in_thread_tx
                .send(Message::MoveNPC {
                    id: id.to_owned(),
                    point: point.to_owned(),
                })
                .unwrap();
        }

        let duration = start.elapsed().as_millis();

        let diff = max(2000 - duration, 0) as u64;

        thread::sleep(Duration::from_millis(diff));
    });

    thread::spawn(move || loop {
        let msg = rx.recv().unwrap();
        if let Ok(guard) = game.lock() {
            let mut this = guard;
            match msg {
                Message::MoveNPC { id, point } => this.move_npc(&id, &point),
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
                            let key = &stdin.next().unwrap_or(Ok(0)).unwrap_or(0);
                            input_loop_tx
                                .send(Message::MovePlayer {
                                    key: key.to_owned(),
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
                32 => {
                    input_loop_tx.send(Message::Interact).unwrap();
                }
                _ => {}
            }
        }

        let duration = start.elapsed().as_millis();

        thread::sleep(Duration::from_millis(max(100 - duration, 0) as u64));
    }

    println!("{}{}{}", clear::All, cursor::Show, cursor::Goto(1, 1));
}
