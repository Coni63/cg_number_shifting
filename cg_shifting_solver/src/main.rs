use rayon::prelude::*;
use std::cell::RefCell;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

mod entity;
mod loader;
mod solver;
use entity::action::Metric;
use entity::game_state::GameState;
use solver::Solution;
use solver::Solver;

// fn main() {
//     let mut game = loader::read_input();

//     let mut solver = solver::Solver::new(game);

//     if let Some(solution) = solver.solve() {
//         eprintln!("Solver: {:?}", solver.init_score);
//         eprintln!("Solution: {:?}", solution.score);

//         loader::write_output(solution.actions);
//     }
// }

fn main() {
    // Number of solvers to run in parallel
    let num_solvers = 5;

    // Shared flag to signal when a solution is found
    let solution_found = Arc::new(AtomicBool::new(false));

    // Create a game instance to be shared among solvers
    let game = loader::read_input();

    // Create solvers in parallel and stop as soon as one finds the solution
    let solution = (0..num_solvers).into_par_iter().find_any(|_| {
        let game_clone = game.clone(); // Create a deep copy of the game
        let solver = RefCell::new(solver::Solver::new(game_clone, Metric::RemainingSum));

        if !solution_found.load(Ordering::Relaxed) {
            let mut solver_mut = solver.borrow_mut();
            if let Some(_solution) = solver_mut.solve() {
                // Solution found, set the flag and return true to stop other solvers
                solution_found.store(true, Ordering::Relaxed);
                loader::write_output(_solution.actions);
                return true;
            }
        }

        false
    });

    // Check if a solution is found
    if let Some(solution) = solution {
        eprintln!("Solution found: {:?}", solution);
    } else {
        eprintln!("No solution found.");
    }
}
