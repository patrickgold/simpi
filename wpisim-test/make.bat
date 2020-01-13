@echo off
setlocal
set IN_SRC_FILES="test\blinky.c"
if "%~1"=="build" (
    if exist "..\wpisim\target\debug\wpisim.dll.lib" (
        title Building executable
        echo Building executable...
        call ..\clenv.bat /EHsc %IN_SRC_FILES% /I ..\wpisim /link ..\wpisim\target\debug\wpisim.dll.lib /out:out\wpi_test.exe
        move *.obj out\
        xcopy "..\wpisim\target\debug\wpisim.dll" "out\" /Y >nul 2>&1
    ) else (
        echo Cannot build target
        echo  Reason: missing dll '..\wpisim\target\debug\wpisim.dll.lib'.
    )
) else if "%~1"=="run" (
    if exist ".\out\wpi_test.exe" (
        title WiringPi Test
        echo Executing '.\out\wpi_test.exe' ...
        set RUST_BACKTRACE=1
        set WPISIM_LOG=1
        .\out\wpi_test.exe
    ) else (
        echo There is no available build at this time!
    )
) else if "%~1"=="help" (
    echo help    - Show this help dialog.
    echo build   - Build the executable.
    echo run     - Run the executable.
) else (
    echo Unknown command '%1' (use 'help' for more info)
)
endlocal