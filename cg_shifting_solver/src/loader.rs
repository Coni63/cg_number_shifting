use std::io;

use crate::entity::action::Action;
use crate::entity::game_state::GameState;

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
    for _ in 0..height as usize {
        let mut inputs = String::new();
        io::stdin().read_line(&mut inputs).unwrap();
        let row = inputs
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        game_state.board.push(row);
    }

    game_state
}

pub fn write_output(solution: Vec<Action>) {
    solution.iter().for_each(|action| {
        println!("{}", action.to_string());
    });
}
