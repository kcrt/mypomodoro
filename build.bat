@echo off
REM Build script for MyPomodoro application on Windows

echo Building MyPomodoro for Windows...

REM Build the Windows executable
echo Building Windows executable...
cargo build --release

echo Build process completed!
echo Check the 'target\release\' directory for the executable.
