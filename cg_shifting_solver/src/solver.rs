use crate::entity::action::Action;
use crate::entity::enums::Metric;
use crate::entity::game_state::GameState;
use crate::entity::solution::Solution;
use rand::seq::SliceRandom;
use rand::Rng;

pub struct Solver {
    pub game: GameState,
    pub init_score: i32,
    pub metric: Metric,
    pub temperature: f64,
    pub cooling_rate: f64,
    pub num_steps: i32,
}

impl Solver {
    pub fn new(game: GameState, metric: Metric) -> Solver {
        let mut s = Solver {
            init_score: game.score(&metric),
            game,
            metric,
            temperature: 1.0,
            num_steps: 200_000,
            cooling_rate: 1.0,
        };
        s.cooling_rate = (0.001_f64.log2() / s.num_steps as f64).exp2();
        eprintln!("Cooling rate: {:?}", s.cooling_rate);
        s
    }

    pub fn solve(&mut self) -> Option<Solution> {
        let mut solution = self.generate_random_solution();

        let mut loop_count = 0;

        loop {
            if loop_count > self.num_steps {
                loop_count = 0;
                solution = self.generate_random_solution();
                self.temperature = 1.0;
                self.metric = self.get_random_metric();
                eprintln!("Restarting with new metric: {:?}", self.metric);
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
            let p = (-diff / self.temperature).exp();

            // eprintln!("p: {:?} - diff: {} - T: {}", p, diff, self.temperature);

            p * 0.5
        }
    }

    fn mutate(&mut self, base_solution: &Solution) -> Solution {
        let mut rng = rand::thread_rng();
        let idx_mutated = rng.gen_range(0..base_solution.actions.len());

        let mut new_actions: Vec<Action> = Vec::new();
        self.game.reset();

        // replay the game to the state of the base solution
        for i in 0..idx_mutated {
            new_actions.push(base_solution.actions[i].clone());
            self.game.apply_action(&base_solution.actions[i]).unwrap();
        }

        // alter the action
        // let prev_action = base_solution.actions[idx_mutated];
        // if let Some(altered_action) = self.alter_action(&prev_action) {
        //     self.game.apply_action(&altered_action).unwrap();
        //     new_actions.push(altered_action);
        // }

        // replay the game to the end -- filter out invalid actions
        for i in idx_mutated + 1..base_solution.actions.len() {
            let action = base_solution.actions.get(i).unwrap();

            if self
                .game
                .is_valid_action(action.row, action.col, &(action.direction))
            {
                self.game.apply_action(action).unwrap();
                new_actions.push(action.clone());
            }
        }

        // add random actions to the end
        while let Some(action) = self.get_random_action() {
            self.game.apply_action(&action).unwrap();
            new_actions.push(action);
        }

        Solution {
            actions: new_actions,
            score: self.game.score(&self.metric),
        }
    }

    fn generate_random_solution(&mut self) -> Solution {
        self.game.reset();
        let mut actions: Vec<Action> = Vec::new();

        while let Some(action) = self.get_random_action() {
            self.game.apply_action(&action).unwrap();
            actions.push(action);
        }

        Solution::new(actions, self.game.score(&self.metric))
    }

    fn alter_action(&self, action: &Action) -> Option<Action> {
        let mut rng = rand::thread_rng();

        let mut possible_actions = self.game.get_actions(action.row, action.col);

        if possible_actions.len() <= 1 {
            return None;
        }

        possible_actions.shuffle(&mut rng);

        for new_action in possible_actions.iter() {
            if new_action == action {
                continue;
            }

            return Some(new_action.clone());
        }

        None
    }

    fn get_random_action(&mut self) -> Option<Action> {
        let mut rng = rand::thread_rng();

        let mut pos = self.game.get_position_value();

        if pos.is_empty() {
            return None;
        }

        pos.shuffle(&mut rng);

        for (row, col, _) in pos.iter() {
            let mut actions = self.game.get_actions(*row, *col);

            if actions.is_empty() {
                continue;
            }

            actions.shuffle(&mut rng);

            for action in actions {
                if self
                    .game
                    .is_valid_action(action.row, action.col, &(action.direction))
                {
                    return Some(action);
                }
            }
        }

        None
    }

    fn get_random_metric(&self) -> Metric {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..3);
        match index {
            0 => Metric::RemainingSum,
            1 => Metric::RemainingTiles,
            2 => Metric::ColRowsUsed,
            _ => unreachable!(), // This should never happen
        }
    }
}
