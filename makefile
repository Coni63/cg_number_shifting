run_solver:
	cd cg_shifting_solver && cargo run < ../level.txt > ../solution.txt

run_build_solver:
	cd cg_shifting_solver && cargo build --release && target\release\cg_shifting_solver.exe < ../level.txt > ../solution.txt