#!/bin/bash

# Make simpi and out dir
mkdir ./out     2>/dev/null
mkdir ./simpi   2>/dev/null
rm -r ./out/*   2>/dev/null
rm -r ./simpi/* 2>/dev/null

# Script-scope global variables
BIN_cargo=""
CURR_OS_ID="" # windows, linux
CURR_OS_NAME=""
CURR_TARGET=""
PACK_PATH="$PWD"

HELPER_echo_step_count=1
HELPER_echo_step () {
    echo -e "\e[4mStep ${HELPER_echo_step_count}: $1\e[0m"
    let "HELPER_echo_step_count++"
}
HELPER_echo_error () {
    echo "$1" 1>&2
}

check_requirements () {
    echo -e "\e[1mPre Step: Install requirements check\e[0m"

    echo "> Rust"
    local file="$HOME/.cargo/bin/cargo"
    if test -f "$file"; then
        BIN_cargo="$file"
    else
        HELPER_echo_error "  No installation found! Aborting installation..."
        exit 1
    fi
    echo "  Selected: $file"

    apt install build-essential binutils-dev libunwind-dev gcc-mingw-w64-x86-64

    echo
}

STEP_prepare () {
    HELPER_echo_step "Preparing package build"

    rm -r ./simpi/*     2>/dev/null
}

STEP_build () {
    HELPER_echo_step "Building binaries"

    echo "> broker"
    cd ../broker
    ${BIN_cargo} build --release --target=$CURR_TARGET
    # if [ $ec -ne 0 ]; then
    #     HELPER_echo_error "  Build process failed!"
    #     exit 1
    # fi
    if [ "$CURR_OS_ID" == "windows" ]; then
        cp "./target/release/simpi_broker.exe" "$PACK_PATH/simpi"
    else
        cp "./target/release/simpi_broker" "$PACK_PATH/simpi"
    fi
    cd $PACK_PATH

    echo "> wpsism"
    cd ../wpisim
    ${BIN_cargo} build --release --target=$CURR_TARGET
    # if [ $ec -ne 0 ]; then
    #     HELPER_echo_error "  Build process failed!"
    #     exit 1
    # fi
    mkdir "$PACK_PATH/simpi/lib"
    if [ "$CURR_OS_ID" == "windows" ]; then
        cp "./target/release/wpisim.dll" "$PACK_PATH/simpi/lib"
        cp "./target/release/wpisim.dll.lib" "$PACK_PATH/simpi/lib"
    else
        cp "./target/release/libwpisim.d" "$PACK_PATH/simpi/lib"
        cp "./target/release/libwpisim.so" "$PACK_PATH/simpi/lib"
    fi
    cd $PACK_PATH
}

STEP_copy_files () {
    HELPER_echo_step "Copy files to simpi folder"

    if [ "$CURR_OS_ID" == "windows" ]; then
        cp -r "./windows_data/." "./simpi"
        cp "../uninstall.bat" "./simpi"
        cp "../wpisim/test/clenv.bat" "./simpi"
    else
        cp -r "./linux_data/." "./simpi"
        cp "../uninstall.sh" "./simpi"
    fi
    cp "../wpisim/wiringPi.h" "./simpi/lib"
    cp "../broker/media/app_icon.ico" "./simpi"
    cp "../LICENSE" "./simpi"
    cp "../README.md" "./simpi"
}

STEP_make_package () {
    HELPER_echo_step "Compress and zip all files"

    cd out
    if [ "$CURR_OS_ID" == "windows" ]; then
        zip -r "simpi-${CURR_OS_NAME}.zip" "../simpi"
        sha256sum "simpi-${CURR_OS_NAME}.zip" > "simpi-${CURR_OS_NAME}.sha256"
    else
        tar -czvf "simpi-${CURR_OS_NAME}.tar.gz" "../simpi"
        sha256sum "simpi-${CURR_OS_NAME}.tar.gz" > "simpi-${CURR_OS_NAME}.sha256"
    fi
    cd ..
}

build () {
    HELPER_echo_step_count=1

    echo -e "\e[35m\e[1mMake build package for target '$CURR_TARGET'\e[0m"

    rustup target add $CURR_TARGET

    echo

    STEP_prepare
    STEP_build
    STEP_copy_files
    STEP_make_package

    echo
}

build_all () {
    echo -e "--< Simpi Precompiled Installer Build Script >--\n"

    check_requirements
    
    # Linux 64bit
    CURR_OS_ID="linux"
    CURR_OS_NAME="linux-x86_64"
    CURR_TARGET="x86_64-unknown-linux-gnu"
    build

    # Windows 64bit
    CURR_OS_ID="windows"
    CURR_OS_NAME="windows-x86_64"
    CURR_TARGET="x86_64-pc-windows-gnu"
    build
    
    # read -n 1 -s -r -p "Done! Press any key to continue..."
    echo -e "Done!\n"
}

build_all
