@echo off
cargo build --release
echo Checking output...
mkdir built_mddr
move target\release\mddr.exe built_mddr\mddr.exe
echo Execution complete. Press any key to exit.
pause >nul