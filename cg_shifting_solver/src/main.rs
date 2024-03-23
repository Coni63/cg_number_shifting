mod entity;
mod loader;

use entity::game_state::GameState;

fn main() {
    let mut game = loader::read_input();

    // game.print();

    let won = game.step();

    game.actions_taken.iter().for_each(|action| {
        println!("{}", action.to_string());
    });

    // println!("7 4 L +");
    // println!("3 0 D -");
    // println!("6 4 L -");
}
