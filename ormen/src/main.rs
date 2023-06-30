use ormen::*;

use crate::lib::Game;
use crate::lib::Player;
pub mod lib;

fn main() {
    let mut game = Game::new();
    let player = Player { wins: 0, losses: 0 };
    game.play(player);
}
