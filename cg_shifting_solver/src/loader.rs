use std::io;

use crate::entity::{game_state::GameState, tile::Tile};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

pub fn read_input() -> GameState {
    let mut game_state = GameState::new();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let _width = parse_input!(inputs[0], i32);
    let height = parse_input!(inputs[1], i32);
    for i in 0..height as usize {
        let mut inputs = String::new();
        io::stdin().read_line(&mut inputs).unwrap();
        for (j, val) in inputs.split_whitespace().enumerate() {
            let cell = parse_input!(val, i32);
            if cell != 0 {
                game_state.tiles.push(Tile {
                    value: cell,
                    row: i as u8,
                    col: j as u8,
                });
            }
        }
    }

    game_state
}
