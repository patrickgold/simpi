# SimPi Broker

This is the core of the project and manages the GPIO Register of the simulated
Raspberry Pi as well as the front-end user interface for the browser.

## How to build and run (development only)

1. (Windows only) check the `cl.exe` path in clenv.bat to match your
    Visual C++ installation
2. Execute `make build` in the shell or cmd
3. Execute `make run` in the shell or cmd to run the SimPi Broker
4. Open [`http://127.0.0.1:32000`](http://127.0.0.1:32000) in your browser
    (Firefox, Chromium-based or Safari only!!)
