mod lib;

use lib::{game::Game, helper::parse_input};

fn main() {
    let input = parse_input(&include_str!("../ext/grid"));

    Game::test(&input).run();
}
