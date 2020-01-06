# SimPi
A tool for simulating wiringPi projects written for the Raspberry Pi on
Windows and GNU/Linux.

*NOTE: this project is currently in beta version. Features may not work as
intended or may behave unexpectedly!*

*NOTE: altough this project aims to support both Windows and GNU/Linux,
currently the focus lies on GNU/Linux due to its easier and better support
of used features in this project.*

## Features
* Simulates the Raspberry Pi GPIO Register (currently of the 3B+ model)
* Allows you to build and run programs written with the wringPi library
* Supports most of the original library functions
* Works on GNU/Linux and (with complications) Windows

## Prerequisites
In order to build and run this project, the following tools should be installed
on your machine:
* GNU/Linux:
  * gcc/g++ compiler suite
  * git
  * make
  * Library pthread
* Windows:
  * Visual Studio C++ Compiler
  * Git (you can also dowload the zip of this repo and unpack it, if you do
    not want to install git on Windows)

## Installation

### From Source (GNU/Linux)
```bash
$ git clone https://github.com/patrickgold/simpi.git
$ cd simpi/
$ chmod +x install.sh
$ sudo ./install.sh
```

### From Source (Windows)
Open the Command Prompt **as Admin**, then
```cmd
> cd %USERPROFILE%\Downloads
> git clone https://github.com/patrickgold/simpi.git
> cd simpi
> notepad clenv.bat
```
Change the `ENV_SETUP_PATH` variable to match your Visual Studio C++
installation (directory of `cl.exe`), then hit save and close notepad.

This [documentation](https://docs.microsoft.com/en-us/cpp/build/building-on-the-command-line?view=vs-2019#developer_command_file_locations) by Microsoft may help you to locate `cl.exe`.
```cmd
> install.bat
```

## Usage
Search for "SimPi" in your start menu, there you should find a shortcut to the
SimPi Broker executable.

If there is no shortcut, you can locate the executable in the following folder:
* GNU/Linux: `/opt/simpi`
* Windows: `$programfiles\simpi`

The broker then automatically opens `127.0.0.1:32000` in your default browser.

## Compiling Programs

### GNU/Linux
Use the library flag `-lwiringPiSim` of gcc to compile your wiringPi program.
When running the compiled program, it willl try to communicate with the SimPi
Broker. If this fails, an error message will be outputted.

### Windows
(todo: create windows compilation script)

## Used libraries and fonts
- [wiringPi](https://github.com/WiringPi/WiringPi)
    by [WiringPi](https://github.com/WiringPi) (Header file `wiringPi.h` only)
- [httplib](https://github.com/yhirose/cpp-httplib)
    by [yhirose](https://github.com/yhirose)
- [Material Icons](https://github.com/google/material-design-icons)
    by [google](https://github.com/google)

## License
This project is licensed under the GNU General Public License v3.0 - see the
[LICENSE](LICENSE) file for details.

