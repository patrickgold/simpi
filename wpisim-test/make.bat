@echo off
setlocal
set IN_SRC_FILES="test\blinky.c"
if "%~1"=="build" (
    if exist "..\wpisim\target\debug\libwpisim.dll" (
        title Building executable
        echo Building executable...
        call ..\clenv.bat /EHsc %IN_SRC_FILES% /link ..\wpisim\target\debug\libwpisim.dll /out:out\wpi_test.exe
        move *.obj out\
    ) else (
        echo Cannot build target
        echo  Reason: missing obj file '.\out\wiringPiSim.obj'.
    )
) else if "%~1"=="run" (
    if exist ".\out\wpi_test.exe" (
        title WiringPi Test
        echo Executing '.\out\wpi_test.exe' ...
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