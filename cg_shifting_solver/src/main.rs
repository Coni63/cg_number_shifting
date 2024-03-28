mod entity;
mod loader;
mod solver;

use entity::enums::Metric;
use solver::Solver;

fn main() {
    let game = loader::read_input();

    eprintln!("Game: {:?}", game.board);

    let mut solver = Solver::new(game, Metric::RemainingSum);

    if let Some(solution) = solver.solve() {
        eprintln!("Solver: {:?}", solver.init_score);
        eprintln!("Solution: {:?}", solution.score);

        loader::write_output(solution.actions);
    } else {
        eprintln!("No solution found.");
    }
}
