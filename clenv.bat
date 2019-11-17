@echo off
:: Setup env vars (do not edit these!!)
setlocal
set PWD=%cd%
set COMPILER_NAME=cl.exe
:: Edit the following to match your Visual Studio installation:
::    (the path provided must point to the folder containing vcvarsall.bat !!)
set ENV_SETUP_PATH="C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Auxiliary\Build\"
:: Edit here only the arguments for vcvarsall.bat (like x64 to x86 ...)
set ENV_SETUP_NAME=vcvarsall.bat x64
:: Let Visual Studio do its job and set up the environment
cd /d %ENV_SETUP_PATH%
call %ENV_SETUP_NAME%
:: Invoke compiler with any options passed to this batch file
cd /d %PWD%
%COMPILER_NAME% %*
endlocal