@echo off
REM Build script for MyPomodoro application on Windows

echo Building MyPomodoro for Windows...

REM Check if cargo-bundle is installed
cargo install --list | findstr "cargo-bundle" > nul
if %ERRORLEVEL% NEQ 0 (
    echo cargo-bundle is not installed. Installing now...
    cargo install cargo-bundle
)

REM Build the Windows executable with icon
echo Building Windows executable with icon...
cargo build --release
cargo bundle --release

echo Build process completed!
echo Check the 'target\release\bundle\' directory for the executable.
