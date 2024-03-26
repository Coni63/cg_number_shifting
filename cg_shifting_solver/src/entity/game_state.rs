use std::hash::Hash;
use std::hash::Hasher;

use crate::entity::action::{Action, Direction, Metric, Operation};
use fxhash::FxHasher;

pub struct GameState {
    pub board: Vec<Vec<u8>>,
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
        }
    }
}

impl GameState {
    pub fn new() -> GameState {
        GameState { board: Vec::new() }
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

    pub fn get_position_value(&self) -> Vec<(usize, usize, u8)> {
        let mut positions: Vec<(usize, usize, u8)> = Vec::new();
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
        // eprintln!("Applying {}", action.to_string());

        if !self.is_valid_action(action.row, action.col, &action.direction) {
            return Err(false);
        }

        let source_value = self.board[action.row][action.col] as i8;
        let (r2, c2) = self.get_target_position(
            action.row,
            action.col,
            &action.direction,
            source_value as usize,
        );
        let target_value = self.board[r2][c2] as i8;

        if source_value == 0 || target_value == 0 {
            return Err(false);
        }

        self.board[action.row][action.col] = 0;
        self.board[r2][c2] = match action.op {
            Operation::Plus => (source_value + target_value) as u8,
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

    fn impossible_state(&self) -> bool {
        // if the highest value is above the sum of all other values it's impossible
        let mut max_value = 0;
        let mut sum = 0;
        for row in &self.board {
            for &cell in row {
                if cell > max_value {
                    max_value = cell;
                }
                sum += cell;
            }
        }
        return max_value * 2 > sum;
    }

    fn get_all_possible_actions(&self) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();
        for (row, cells) in self.board.iter().enumerate() {
            for (col, &cell) in cells.iter().enumerate() {
                if cell > 0 {
                    actions.extend(self.get_actions(row, col));
                }
            }
        }
        actions
    }

    pub fn get_all_tiles(&self) -> Vec<(usize, usize)> {
        let mut tiles: Vec<(usize, usize)> = Vec::new();
        for (row, cells) in self.board.iter().enumerate() {
            for (col, &cell) in cells.iter().enumerate() {
                if cell > 0 {
                    tiles.push((row, col));
                }
            }
        }
        tiles
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
        match direction {
            Direction::Up => row >= value && self.board[row - value][col] > 0,
            Direction::Down => row + value < self.board.len() && self.board[row + value][col] > 0,
            Direction::Left => col >= value && self.board[row][col - value] > 0,
            Direction::Right => {
                col + value < self.board[0].len() && self.board[row][col + value] > 0
            }
        }
    }

    pub fn get_target_position(
        &self,
        row: usize,
        col: usize,
        direction: &Direction,
        value: usize,
    ) -> (usize, usize) {
        match direction {
            Direction::Up => (row - value, col),
            Direction::Down => (row + value, col),
            Direction::Left => (row, col - value),
            Direction::Right => (row, col + value),
        }
    }
}
