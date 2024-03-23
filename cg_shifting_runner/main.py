import subprocess

import requests
from database import add_level, set_solution, get_first_unsolved
from dotenv import load_dotenv
import os

load_dotenv()

program_execute = "python foo.py"
cookie = os.environ.get("COOKIE_REMEMBERME")
user_id = os.environ.get("USER_ID")

session = requests.Session()
session.cookies.set("rememberMe", cookie, domain="codingame.com")


def extract_current_puzzle() -> tuple[str, int]:
    level = get_first_unsolved()
    with open("level.txt", "w") as f:
        f.write(level.level_input)
    return (level.hash, level.level_number)


def extract_current_solution() -> str:
    with open("solution.txt", "r") as f:
        solution = f.read()
    return solution


def solve_level():
    level_pass, level_number = extract_current_puzzle()
    completed_process = subprocess.run("solver.exe < level.txt > solution.txt", shell=True)
    exit_code = completed_process.returncode
    if exit_code == 0:
        solution = extract_current_solution()
        set_solution(level_number, solution)
        print(f"Solution found for level {level_number} and saved to database.")
        return True, level_pass, solution
    else:
        print(f"Solver exited with non-zero exit code: {exit_code}")
        return False, None, None


def get_api_handle():
    r = session.post(
        "https://www.codingame.com/services/Puzzle/generateSessionFromPuzzlePrettyId",
        json=[user_id, "number-shifting", False],
    )
    return r.json()["handle"]


def submit_solution(handle: int, level_pass: str, solution: str) -> tuple[str, int, str]:
    r = session.post(
        "https://www.codingame.com/services/TestSession/play",
        json=[
            handle,
            {
                "code": level_pass + '\n' + solution,
                "programmingLanguageId": "PHP",
                "multipleLanguages": {"testIndex": 1},
            },
        ],
    )

    if r.status_code != 200:
        print(f"Failed to submit solution: {r.status_code}")
        return None, None, None

    data = r.json()
    for frame in data["frames"]:
        if "Code for next level" in frame.get("gameInformation", ""):
            game_info = frame.get("gameInformation", "")
            level_pass = game_info.split("\n")[0].split(":")[1].strip()
            level_data = game_info.split("\n", 1)[1]
            number_level = 1 + int(data["metadata"]["Level"])
            return level_pass, number_level, level_data
    return None, None, None


def main():
    handle = get_api_handle()
    while True:
        solved, level_pass, solution = solve_level()
        if not solved:
            break

        level_pass, number_level, level_data = submit_solution(handle, level_pass, solution)
        if level_pass is None:
            break

        add_level(number_level, level_pass, level_data)


def main_offline():
    solved, level_pass, solution = solve_level()
    print(f"Level pass: {level_pass}")
    print(f"Solution: \n{solution}")


def setup_db():
    add_level(1, "first_level", "8 5\n0 0 0 4 0 0 0 0\n0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0\n0 0 0 1 0 0 2 1")
    set_solution(1, "7 4 L +\n3 0 D -\n6 4 L -")

    add_level(2, "pmkhklcgypoivqgfzyyuvmtsywegacwu", '8 5\n0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0\n0 0 0 0 2 0 0 0\n0 0 0 0 0 0 0 0\n11 0 0 0 6 0 0 7\n')
    set_solution(2, "4 2 D -\n7 4 L -\n4 4 L -")

    add_level(3, "vtiuddduknpfjutlzlxrkbavooshdkgt", "8 5\n0 3 0 0 0 0 0 0\n0 0 0 0 0 0 0 0\n0 3 0 0 0 0 0 0\n0 6 0 2 0 0 0 0\n0 2 0 0 0 0 0 0\n")


if __name__ == "__main__":
    # setup_db()
    # main()
    main_offline()