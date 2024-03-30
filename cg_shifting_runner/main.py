from pathlib import Path
import subprocess
import time

import requests
from database import (
    Results,
    add_level,
    get_level_by_id,
    reset_level,
    set_solution,
    get_first_unsolved,
)
from dotenv import load_dotenv
import os
import bz2
import base64

load_dotenv()

program_execute = r"cg_shifting_solver\target\release\cg_shifting_solver.exe"
cookie = os.environ.get("COOKIE_REMEMBERME")
user_id = os.environ.get("USER_ID")

session = requests.Session()
session.cookies.set("rememberMe", cookie, domain="codingame.com")


def _load_current_solution() -> str:
    """
    Load the solution from the file. The solver writes the solution to a file after solving the puzzle.
    """
    with open("../solution.txt", "r") as f:
        solution = f.read()
    return solution


def _get_api_handle():
    """
    Get the handle of the current session from the Codingame API.
    """
    r = session.post(
        "https://www.codingame.com/services/Puzzle/generateSessionFromPuzzlePrettyId",
        json=[user_id, "number-shifting", False],
    )
    return r.json()["handle"]


def _extract_puzzle(level: Results):
    """
    Extract the puzzle from the database and write it to a file.
    Requires for the solver to read the puzzle from a file.
    """
    with open("../level.txt", "w") as f:
        f.write(level.level_input)


def extract_current_puzzle() -> tuple[str, int, bool]:
    """
    Prepare the input file for the solver by extracting the puzzle from the database.
    """
    level = get_first_unsolved()
    if level is None:
        print("No more levels to solve.")
        return None, None

    _extract_puzzle(level)
    return (level.level_pass, level.level_number, level.solved)


def extract_puzzle_by_id(level_id: int) -> str:
    """
    Prepare the input file for the solver by extracting the puzzle from the database.
    """
    level = get_level_by_id(level_id)
    if level is None:
        print(f"ID #{level_id} not found.")
        return None, None

    _extract_puzzle(level)
    return (level.level_pass, level.level_number)


def solve_level() -> bool:
    """
    Run the solver and provide path to the input and output files.
    """
    cwd = Path.cwd().parent
    completed_process = subprocess.run(
        f"{program_execute} < level.txt > solution.txt", shell=True, cwd=cwd
    )
    exit_code = completed_process.returncode
    return exit_code == 0


def solve_level_multi() -> bool:
    import signal
    import subprocess
    import os
    from pathlib import Path
    import shutil
    import psutil

    def kill_process_tree(pid):
        parent = psutil.Process(pid)
        children = parent.children(recursive=True)
        for child in children:
            child.terminate()
        parent.terminate()

    cwd = Path.cwd().parent

    tmp_dir = cwd / "tmp"
    tmp_dir.mkdir(exist_ok=True)

    processes = []
    num_processes = 4

    for i in range(num_processes):
        input_file = tmp_dir / f"level_{i}.txt"
        output_file = tmp_dir / f"solution_{i}.txt"
        shutil.copyfile(cwd / "level.txt", input_file)
        # print(f"{cwd / program_execute} < {input_file} > {output_file}")
        proc = subprocess.Popen(
            f"{cwd / program_execute} < {input_file} > {output_file}",
            shell=True,
            cwd=cwd,
        )
        processes.append(proc)

    finished = False
    solver_id = None
    while not finished:
        for i, proc in enumerate(processes):
            if proc.poll() is not None:
                print(f"Solution found by solver #{i} (PID: {proc.pid})")
                solver_id = i
                for j, proc2 in enumerate(processes):
                    if j != i:
                        kill_process_tree(proc2.pid)
                        print(f"Solution found by solver #{j} (PID: {proc2.pid})")
                        finished = True
                break

    solution_file = tmp_dir / f"solution_{solver_id}.txt"
    shutil.copyfile(solution_file, cwd / "solution.txt")

    return True


def submit_solution(handle: int, level_pass: str, solution: str) -> tuple[str, int]:
    """
    Submit the solution to the Codingame API and return the next level pass and level data.

    :param handle: The handle of the current session.
    :param level_pass: The level pass of the current level.
    :param solution: The list of actions applied to the current level.
    """
    r = session.post(
        "https://www.codingame.com/services/TestSession/play",
        json=[
            handle,
            {
                "code": level_pass + "\n" + solution,
                "programmingLanguageId": "PHP",
                "multipleLanguages": {"testIndex": 1},
            },
        ],
    )

    if r.status_code != 200:
        print(f"Failed to submit solution: {r.status_code}")
        return None, None

    data = r.json()
    for frame in data["frames"]:
        if "Code for next level" in frame.get("gameInformation", ""):
            game_info = frame.get("gameInformation", "")
            level_pass = game_info.split("\n")[0].split(":")[1].strip()
            number_level = 1 + int(data["metadata"]["Level"])
            return level_pass, number_level
    return None, None


