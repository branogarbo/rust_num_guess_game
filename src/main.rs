mod game;
use game::Game;

fn main() {
    let mut guesser = Game::new();

    guesser.run();
}
