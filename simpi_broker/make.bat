@echo off
setlocal
if "%1"=="build" (
    title Building executable
    echo Building executable...
    call ..\clenv.bat /EHsc src\main.cpp lib\Broker.cpp /link /out:out\simpi_broker.exe
    move *.obj out\
) else if "%1"=="run" (
    if exist ".\out\simpi_broker.exe" (
        title SimPi Broker
        echo Executing '.\out\simpi_broker.exe' ...
        .\out\simpi_broker.exe
    ) else (
        echo There is no available build at this time!
    )
) else if "%1"=="help" (
    echo help    - Show this help dialog.
    echo build   - Build the executable.
    echo run     - Run the executable.
) else (
    echo Unknown command '%1' (use 'help' for more info)
)
endlocal