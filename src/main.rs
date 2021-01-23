use std::process;

mod game;

use game::Game;

fn main() {

    let mut game = Game::new().unwrap_or_else(|err| {
        eprintln!("Error creating game: {}", err);
        process::exit(1);
    });

    if let Err(err) = game.run() {
        eprintln!("Error while running game: {}", err);
        process::exit(1);
    };
}