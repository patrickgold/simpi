@echo off
echo --^< SimPi Installer (Precompiled) ^>--
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
set _PROGRAM_FILES_DIR=%programfiles%\%_APP_ID%
set _START_MENU_DIR=C:\ProgramData\Microsoft\Windows\Start Menu\Programs\%_APP_NAME%
:: #0 - Setup folders
echo Setting up folders...
mkdir "%_PROGRAM_FILES_DIR%" >nul 2>&1
mkdir "%_START_MENU_DIR%" >nul 2>&1
:: #1 - Copy files
echo Copy files to install folder...
xcopy "simpi_broker.exe" "%_PROGRAM_FILES_DIR%" /Y >nul 2>&1
xcopy "app_icon.ico" "%_PROGRAM_FILES_DIR%" /Y >nul 2>&1
xcopy "lib\wpisim.dll" "%_PROGRAM_FILES_DIR%" /Y >nul 2>&1
xcopy "lib\wpisim.dll.lib" "%_PROGRAM_FILES_DIR%" /Y >nul 2>&1
xcopy "lib\wiringPi.h" "%_PROGRAM_FILES_DIR%" /Y >nul 2>&1
xcopy "LICENSE" "%_PROGRAM_FILES_DIR%" /Y >nul 2>&1
xcopy "README.md" "%_PROGRAM_FILES_DIR%" /Y >nul 2>&1
xcopy "clenv.bat" "%_PROGRAM_FILES_DIR%" /Y >nul 2>&1
xcopy "uninstall.bat" "%_PROGRAM_FILES_DIR%" /Y >nul 2>&1
:: #2 - Create start menu entry
echo Create start menu entry...
:: Credit to 'rojo' for this solution of creating shortcuts on Windows:
:: https://stackoverflow.com/a/30029955
powershell "$s=(New-Object -COM WScript.Shell).CreateShortcut('%_START_MENU_DIR%\%_APP_NAME%.lnk');$s.TargetPath='%_PROGRAM_FILES_DIR%\simpi_broker.exe';$s.WorkingDirectory='%_PROGRAM_FILES_DIR%';$s.IconLocation='%_PROGRAM_FILES_DIR%\app_icon.ico';$s.Save()" >nul 2>&1
powershell "$s=(New-Object -COM WScript.Shell).CreateShortcut('%_START_MENU_DIR%\Uninstall %_APP_NAME%.lnk');$s.TargetPath='%_PROGRAM_FILES_DIR%\uninstall.bat';$s.WorkingDirectory='%_PROGRAM_FILES_DIR%';$s.Save()" >nul 2>&1
:: #3 - Done
echo Done! Press any key to continue...
pause >nul
endlocal