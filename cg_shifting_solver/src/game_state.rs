use rand::Rng;

use crate::constants::{NUM_ACTIONS, NUM_COLS, NUM_ROWS};

pub struct GameState {
    pub width: usize,
    pub height: usize,
    pub board: [[i16; NUM_COLS]; NUM_ROWS],
    pub init_state: [(usize, usize, i16); NUM_ACTIONS],
    pub count_tile: i32,
    pub generator: rand::rngs::ThreadRng,
}

impl GameState {
    pub fn new(
        width: usize,
        height: usize,
        count_tile: i32,
        board: [[i16; NUM_COLS]; NUM_ROWS],
    ) -> GameState {
        let mut game = GameState {
            width,
            height,
            count_tile,
            board,
            init_state: [(100, 100, 100); NUM_ACTIONS],
            generator: rand::thread_rng(),
        };
        game.set_position_value();
        game
    }

    pub fn reset(&mut self) {
        for (row, col, value) in self.init_state.iter().take(self.count_tile as usize) {
            self.board[*row][*col] = *value;
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for row in &self.board {
            eprintln!("{:?}", row);
        }
    }

    pub fn set_position_value(&mut self) {
        let mut idx = 0;
        for (row, cells) in self.board.iter().enumerate() {
            for (col, &cell) in cells.iter().enumerate() {
                if cell > 0 {
                    self.init_state[idx] = (row, col, cell);
                    idx += 1;
                }
            }
        }
    }

    pub fn apply_action(&mut self, action: (u8, u8, u8, bool)) {
        let (row, col, dir, op) = action;
        let value = self.board[row as usize][col as usize] as u8;
        let (r2, c2) = match dir {
            0 => (row - value, col),
            1 => (row + value, col),
            2 => (row, col - value),
            3 => (row, col + value),
            _ => (row, col),
        };

        let source_value = value as i16;
        let target_value = self.board[r2 as usize][c2 as usize];
        self.board[row as usize][col as usize] = 0;
        self.board[r2 as usize][c2 as usize] = match op {
            true => source_value + target_value,
            false => (source_value - target_value).abs(),
        };
    }

    pub fn score(&self, metric: u8) -> i32 {
        match metric {
            0 => self.score_total_sum(),
            1 => self.score_count_tiles(),
            2 => self.score_col_rows_used(),
            _ => i32::MAX,
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
        let mut col_used: [bool; NUM_COLS] = [false; NUM_COLS];
        let mut rows_used: [bool; NUM_ROWS] = [false; NUM_ROWS];

        for row in 0..self.height {
            for col in 0..self.width {
                if self.board[row][col] > 0 {
                    col_used[col] = true;
                    rows_used[row] = true;
                }
            }
        }

        self.count_true(&rows_used) + self.count_true(&col_used)
    }

    fn count_true(&self, arr: &[bool]) -> i32 {
        let mut count = 0;
        for &value in arr.iter() {
            if value {
                count += 1;
            }
        }
        count
    }

    pub fn get_random_action(&mut self, row: usize, col: usize) -> Option<(u8, u8, u8, bool)> {
        let mut valid_dir: Vec<u8> = Vec::new();
        for direction in 0..4_u8 {
            if !self.is_valid_action(row, col, direction) {
                continue;
            }

            valid_dir.push(direction);
        }

        if valid_dir.is_empty() {
            return None;
        }

        let direction = valid_dir[self.generator.gen_range(0..valid_dir.len())];
        let sign = self.generator.gen_range(0..4);

        Some((row as u8, col as u8, direction, sign == 0)) // 3 chances out of 4 to be a subtraction
    }

    pub fn is_valid_action(&self, row: usize, col: usize, direction: u8) -> bool {
        let value = self.board[row][col] as usize;
        if value == 0 {
            return false;
        }
        match direction {
            0 => row >= value && self.board[row - value][col] > 0,
            1 => row + value < self.height && self.board[row + value][col] > 0,
            2 => col >= value && self.board[row][col - value] > 0,
            3 => col + value < self.width && self.board[row][col + value] > 0,
            _ => false,
        }
    }
}
