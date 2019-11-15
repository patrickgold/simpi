# simpi
A tool for simulating WiringPi projects on Windows and GNU/Linux.

*NOTE: this project is currently in alpha version and therefore bugs may
happen!*

## SimPi Broker

This is the core of the project and manages the GPIO Register of the simulated
Raspberry Pi as well as the front-end user interface for the browser.

### How to build and run

1. Change to the directory `simpi_broker`
2. Execute `make build` in the shell or cmd
3. Locate the `simpi_broker.exe` or `simpi_broker` in the `.\out` dir and run
    it.
4. Open [`http://127.0.0.1:32000`](http://127.0.0.1:32000) in your browser
    (Firefox, Chromium-based or Safari only!!)

## SimPi wiringPi

This is the library part of the library wiringPi which emulates to be the real
library, but instead just communicates with the broker. The header file
`wiringPi.h` is the original one of the library and not modified.

### How to build and run

1. Change to the directory `simpi_wiringpi`
2. Execute `make build-wiring-pi-sim` in the shell or cmd to build the library
    for your system
2. Execute `make build` in the shell or cmd to build the executable of your
    file
3. Locate the `wpi_test.exe` or `wpi_test` in the `.\out` dir and run it.

## Used libraries and fonts
- [wiringPi](https://github.com/WiringPi/WiringPi)
    by [WiringPi](https://github.com/WiringPi) (Header file `wiringPi.h` only)
- [httplib](https://github.com/yhirose/cpp-httplib)
    by [yhirose](https://github.com/yhirose)
- [Material Icons](https://github.com/google/material-design-icons)
    by [google](https://github.com/google)

