@echo off
REM Change directory to the project folder
cd /D rape-ue

REM Change the title
title rape-ue build [release]

REM Clean the project
echo Cleaning the project...
cargo clean
if %errorlevel% neq 0 (
    echo Failed to clean the project.
    pause
    exit /b %errorlevel%
)

REM Clear the console screen
cls

REM Build the project in release mode
echo Building the project in release mode...
cargo build --release
if %errorlevel% neq 0 (
    echo Build failed.
    pause
    exit /b %errorlevel%
)

REM Success message
echo Build completed successfully and located at "target\release".
pause