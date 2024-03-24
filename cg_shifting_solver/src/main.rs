mod entity;
mod loader;
mod solver;

use entity::game_state::GameState;
use solver::Solution;
use solver::Solver;

fn main() {
    let mut game = loader::read_input();

    let mut solver = solver::Solver::new(game);

    let solution = solver.solve();

    eprintln!("Solver: {:?}", solver.init_score);
    eprintln!("Solution: {:?}", solution.score);

    loader::write_output(solution.actions);
}
