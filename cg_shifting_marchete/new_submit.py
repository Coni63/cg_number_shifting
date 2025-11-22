#!/usr/bin/env python3
import bz2
import time
import glob
import re
import traceback
import base64
import os
import requests
import subprocess
import argparse


cookie = os.environ.get("COOKIE_REMEMBERME")
user_id = os.environ.get("USER_ID")

session = requests.Session()
session.cookies.set("rememberMe", cookie, domain="codingame.com")

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


def _get_api_handle(user_id):
    """
    Get the handle of the current session from the Codingame API.
    """
    r = session.post(
        "https://www.codingame.com/services/Puzzle/generateSessionFromPuzzlePrettyId",
        json=[user_id, "number-shifting", False],
    )
    return r.json()["handle"]


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
    raise ValueError("Next level information not found in the response. : " + str(data))


def parse_input():
    parser = argparse.ArgumentParser(
        description="Execute a program with configurable parameters."
    )

    parser.add_argument(
        "program_name", help="Nom du programme à exécuter (ex: my_program)"
    )

    parser.add_argument(
        "--threads",
        type=int,
        default=2,
        help="Nombre de threads à utiliser (défaut: 2)",
    )

    parser.add_argument(
        "--reset-seconds",
        type=int,
        default=39,
        help="Nombre de secondes avant réinitialisation (défaut: 39)",
    )

    parser.add_argument(
        "--lFa", type=int, default=3200, help="Paramètre lFa (défaut: 3200)"
    )

    parser.add_argument(
        "--K-A", type=int, default=50000, help="Paramètre K_A (défaut: 50000)"
    )

    parser.add_argument("--K-B", type=int, default=0, help="Paramètre K_B (défaut: 0)")

    parser.add_argument(
        "--K-C", type=int, default=20000, help="Paramètre K_C (défaut: 20000)"
    )

    parser.add_argument(
        "--K-D", type=int, default=40000, help="Paramètre K_D (défaut: 40000)"
    )

    parser.add_argument(
        "--inc-time", type=int, default=100, help="Incrément de temps (défaut: 100)"
    )

    parser.add_argument(
        "--inc-lfa", type=int, default=40, help="Incrément de lFa (défaut: 40)"
    )

    parser.add_argument(
        "--allow-best", type=int, default=999, help="Numbers of threads that can go from LAHC to Recombinate (défaut: 999)"
    )

    parser.add_argument(
        "--recomb", type=int, default=0, help="Valeur de recomb (défaut: 0)"
    )

    return parser.parse_args()


args = parse_input()

handle = _get_api_handle(user_id)


program_execute = (
    f"{args.program_name}.exe {args.threads} {args.reset_seconds} {args.lFa} "
    f"{args.K_A} {args.K_B} {args.K_C} {args.K_D} {args.inc_time} "
    f"{args.inc_lfa} {args.allow_best} {args.recomb}"
)

salida = "solution.txt"
number_level = 0


def clean_files():
    # Liste des patterns de fichiers à supprimer
    patterns = ["SAFE_*.txt", "APROX_*.txt", "EXTERN_*.txt"]

    for pattern in patterns:
        for file in glob.glob(pattern):
            try:
                os.remove(file)
                print(f"Supprimé : {file}")
            except FileNotFoundError:
                pass  # ignore si le fichier n'existe pas
            except Exception as e:
                print(f"Erreur lors de la suppression de {file}: {e}")


def killProcess():
    global args

    subprocess.run(
        "taskkill /F /IM RN_Explo*.exe /T",
        shell=True,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    subprocess.run(
        f"taskkill /F /IM {args.program_name}.exe /T",
        shell=True,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )


def recompileCode():
    global args

    cpp_filename = args.program_name

    with open(cpp_filename + ".cpp", "r") as f:
        original_content = f.read().strip()

    try:
        if os.path.exists("level.txt"):
            with open("level.txt", "r") as f:
                rows = f.readlines()

            W, H = [int(x) for x in rows[0].strip().split(" ")]
            MAX_NUMBERS = 1  # 1 + number of non-zero entries
            for row in rows[1:]:
                for v in row.split():
                    MAX_NUMBERS += v != "0"

            new_content = re.sub(
                r"const\s+int\s+MAX_W\s*=.*;",
                "const int MAX_W = " + str(W) + ";",
                original_content,
            )
            new_content = re.sub(
                r"const\s+int\s+MAX_H\s*=.*;",
                "const int MAX_H = " + str(H) + ";",
                new_content,
            )
            new_content = re.sub(
                r"const\s+int\s+MAX_NUMBERS\s*=.*;",
                "const int MAX_NUMBERS = " + str(MAX_NUMBERS) + ";",
                new_content,
            )

            print(
                "Recompiling "
                + cpp_filename
                + ".cpp with W="
                + str(W)
                + " H="
                + str(H)
                + " MAX_NUMBERS="
                + str(MAX_NUMBERS)
                + ".... "
            )
        else:
            print("Recompiling " + cpp_filename + ".cpp without changes.... ")

    except Exception as error:
        print("Error compiling: %s" % error)

    with open(cpp_filename + ".cpp", "w") as f:
        f.write(new_content)

    subprocess.run(f"CLANG17.bat {cpp_filename}", shell=True)


while True:
    try:
        recompileCode()

        print("Program execute:" + program_execute)
        # run the solver on level.txt and save output to solution.txt
        subprocess.run(program_execute + " > " + salida, shell=True)

        with open("level_password.txt", "r") as f:
            level_pass = f.read().strip()

        try:
            with open(salida, "r") as f:
                solution = f.read().strip()
        except Exception:
            pass

        if solution == '':
            listaSoluciones = glob.glob("SOLUTION_*_" + level_pass + ".txt")
            for archivo_sol in listaSoluciones:
                print("Found solution file " + archivo_sol)
                keep = False
                with open(archivo_sol, "r") as f:
                    for line in f:
                        line = line.strip()
                        if line == level_pass:
                            keep = True
                            continue

                        if keep:
                            if re.match(r"^[0-9]{1,2} [0-9]{1,2} [UDLR] [+-]$", line):
                                solution += line + "\n"
                            else:
                                break
                break

        if solution == '':
            raise ValueError("No solution found in")
        
        print("Submitting solution...")

        with open("log.txt", "a") as f:
            f.write("\nsolution:\n")
            f.write(solution)

        level_password, next_level_number = submit_solution(
            handle, level_pass, solution
        )

        if next_level_number == 1000:
            print("Reached level 1000, exiting.")
            break

        print("Received next level password:", level_password, next_level_number)

        with open("level_password.txt", "w") as f:
            f.write(level_password)
        with open("number_level.txt", "w") as f:
            f.write(str(next_level_number))

        # get the full level
        level_input = get_level_data(handle, level_password)
        with open("level.txt", "w") as f:
            f.write(level_input)

        with open(f"level_{next_level_number}.txt", "w") as f:
            f.write(level_input)

        with open("log.txt", "a") as f:
            f.write("\n\nLevel " + str(number_level) + ":\n")
            f.write(level_password)
            f.write(level_input)

        clean_files()
        killProcess()
    except Exception as e:
        with open("log.txt", "a") as f:
            f.write("Exception {0}\n".format(str(e)) + " " + traceback.format_exc())
        time.sleep(10)
