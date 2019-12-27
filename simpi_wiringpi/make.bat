@echo off
setlocal
set IN_SRC_FILES="test\blinky.c"
if "%~1"=="build" (
    if exist ".\out\wiringPiSim.obj" (
        title Building executable
        echo Building executable...
        call ..\clenv.bat /EHsc %IN_SRC_FILES% /link out\wiringPiSim.obj /out:out\wpi_test.exe
        move *.obj out\
    ) else (
        echo Cannot build target
        echo  Reason: missing obj file '.\out\wiringPiSim.obj'.
    )
) else if "%~1"=="build-wiring-pi-sim" (
    title Building library
    echo Building library...
    call ..\clenv.bat /c lib\wiringPi.c /Foout\wiringPiSim.obj
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
    echo build-wiring-pi-sim - Build the static .obj file for linking.
    echo run     - Run the executable.
) else (
    echo Unknown command '%1' (use 'help' for more info)
)
endlocal