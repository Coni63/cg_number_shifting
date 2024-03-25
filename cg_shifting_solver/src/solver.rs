use std::collections::VecDeque;

use crate::entity::action::{Action, Direction, Operation};
use crate::entity::game_state::GameState;
use rand::seq::SliceRandom;
use rand::Rng;

pub struct Solver {
    pub play_game: GameState,
    pub late_acceptance: VecDeque<i32>,
    pub init_score: i32,
    pub init_board_prep: Vec<(usize, usize, u8)>,
}

impl Solver {
    pub fn new(game: GameState) -> Solver {
        Solver {
            init_score: game.score(),
            init_board_prep: game.get_position_value(),
            play_game: game,
            late_acceptance: VecDeque::with_capacity(20),
        }
    }

    pub fn solve(&mut self) -> Option<Solution> {
        let mut solution = self.generate_random_solution();
        self.late_acceptance.push_front(solution.score);

        let mut loop_count = 0;

        loop {
            if loop_count > 1000000 {
                loop_count = 0;
                solution = self.generate_random_solution();
                self.late_acceptance.clear();
                self.late_acceptance.push_front(solution.score);
                eprintln!("Restarting")
            }

            solution = self.mutate(&solution);

            if solution.score == 0 {
                return Some(solution);
            }

            if self.late_acceptance.len() < self.late_acceptance.capacity() {
                self.late_acceptance.push_front(solution.score);
            } else {
                let max_score = self.late_acceptance.iter().max().unwrap();
                let t = rand::thread_rng().gen_range(0.0..1.0);
                if &solution.score <= max_score || t < 0.05 {
                    self.late_acceptance.pop_back();
                    self.late_acceptance.push_front(solution.score);
                }
            }

            loop_count += 1;
        }
    }

    fn mutate(&mut self, base_solution: &Solution) -> Solution {
        let mut rng = rand::thread_rng();
        let idx_mutated = rng.gen_range(0..base_solution.actions.len());

        let mut new_actions: Vec<Action> = Vec::new();

        // replay the game to the state of the base solution
        for i in 0..idx_mutated {
            self.play_game
                .apply_action(&base_solution.actions[i])
                .unwrap();
            new_actions.push(base_solution.actions[i].clone());
        }

        // alter the action
        let prev_action = base_solution.actions.get(idx_mutated).unwrap();
        if let Some(altered_action) = self.alter_action(prev_action) {
            self.play_game.apply_action(&altered_action).unwrap();
            new_actions.push(altered_action);
        }

        // replay the game to the end -- filter out invalid actions
        for i in idx_mutated + 1..base_solution.actions.len() {
            let action = base_solution.actions.get(i).unwrap();

            if let Ok(()) = self.play_game.apply_action(action) {
                new_actions.push(action.clone());
            }
        }

        // add random actions to the end
        while let Some(action) = self.get_random_action() {
            new_actions.push(action);
        }

        let new_solution = Solution {
            actions: new_actions,
            score: self.play_game.score(),
        };

        self.reset_game();

        new_solution
    }

    pub fn reset_game(&mut self) {
        for (row, col, value) in self.init_board_prep.iter() {
            self.play_game.board[*row][*col] = *value;
        }
    }

    pub fn generate_random_solution(&mut self) -> Solution {
        let mut solution = Solution::new(self.init_score);

        while let Some(action) = self.get_random_action() {
            solution.actions.push(action);
        }

        solution.score = self.play_game.score();

        self.reset_game();
        solution
    }

    fn alter_action(&self, action: &Action) -> Option<Action> {
        let mut rng = rand::thread_rng();

        let possible_actions = self.play_game.get_actions(action.row, action.col);

        if possible_actions.len() <= 1 {
            return None;
        }

        let new_action = possible_actions.choose(&mut rng).unwrap().to_owned();

        Some(new_action)
    }

    fn get_random_action(&mut self) -> Option<Action> {
        let mut rng = rand::thread_rng();

        let mut pos = self.play_game.get_all_tiles();

        if pos.is_empty() {
            return None;
        }

        pos.shuffle(&mut rng);

        for (row, col) in pos.iter() {
            let mut actions = self.play_game.get_actions(*row, *col);

            if actions.is_empty() {
                continue;
            }

            actions.shuffle(&mut rng);

            for action in actions {
                if let Ok(()) = self.play_game.apply_action(&action) {
                    return Some(action);
                }
            }
        }

        None
    }
}

pub struct Solution {
    pub actions: Vec<Action>,
    pub score: i32,
}

impl Solution {
    pub fn new(base_score: i32) -> Solution {
        Solution {
            actions: Vec::new(),
            score: base_score,
        }
    }
}

impl Clone for Solution {
    fn clone(&self) -> Solution {
        Solution {
            actions: self.actions.clone(),
            score: self.score,
        }
    }
}
