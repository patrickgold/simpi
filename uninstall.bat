@echo off
echo --^< SimPi Uninstaller ^>--
net session >nul 2>&1
if %errorLevel% == 0 (
    rem user can continue
) else (
    echo [FAIL] This script must be run with admin rights.
    echo        Right click 'uninstall.bat' and select 'Run as administrator' or use an elevated cmd session.
    echo Press any key to continue...
    pause >nul
    exit /b
)
:: change directory to dir of uninstall.bat, bc "Run as admin" sets path to sys32
cd /d "%~dp0"
set _APP_ID=simpi
set _APP_NAME=SimPi Broker
set _APP_DATA_DIR=%APPDATA%\%_APP_ID%
set _PROGRAM_FILES_DIR=%programfiles%\%_APP_ID%
set _START_MENU_DIR=C:\ProgramData\Microsoft\Windows\Start Menu\Programs\%_APP_NAME%
:: #0 - Delete folders
echo Delete Program Files Simpi folder
rmdir "%_START_MENU_DIR%" /S
rmdir "%_PROGRAM_FILES_DIR%" /S
:: #1 - Done
echo Done! Press any key to continue...
pause >nul
EXIT