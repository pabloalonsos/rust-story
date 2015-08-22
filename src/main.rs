extern crate sdl2;

mod game;
mod graphics;
mod events;

use self::game::Game;

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let mut new_game: Game = Game::new(&sdl_context);

    new_game.start();

}
