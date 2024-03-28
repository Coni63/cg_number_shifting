use std::io;

use crate::entity::action::Action;
use crate::entity::game_state::GameState;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

pub fn read_input() -> GameState {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let width = parse_input!(inputs[0], usize);
    let height = parse_input!(inputs[1], usize);

    let mut board: Vec<Vec<u16>> = Vec::new();
    for _ in 0..height {
        let mut inputs = String::new();
        io::stdin().read_line(&mut inputs).unwrap();
        let row = inputs
            .split_whitespace()
            .map(|x| x.parse::<u16>().unwrap())
            .collect::<Vec<u16>>();
        board.push(row);
    }

    GameState::new(width, height, board)
}

pub fn write_output(solution: Vec<Action>) {
    solution.iter().for_each(|action| {
        println!("{}", action.to_string());
    });
}
