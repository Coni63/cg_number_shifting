@echo off
if "%~1"=="" (
    echo Usage: %0 NomDuFichier
    pause
    exit /b 1
)

set AI_Name=%~n1
clang++ -std=c++17 -march=native -mpopcnt -mbmi2 -mfma -mavx2 -O3 -ffast-math -funroll-loops -finline "%AI_Name%.cpp" -o "%AI_Name%.exe"

if %ERRORLEVEL% NEQ 0 (
    echo Erreur de compilation!
    pause
    exit /b 1
)

echo Compilation reussie: %AI_Name%.exe