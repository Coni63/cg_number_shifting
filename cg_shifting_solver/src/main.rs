mod constants;
mod game_state;
mod loader;
mod solver;

use solver::Solver;

fn main() {
    let game = loader::read_input();

    eprintln!("Game state loaded");
    let mut solver = Solver::new(game);

    if let Some(solution) = solver.solve() {
        eprintln!("Solution found");
        loader::write_output(solution);
    } else {
        eprintln!("No solution found.");
    }
}