def get_level_data(handle: int, level_pass: str) -> str:
    solution = f"""
import sys
import math
import bz2
import base64

print("{level_pass}")

while True:
    line = input()
    txt = [line]
    width, height = [int(i) for i in line.split()]
    for i in range(height):
        line = input()
        txt.append(line)

    string_to_encode = "\\n".join(txt)
    c = bz2.BZ2Compressor(9)
    compressed_data = c.compress(string_to_encode.encode("utf-8")) + c.flush()
    compressed_string = base64.b64encode(compressed_data).decode("utf-8")

    print(compressed_string)
"""
    r = session.post(
        "https://www.codingame.com/services/TestSession/play",
        json=[
            handle,
            {
                "code": solution,
                "programmingLanguageId": "Python3",
                "multipleLanguages": {"testIndex": 1},
            },
        ],
    )
    data = r.json()

    compressed_string = data["frames"][-1]["stdout"].split("\n")[0]

    d = bz2.BZ2Decompressor()
    data2 = base64.b64decode(compressed_string.encode("utf-8"))
    data = d.decompress(data2).decode("utf-8")

    return data


def main():
    """
    Main function to solve all the levels. Starting from the last solved level, it will solve all the levels after that.
    """
    handle = _get_api_handle()
    while True:
        level_pass, number_level, solved = extract_current_puzzle()
        if level_pass is None:
            break

        print(f"Restart at Level {number_level}")
        # worked = solve_level()
        worked = solve_level_multi()
        if not worked:
            break

        print(f"Level {number_level} solved... Saving solution.")
        solution = _load_current_solution()
        set_solution(number_level, solution)
        level_pass, number_level = submit_solution(handle, level_pass, solution)

        if level_pass is None:
            break

        level_data = get_level_data(handle, level_pass)

        add_level(number_level, level_pass, level_data)


def main_offline():
    """
    Main function to solve the last unsolved puzzle. This version is offline so it will not
    submit the solution to the Codingame API. As a result, it will not get the next level pass.
    """
    level_pass, number_level = extract_current_puzzle()
    if level_pass is None:
        return

    worked = solve_level()
    if not worked:
        return

    solution = _load_current_solution()
    print(f"Level pass: {level_pass}")
    print(f"Solution: \n{solution}")


def dry_run(
    a,
    b,
):
    """
    Benchmark the solver for levels a to b.
    There is an overhead due to subprocess.run() so the time taken is not accurate.
    """
    for i in range(a, b):
        extract_puzzle_by_id(i)
        tic = time.time()
        worked = solve_level()
        toc = time.time()
        if not worked:
            print(f"Level {i} error - Time taken: {toc - tic}")
        else:
            print(f"Level {i} solved - Time taken: {toc - tic}")


def setup_db():
    # """
    # Initialize the database with the first few levels.
    # """
    add_level(
        1,
        "first_level",
        "8 5\n0 0 0 4 0 0 0 0\n0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0\n0 0 0 1 0 0 2 1",
    )
    set_solution(1, "7 4 L +\n3 0 D -\n6 4 L -")

    add_level(
        2,
        "pmkhklcgypoivqgfzyyuvmtsywegacwu",
        "8 5\n0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0\n0 0 0 0 2 0 0 0\n0 0 0 0 0 0 0 0\n11 0 0 0 6 0 0 7\n",
    )
    set_solution(2, "4 2 D -\n7 4 L -\n4 4 L -")

    add_level(
        3,
        "vtiuddduknpfjutlzlxrkbavooshdkgt",
        "8 5\n0 3 0 0 0 0 0 0\n0 0 0 0 0 0 0 0\n0 3 0 0 0 0 0 0\n0 6 0 2 0 0 0 0\n0 2 0 0 0 0 0 0\n",
    )


if __name__ == "__main__":
    # setup_db()
    main()
    # main_multi()
    # main_offline()
    # dry_run(1, 25)
    # benchmark(1, 15)
    # reset_level(270)
    # reset_level(271)
