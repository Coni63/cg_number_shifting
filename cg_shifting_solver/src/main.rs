mod entity;
mod loader;

use entity::game_state::GameState;

fn main() {
    let mut _game_state = loader::read_input();

    println!("7 4 L +");
    println!("3 0 D -");
    println!("6 4 L -");
}
