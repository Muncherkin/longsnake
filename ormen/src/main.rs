use ormen::*;

use crate::lib::Game;
use crate::lib::Player;
pub mod lib;

fn main() {
    let mut game;
    let mut player = Player { wins: 0, losses: 0 };
    for i in 1..10001 {
        game = Game::new();
        let won = game.play(&player);
        if won {
            player.wins += 1;
        } else {
            player.losses += 1;
        }
        if i % 1000 == 0 {
            println!("Wins: {}", player.wins);
            println!("Losses: {}", player.losses);
        }
    }
}
