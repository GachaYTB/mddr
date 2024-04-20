@echo off
cargo build --release
echo Checking output...
mkdir TEMP_BUILT_MDDR
move target\release\mddr.exe TEMP_BUILT_MDDR\mddr.exe
rmdir /s /q target
echo Successfully built mddr.exe. Zipping files into zip file...
cd TEMP_BUILT_MDDR
for /f "tokens=3,2,4 delims=/- " %%x in ("%date%") do set d=%%y%%x%%z
set data=%d%
"C:\Program Files\7-Zip\7z.exe" a -tzip "./MDDR.zip" "./mddr.exe"
cd ..
move TEMP_BUILT_MDDR\MDDR.zip ./
rmdir /s /q TEMP_BUILT_MDDR
echo Successfully built mddr. Press enter to exit.
pause >nul