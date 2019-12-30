# SimPi wiringPi

This is the library part of the library wiringPi which emulates to be the real
library, but instead just communicates with the broker. The header file
`wiringPi.h` is the original one of the wiringPi library and has not been
modified.

## How to build and run (development only)

1. (Windows only) check the `cl.exe` path in clenv.bat to match your
    Visual C++ installation
2. Execute `make build-wiring-pi-sim` in the shell or cmd to build the library
    for your system
3. Locate the `make.bat` or `makefile` and change the `IN_SRC_FILES` or `SRC`
    env variable to the file you want to compile
4. Execute `make build` in the shell or cmd to build the executable of your
    file
5. Make sure you have started the SimPi Broker
6. Execute `make run` in the shell or cmd to run your program
