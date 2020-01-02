#[macro_use]
extern crate lazy_static;

mod lib;

use lib::{game::Game, helper::parse_maps};

fn main() {
    let maps = parse_maps(&include_str!("../lib/maps"));
    Game::new(maps).run();
}
