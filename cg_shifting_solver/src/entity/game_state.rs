use crate::entity::action::{Action, Direction, Operation};

pub struct GameState<'a> {
    pub board: Vec<Vec<u16>>,
    pub actions_taken: Vec<Action<'a>>,

    pub all_directions: Vec<&'a Direction>,
    pub all_operations: Vec<&'a Operation>,
}

impl<'a> GameState<'a> {
    pub fn new() -> GameState<'a> {
        GameState {
            board: Vec::new(),
            actions_taken: Vec::new(),
            all_directions: vec![
                &Direction::Up,
                &Direction::Down,
                &Direction::Left,
                &Direction::Right,
            ],
            all_operations: vec![&Operation::Plus, &Operation::Minus],
        }
    }

    pub fn print(&self) {
        for row in &self.board {
            eprintln!("{:?}", row);
        }
    }

    pub fn step(&mut self) -> bool {
        if self.game_won() {
            return true;
        }

        for action in self.get_all_possible_actions().iter() {
            self.apply_action(action);
            if !self.step() {
                self.undo_action(action);
            } else {
                return true;
            }
        }
        false
    }

    fn apply_action(&mut self, action: &Action<'a>) {
        // eprintln!("Applying {}", action.to_string());
        let (r2, c2) = self.get_target_position(
            action.row,
            action.col,
            action.direction,
            action.source_value as usize,
        );

        self.board[action.row][action.col] = 0;
        self.board[r2][c2] = match action.op {
            Operation::Plus => action.source_value + action.target_value,
            Operation::Minus => {
                ((action.source_value as i16) - (action.target_value as i16)).unsigned_abs()
            }
        };
        self.actions_taken.push(action.clone());
    }

    fn undo_action(&mut self, action: &Action<'a>) {
        // eprintln!("Reverting {}", action.to_string());
        let (r2, c2) = self.get_target_position(
            action.row,
            action.col,
            action.direction,
            action.source_value as usize,
        );

        self.board[action.row][action.col] = action.source_value;
        self.board[r2][c2] = action.target_value;
        self.actions_taken.pop();
    }

    fn game_won(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|&cell| cell == 0))
    }

    fn get_all_possible_actions(&self) -> Vec<Action<'a>> {
        let mut actions: Vec<Action<'a>> = Vec::new();
        for (row, cells) in self.board.iter().enumerate() {
            for (col, &cell) in cells.iter().enumerate() {
                if cell > 0 {
                    actions.extend(self.get_actions(row, col));
                }
            }
        }
        actions
    }

    fn get_actions(&self, row: usize, col: usize) -> Vec<Action<'a>> {
        let mut actions: Vec<Action<'a>> = Vec::new();
        for direction in self.all_directions.iter() {
            if !self.is_valid_action(row, col, direction) {
                continue;
            }

            for op in self.all_operations.iter() {
                let source_value = self.board[row][col];

                let (r, c) = self.get_target_position(row, col, direction, source_value as usize);
                let target_value = self.board[r][c];

                let action: Action<'a> = Action {
                    row,
                    col,
                    direction,
                    op,
                    source_value,
                    target_value,
                };
                actions.push(action);
            }
        }
        actions
    }

    fn is_valid_action(&self, row: usize, col: usize, direction: &Direction) -> bool {
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

    fn get_target_position(
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
