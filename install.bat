@echo off
echo --^< SimPi Installer ^>--
net session >nul 2>&1
if %errorLevel% == 0 (
    rem user can continue
) else (
    echo [FAIL] This script must be run with admin rights.
    echo        Right click 'install.bat' and select 'Run as administrator' or use an elevated cmd session.
    exit /b
)
setlocal
:: change directory to dir of install.bat, bc "Run as admin" sets path to sys32
cd /d "%~dp0"
set _APP_ID=simpi
set _APP_NAME=SimPi Broker
set _APP_DATA_DIR=%APPDATA%\%_APP_ID%
set _PROGRAM_FILES_DIR=%programfiles%\%_APP_ID%
set _START_MENU_DIR=C:\ProgramData\Microsoft\Windows\Start Menu\Programs\%_APP_NAME%
:: #0 - Setup folders
echo Setting up folders...
mkdir "%_APP_DATA_DIR%" >nul 2>&1
mkdir "%_PROGRAM_FILES_DIR%" >nul 2>&1
mkdir "%_START_MENU_DIR%" >nul 2>&1
:: #1 - Setup SimPi Broker
echo Building SimPi Broker...
cd simpi_broker
call make.bat build >nul 2>&1
echo Copy files to program install location...
xcopy "www" "%_PROGRAM_FILES_DIR%\www" /e /i /h /Y >nul 2>&1
xcopy "out\simpi_broker.exe" "%_PROGRAM_FILES_DIR%" /Y >nul 2>&1
echo Create start menu entry...
:: Credit to 'rojo' for this solution of creating shortcuts on Windows:
:: https://stackoverflow.com/a/30029955
powershell "$s=(New-Object -COM WScript.Shell).CreateShortcut('%_START_MENU_DIR%\%_APP_NAME%.lnk');$s.TargetPath='%_PROGRAM_FILES_DIR%\simpi_broker.exe';$s.WorkingDirectory='%_PROGRAM_FILES_DIR%';$s.IconLocation='%_PROGRAM_FILES_DIR%\www\media\app_icon.ico';$s.Save()" >nul 2>&1
cd ..
:: #2 - Setup wiringPi lib
echo Building wiringPi lib...
cd simpi_wiringpi
call make.bat build-wiring-pi-sim >nul 2>&1
xcopy "out\wiringPiSim.obj" "%_PROGRAM_FILES_DIR%" /Y >nul 2>&1
cd ..
:: #3 - Copy clenv.bat, LICENSE, README.md and uninstall.bat
xcopy "clenv.bat" "%_PROGRAM_FILES_DIR%" /Y >nul 2>&1
xcopy "LICENSE" "%_PROGRAM_FILES_DIR%" /Y >nul 2>&1
xcopy "README.md" "%_PROGRAM_FILES_DIR%" /Y >nul 2>&1
xcopy "uninstall.bat" "%_PROGRAM_FILES_DIR%" /Y >nul 2>&1
powershell "$s=(New-Object -COM WScript.Shell).CreateShortcut('%_START_MENU_DIR%\Uninstall %_APP_NAME%.lnk');$s.TargetPath='%_PROGRAM_FILES_DIR%\uninstall.bat';$s.WorkingDirectory='%_PROGRAM_FILES_DIR%';$s.Save()" >nul 2>&1
:: #4 - Done
echo Done! Press any key to continue...
pause >nul
endlocal