use std::hash::Hash;
use std::hash::Hasher;

use crate::entity::action::Action;
use crate::entity::enums::{Direction, Metric, Operation};
use fxhash::FxHasher;

pub struct GameState {
    pub width: usize,
    pub height: usize,
    pub board: Vec<Vec<u16>>,
    pub init_state: Vec<(usize, usize, u16)>,
}

impl Hash for GameState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.board.hash(state);
    }
}

impl Clone for GameState {
    fn clone(&self) -> Self {
        GameState {
            board: self.board.clone(),
            width: self.width,
            height: self.height,
            init_state: self.init_state.clone(),
        }
    }
}

impl GameState {
    pub fn new(width: usize, height: usize, board: Vec<Vec<u16>>) -> GameState {
        let mut game = GameState {
            width,
            height,
            board,
            init_state: Vec::new(),
        };
        game.init_state = game.get_position_value();
        game
    }

    pub fn reset(&mut self) {
        for (row, col, value) in self.init_state.iter() {
            self.board[*row][*col] = *value;
        }
    }

    pub fn get_hash(&self) -> u64 {
        let mut hasher = FxHasher::default();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn print(&self) {
        for row in &self.board {
            eprintln!("{:?}", row);
        }
    }

    pub fn get_position_value(&self) -> Vec<(usize, usize, u16)> {
        let mut positions: Vec<(usize, usize, u16)> = Vec::new();
        for (row, cells) in self.board.iter().enumerate() {
            for (col, &cell) in cells.iter().enumerate() {
                if cell > 0 {
                    positions.push((row, col, cell));
                }
            }
        }
        positions
    }

    pub fn apply_action(&mut self, action: &Action) -> Result<(), bool> {
        // if !self.is_valid_action(action.row, action.col, &action.direction) {
        //     return Err(false);
        // }

        let value = self.board[action.row][action.col] as usize;
        let (r2, c2) = match &action.direction {
            Direction::Up => (action.row - value, action.col),
            Direction::Down => (action.row + value, action.col),
            Direction::Left => (action.row, action.col - value),
            Direction::Right => (action.row, action.col + value),
        };

        let source_value = self.board[action.row][action.col] as i16;
        let target_value = self.board[r2][c2] as i16;

        self.board[action.row][action.col] = 0;
        self.board[r2][c2] = match action.op {
            Operation::Plus => (source_value + target_value) as u16,
            Operation::Minus => (source_value - target_value).unsigned_abs(),
        };

        Ok(())
    }

    pub fn game_won(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|&cell| cell == 0))
    }

    pub fn score(&self, metric: &Metric) -> i32 {
        match metric {
            Metric::RemainingSum => self.score_total_sum(),
            Metric::RemainingTiles => self.score_count_tiles(),
            Metric::ColRowsUsed => self.score_col_rows_used(),
        }
    }

    fn score_total_sum(&self) -> i32 {
        let mut score = 0;
        for row in &self.board {
            for &cell in row {
                score += cell as i32;
            }
        }
        score
    }

    fn score_count_tiles(&self) -> i32 {
        let mut score: i32 = 0;
        for row in &self.board {
            for &cell in row {
                if cell > 0 {
                    score += 1;
                }
            }
        }
        score
    }

    fn score_col_rows_used(&self) -> i32 {
        let rows_used = self
            .board
            .iter()
            .filter(|&row| row.iter().any(|&cell| cell > 0))
            .count() as i32;

        let mut cols_used = 0;
        for col in 0..self.board[0].len() {
            cols_used += if self.board.iter().any(|row| row[col] > 0) {
                1
            } else {
                0
            };
        }

        rows_used + cols_used
    }

    pub fn get_actions(&self, row: usize, col: usize) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if !self.is_valid_action(row, col, &direction) {
                continue;
            }

            for op in [Operation::Plus, Operation::Minus] {
                let action: Action = Action {
                    row,
                    col,
                    direction: direction.clone(),
                    op: op.clone(),
                };
                actions.push(action);
            }
        }
        actions
    }

    pub fn is_valid_action(&self, row: usize, col: usize, direction: &Direction) -> bool {
        let value = self.board[row][col] as usize;
        if value == 0 {
            return false;
        }
        match direction {
            Direction::Up => row >= value && self.board[row - value][col] > 0,
            Direction::Down => row + value < self.height && self.board[row + value][col] > 0,
            Direction::Left => col >= value && self.board[row][col - value] > 0,
            Direction::Right => col + value < self.width && self.board[row][col + value] > 0,
        }
    }
}
