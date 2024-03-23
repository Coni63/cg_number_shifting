import os
import io
import requests
import subprocess
from dotenv import load_dotenv

load_dotenv()

program_execute = "python foo.py"
cookie = os.environ.get("COOKIE_REMEMBERME")
user_id = os.environ.get("USER_ID")

session = requests.Session()
session.cookies.set("rememberMe", cookie, domain="codingame.com")

r = session.post(
    "https://www.codingame.com/services/Puzzle/generateSessionFromPuzzlePrettyId",
    json=[user_id, "number-shifting", False],
)
handle = r.json()["handle"]

# for each level of the game
while True:
    # run the solver on level.txt and save output to solution.txt
    # for Windows: set shell=False, if solution file isn't updated
    # subprocess.run(program_execute + " < level.txt > solution.txt", shell=True)
    with open("level_password.txt", "r") as f:
        level_pass = f.read().strip()
    with open("solution.txt", "r") as f:
        solution = f.read().strip()
    if solution == "":
        print("Empty solution, crashed? ...")
        break
    # solution = print(level_pass) + "\n" + "\n".join(f"print('{x}')" for x in solution)

    solution = level_pass + '\n' + solution
    with open("log.txt", "w") as f:
        f.write("\nsolution:\n")
        f.write(solution)

    # submit the solution to CodinGame
    r = session.post(
        "https://www.codingame.com/services/TestSession/play",
        json=[
            handle,
            {
                "code": solution,
                "programmingLanguageId": "PHP",
                "multipleLanguages": {"testIndex": 1},
            },
        ],
    )
    data = r.json()
    print(data)
    print("replay: https://www.codingame.com/replay/" + str(r.json()["gameId"]))
    next_level = ""
    if "gameInformation" in r.json()["frames"][-2]:
        next_level = r.json()["frames"][-2]["gameInformation"]
    if "Code for next level" not in next_level:
        print("The solution was wrong, watch the replay for details")
        break
    next_level = next_level[next_level.find(":") + 2:]
    level_password = next_level.split("\n")[0]
    number_level = int(1 + r.json()["metadata"]["Level"])
    with open("level_password.txt", "w") as f:
        f.write(level_password)
    with open("number_level.txt", "w") as f:
        f.write(str(number_level))
    # get the full level
    level_input = "\n".join(next_level.split("\n")[1:])
    if number_level > 258:  # fix for CG stderr limitations
        r = session.post(
            "https://www.codingame.com/services/TestSession/play",
            json=[
                handle,
                {
                    "code": 'echo "' + level_password + '";cat >&2',
                    "programmingLanguageId": "Bash",
                    "multipleLanguages": {"testIndex": 1},
                },
            ],
        )
        level_input = r.json()["frames"][2]["stderr"]
    with open("level.txt", "w") as f:
        f.write(level_input + "\n")

    # save input for next level
    with open("log.txt", "a") as f:
        f.write("\nreplay: https://www.codingame.com/replay/" + str(r.json()["gameId"]))
        f.write("\n\nLevel " + str(number_level) + ":\n")
        f.write(level_password + "\n")
        f.write(level_input)
