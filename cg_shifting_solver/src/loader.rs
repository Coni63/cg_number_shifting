use std::io;

use crate::constants::{NUM_ACTIONS, NUM_COLS, NUM_ROWS};
use crate::game_state::GameState;
use crate::solver::Solution;

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
    let mut count_tile = 0;

    let mut board: [[i16; NUM_COLS]; NUM_ROWS] = [[0; NUM_COLS]; NUM_ROWS];

    for i in 0..height {
        let mut inputs = String::new();
        io::stdin().read_line(&mut inputs).unwrap();
        for (j, value) in inputs
            .split_whitespace()
            .map(|x| x.parse::<i16>().unwrap())
            .enumerate()
        {
            count_tile += if value > 0 { 1 } else { 0 };
            board[i][j] = value;
        }
    }

    GameState::new(width, height, count_tile, board)
}

pub fn write_output(solution: Solution) {
    for (row, col, dir, op) in solution.actions {
        if row == 100 {
            break;
        }
        let dir_sign = ["U", "D", "L", "R"][dir as usize];
        let op_sign = if op { "+" } else { "-" };
        println!("{} {} {} {}", col, row, dir_sign, op_sign);
    }
}
