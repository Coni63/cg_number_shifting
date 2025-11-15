use crate::constants::NUM_ACTIONS;
use crate::game_state::GameState;
use rand::seq::SliceRandom;
use rand::Rng;

pub struct Solution {
    pub actions: [(u8, u8, u8, bool); NUM_ACTIONS],
    pub score: i32,
    pub num_moves: i32,
}

pub struct Solver {
    pub game: GameState,
    pub init_score: i32,
    pub metric: u8,
    pub temperature: f64,
    pub cooling_rate: f64,
    pub num_steps: i32,
    pub generator: rand::rngs::ThreadRng,
}

impl Solver {
    pub fn new(game: GameState, metric: u8) -> Solver {
        let score = game.score();

        let mut s = Solver {
            init_score: game.score().0,
            game,
            metric,
            temperature: 1.0,
            num_steps: 400_000,
            cooling_rate: 1.0,
            generator: rand::thread_rng(),
        };
        s.cooling_rate = (0.001_f64.log2() / s.num_steps as f64).exp2();
        s
    }

    pub fn solve(&mut self) -> Option<Solution> {
        let mut solution = self.generate_random_solution();

        let mut loop_count = 0;
        let mut start_time = std::time::Instant::now();
        loop {
            if loop_count > self.num_steps {
                // eprintln!("Time elapsed: {:?}", start_time.elapsed());
                loop_count = 0;
                solution = self.generate_random_solution();
                self.temperature = 1.0;
                self.metric = self.generator.gen_range(0..3) as u8;
                // eprintln!("Restarting with new metric: {:?}", self.metric);
            }

            let mutated_solution = self.mutate(&solution);

            if solution.score == 0 {
                eprintln!("Solution found: {:?}", solution.score);
                return Some(solution);
            }

            let acceptance_probability =
                self.acceptance_probability(solution.score, mutated_solution.score);
            if acceptance_probability > rand::random::<f64>() {
                solution = mutated_solution;
            }

            self.temperature *= self.cooling_rate;

            loop_count += 1;
        }
    }

    fn acceptance_probability(&self, score: i32, new_score: i32) -> f64 {
        let diff = (new_score - score) as f64; // negative is good

        if diff < 0.0 {
            1.0
        } else if diff == 0.0 {
            0.5
        } else {
            0.5_f64 * (-diff / self.temperature).exp()
        }
    }

    fn mutate(&mut self, base_solution: &Solution) -> Solution {
        let mut rng = rand::thread_rng();
        let idx_mutated = rng.gen_range(0..base_solution.num_moves) as usize;
        let mut curr_idx: usize = 0;

        let mut new_actions: [(u8, u8, u8, bool); NUM_ACTIONS] =
            [(100, 100, 100, false); NUM_ACTIONS];
        self.game.reset();

        // replay the game to the state of the base solution
        for i in 0..idx_mutated {
            new_actions[curr_idx] = base_solution.actions[i];
            self.game.apply_action(new_actions[curr_idx]);
            curr_idx += 1;
        }

        // replay the game to the end -- filter out invalid actions
        for i in (idx_mutated + 1)..base_solution.num_moves as usize {
            let (row, col, dir, _) = base_solution.actions[i];

            let value = self.game.board[row as usize][col as usize] as usize;

            if self
                .game
                .is_valid_direction(row as usize, col as usize, value, dir)
            {
                new_actions[curr_idx] = base_solution.actions[i];
                self.game.apply_action(new_actions[curr_idx]);
                curr_idx += 1;
            }
        }

        // add random actions to the end
        while let Some(action) = self.get_random_action() {
            new_actions[curr_idx] = action;
            self.game.apply_action(new_actions[curr_idx]);
            curr_idx += 1;
        }

        Solution {
            actions: new_actions,
            score: self.game.score().0,
            num_moves: (curr_idx - 1) as i32,
        }
    }

    fn generate_random_solution(&mut self) -> Solution {
        self.game.reset();
        let mut actions: [(u8, u8, u8, bool); NUM_ACTIONS] = [(100, 100, 100, false); NUM_ACTIONS];

        let mut idx = 0;
        while let Some(action) = self.get_random_action() {
            self.game.apply_action(action);
            actions[idx] = action;
            idx += 1;
        }

        Solution {
            actions,
            score: self.game.score().0,
            num_moves: (idx - 1) as i32,
        }
    }

    fn get_random_action(&mut self) -> Option<(u8, u8, u8, bool)> {
        let mut tiles: Vec<(usize, usize)> = Vec::new();

        for (row, col, _) in self.game.init_state {
            if row == 100 {
                break;
            }
            if self.game.board[row][col] > 0 {
                tiles.push((row, col));
            }
        }

        if tiles.is_empty() {
            return None;
        }

        tiles.shuffle(&mut self.generator);

        for (row, col) in tiles {
            if let Some(action) = self.game.get_random_action(row, col) {
                return Some(action);
            }
        }
        None
    }
}
