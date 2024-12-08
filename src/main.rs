use crate::game::Game;

mod game;
mod board;
mod ship;

fn main() {

    println!("Loading...");

    //game struct is the game wrapper
    let game = Game::new();
    //run the game
    game.main();
    
}
