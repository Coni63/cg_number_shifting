use rand::Rng;

use crate::constants::{NUM_ACTIONS, NUM_COLS, NUM_ROWS};

#[derive(Clone)]
pub struct GameState {
    pub width: usize,
    pub height: usize,
    pub board: [[i16; NUM_COLS]; NUM_ROWS],
    pub init_state: [(usize, usize, i16); NUM_ACTIONS], // simply store positions and values of tiles for fast reset
    pub count_tile: i32,                                // count of non-empty tiles
    pub initial_count_tile: i32,
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
            initial_count_tile: count_tile,
            board,
            init_state: [(100, 100, 100); NUM_ACTIONS],
        };
        game.set_init_state();
        game
    }

    pub fn reset(&mut self) {
        for (row, col, value) in self
            .init_state
            .iter()
            .take(self.initial_count_tile as usize)
        {
            self.board[*row][*col] = *value;
        }
        self.count_tile = self.initial_count_tile;
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for row in &self.board {
            eprintln!("{:?}", row);
        }
    }

    fn set_init_state(&mut self) {
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
        self.count_tile -= 1;
    }

    pub fn score(&self, f1: u32, f2: u32, f3: u32) -> u32 {
        let mut remaining_tiles: u32 = 0;
        let mut remaining_sum: u32 = 0;

        let mut cols_used = 0u64;
        let mut rows_used = 0u64;

        for (row, col, _) in self
            .init_state
            .iter()
            .take(self.initial_count_tile as usize)
        {
            if self.board[*row][*col] > 0 {
                remaining_tiles += 1;
                remaining_sum += self.board[*row][*col] as u32;
                cols_used |= 1u64 << col;
                rows_used |= 1u64 << row;
            }
        }
        let used_rows_cols = cols_used.count_ones() + rows_used.count_ones();

        f1 * remaining_tiles + f2 * remaining_sum + f3 * used_rows_cols
    }

    pub fn is_solved(&self) -> bool {
        for (row, col, _) in self
            .init_state
            .iter()
            .take(self.initial_count_tile as usize)
        {
            let cell_value = self.board[*row][*col];
            if cell_value > 0 {
                return false;
            }
        }

        true
    }

    pub fn get_random_action(
        &self,
        row: usize,
        col: usize,
        generator: &mut rand::rngs::ThreadRng,
    ) -> Option<(u8, u8, u8, bool)> {
        let value = self.board[row][col] as usize;
        if value == 0 || value >= self.width.max(self.height) {
            return None;
        }

        let mut direction = generator.gen_range(0..4);

        for _ in 0..4 {
            if self.is_valid_direction(row, col, value, direction) {
                let sign = generator.gen_range(0..4) == 0;
                return Some((row as u8, col as u8, direction, sign));
            }
            direction = (direction + 1) & 3; // change direction and try again
        }

        None
    }

    pub fn get_random_tile(&self, generator: &mut rand::rngs::ThreadRng) -> Option<(usize, usize)> {
        let start_idx = generator.gen_range(0..self.initial_count_tile as usize);

        for i in 0..self.initial_count_tile as usize {
            let idx = (start_idx + i) % (self.initial_count_tile as usize);
            let (row, col, _) = self.init_state[idx];
            if self.board[row][col] > 0 {
                return Some((row, col));
            }
        }
        None
    }

    #[inline(always)]
    pub fn is_valid_direction(&self, row: usize, col: usize, value: usize, direction: u8) -> bool {
        // check that the target cell is within bounds and non-empty
        match direction {
            0 => {
                row >= value
                    && unsafe { *self.board.get_unchecked(row - value).get_unchecked(col) > 0 }
            }
            1 => {
                row + value < self.height
                    && unsafe { *self.board.get_unchecked(row + value).get_unchecked(col) > 0 }
            }
            2 => {
                col >= value
                    && unsafe { *self.board.get_unchecked(row).get_unchecked(col - value) > 0 }
            }
            3 => {
                col + value < self.width
                    && unsafe { *self.board.get_unchecked(row).get_unchecked(col + value) > 0 }
            }
            _ => false,
        }
    }
}
