# RN_Explorer

Efficient Solver for Number Shifiting puzzle (https://www.codingame.com/ide/puzzle/number-shifting)

About 300x-1000x performance improvement from previous Solvers.

**Solving time in milliseconds**
Level | Dancing Links | LAHC | LAHC + RN_Explorer
------------ | ------------ | ------------- | -------------
225 | 6318000 | 490000 | 435
336 | Unsolved | 14400000 | 43000

Level 1000 can be solved in under 60 minutes. Due to the random nature of the Solver the solving time isn't fixed. To improve times it can be parallelized, and with more cores the solver tends to solve all levels in under 20 minutes. I had 1 Full RN_Explorer node + N Half RN_Explorers, sending good aproximations to that central RN_Explorer.

See https://github.com/marchete/RN_Explorer/blob/main/Efficient%20Solver%20for%20Number%20Shifiting%20puzzle.md for more info.

It was an evolution of an older code: https://github.com/marchete/Codingame/blob/master/Optimization/Number%20Shift/NumberShift_LAHC.cpp
This old code doesn't perform that well.

# Single CPU Solver for Number Shifting.

**_The code is INTENTIONALLY broken, it won't compile right out of the box. You need to fix the marked TODO's, if you know some C++ it can be done in less than 5 minutes_**

**_This code is not for running on Codingame directly, but for using it locally_**

Solutions are saved on files, that can be submitted to Codingame with PHP code. Submittable solutions are at the end of SOLUTION\_\* files, it starts with a password_level + list of moves. Tested on Ubuntu 18.04 LTS.

I have a distributed Solver package, with a lot of bells and whistles (centralized config files, ability to change solver code on remote nodes, load new level if a remote node solved it, etc..) I don't think it's really needed with the latest version.

## Prerrequisites:

- Python 3 (Tested on Python 3.6.9)
- Clang++-9
- Codingame account (user + password, not a github/google linked account. If you have one you can force a password change to have a password)
- cg_email.txt file with your user credentials. **NEVER SHARE THIS FILE WITH ANYONE**
- cg_password.txt file with your Codingames' password. **NEVER SHARE THIS FILE WITH ANYONE**

## Running the solver

python3 submit.py RN_ExploDiv_7 <THREADS> <LAHC_TIME_LIMIT> <LFA_SIZE> <K_A> <K_B> <K_C> <K_D> <INC_TIME> <INC_LFA> <RN_COUNT>

- \<THREADS>: On a physical machine up to 2x CPU core count. On Intel® Core™ i7-8700K I used 10.
- <LAHC_TIME_LIMIT>: Max time in seconds for LAHC Search. When it's timeout the thread resets and restart.
- <LFA_SIZE>:
- <K_A>: Remaining Numbers score
- <K_B>: Number of X Rows and Y Rows with numbers. I dont't use it on my final run.
- <K_C>: Remaining Points score
- <K_D>: Remaining Squared Points score.
- <INC_TIME>: Increase Search time on milliseconds, after restarts. Like 50ms to allow more time if the level is hard. The code limit the max search time to 70sec.
- <INC_LFA>: Increase LFA array on restarts. The code limit the Max LFA size to LFA_SIZE + 40\*INC_LFA.
- <RN_COUNT>: Defines how many workers will change from LAHC mode to RN_Explore mode. A value <= \<THREADS>. I always use THREADS-1 or THREADS-2, leaving a worker always in LAHC mode.

**Note:** There are a lot more of options and parameters inside the .cpp code. You have more parameters to tweak (probably more important than K_A..K_D) on lines 1491-1498 and 138-169.
Constant value _MIN_DEGREE_TOINSERT_ defines at what amount of Remaining Numbers a Worker changes from LAHC to RN_Explorer. It can be anything from 3 to MAX_NUMBERS. I think a value
between 6 and 9 is good for level 1000. A high value will remove LAHC and always use RN_Explorer. It will eventually solve it, but RN_Explorer has no reset timers, so you can end with
a local minimum that will take some time to go out.

## Features of submit.py

It's an evolved version of the recommended submit.py: https://github.com/eulerscheZahl/NumberShifting/tree/master/solver

- It's resilient to common errors: Solver crashes, wrong solutions, wrong replays, login errors
- It recompiles the code to the correct W,H and MAX_NUMBERS. This ensures maximum performance.
- On solver crashes (due to bugs on the code) it tries to recover the solution from SOLUTION\_\*.txt file.
- It allows a _parameters.txt_ file. Useful when deploying multiples nodes, for changing parameters on the fly.
- It allows a _runningprocess.txt_ file. Useful when deploying multiples nodes, for changing the running code on the fly.

## Performance

With an Intel® Core™ i7-8700K it solved levels 200 to 500 in 2hrs 41min (using 8/10 THREADS depending on the time), while I was doing other things in that PC. This time is total time, c++ execution time + python3 time (downloading levels and replays, etc).

## Compiling

CPP code was tested on CLANG and Visual Studio. It needs these compiler options `/GS /GL /W3 /Gy- /Zc:wchar_t /Zi /Gm- /Ox /Ob2 /sdl- /Zc:inline /fp:precise /D "NDEBUG" /D "_CONSOLE" /D "_CRT_SECURE_NO_WARNINGS" /D "_UNICODE" /D "UNICODE" /errorReport:prompt /WX- /Zc:forScope /arch:AVX2 /Gd /Oy /Oi /MD /std:c++17 /FC /Fa"x64\Release\" /EHsc /nologo /Fo"x64\Release\" /Ot /diagnostics:classic `
