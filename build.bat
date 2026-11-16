@echo off
setlocal
set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"

where cargo >nul 2>nul
if errorlevel 1 (
    echo [ERROR] cargo not found. Install Rust from https://rustup.rs
    exit /b 1
)

set "CMD=%~1"
if "%CMD%"=="" set "CMD=build"

if /I "%CMD%"=="build"   (cargo build & exit /b %ERRORLEVEL%)
if /I "%CMD%"=="run"     (cargo run & exit /b %ERRORLEVEL%)
if /I "%CMD%"=="release" (cargo build --release & exit /b %ERRORLEVEL%)
if /I "%CMD%"=="test"    (cargo test & exit /b %ERRORLEVEL%)
if /I "%CMD%"=="check"   (cargo check & exit /b %ERRORLEVEL%)
if /I "%CMD%"=="clippy"  (cargo clippy --all-targets -- -D warnings & exit /b %ERRORLEVEL%)
if /I "%CMD%"=="fmt"     (cargo fmt --all & exit /b %ERRORLEVEL%)
if /I "%CMD%"=="clean"   (cargo clean & exit /b %ERRORLEVEL%)
if /I "%CMD%"=="dev"     (
    cargo fmt --all
    cargo clippy --all-targets -- -D warnings
    cargo test
    cargo build
    exit /b %ERRORLEVEL%
)

echo Usage: build [build^|run^|release^|test^|check^|clippy^|fmt^|dev^|clean]
exit /b 0