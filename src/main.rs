mod lib;

use lib::{game::Game, helper::parse_input};

fn main() {
    let output = parse_input(&include_str!("../lib/maps"));
    Game::new(output).run();
}
