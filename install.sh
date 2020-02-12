#!/bin/bash

# Aquire root privileges
[ "$UID" -eq 0 ] || exec sudo bash "$0" "$@"

# Script-scope global variables
BIN_cargo=""
app_id="simpi"
app_name="SimPi Broker"
prog_files_dir="/opt/$app_id"
start_menu_dir="/usr/share/applications"

HELPER_echo_step_count=1
HELPER_echo_step () {
    echo -e "\e[1mStep ${HELPER_echo_step_count}: $1\e[0m"
    let "HELPER_echo_step_count++"
}
HELPER_echo_error () {
    echo "$1" 1>&2
}

STEP_check_requirements () {
    HELPER_echo_step "Install requirements check"

    echo "> Rust"
    local file="$HOME/.cargo/bin/cargo"
    if test -f "$file"; then
        BIN_cargo="$file"
    else
        HELPER_echo_error "  No installation found! Aborting installation..."
        exit 1
    fi
    echo "  Selected: $file"
}

STEP_build () {
    HELPER_echo_step "Building binaries"

    echo "> broker"
    cd broker
    ${BIN_cargo} build --release
    cd ..

    echo "> wpsism"
    cd wpisim
    ${BIN_cargo} build --release
    cd ..
}

STEP_copy_files () {
    HELPER_echo_step "Copy files to install folder"

    mkdir -p "$prog_files_dir"

    cd broker
    cp "./target/release/simpi_broker" "$prog_files_dir"
    cp "./media/app_icon.ico" "$prog_files_dir"
    cd ..

    cd wpisim
    cp "./target/release/libwpisim.d" "/usr/local/lib"
    cp "./target/release/libwpisim.so" "/usr/local/lib"
    cp "./wiringPi.h" "/usr/local/include"
    cd ..
    
    cp "./LICENSE" "$prog_files_dir"
    cp "./README.md" "$prog_files_dir"
    cp "./uninstall.sh" "$prog_files_dir"
}

STEP_add_start_menu_entry () {
    HELPER_echo_step "Create start menu entry"

    touch "$start_menu_dir/$app_id.desktop"
    echo "[Desktop Entry]
    Name=$app_name
    Comment=A tool for simulating WiringPi projects
    Exec=$prog_files_dir/simpi_broker
    Path=$prog_files_dir
    Terminal=true
    Type=Application
    Icon=$prog_files_dir/app_icon.ico
    Categories=Development;" >> "$start_menu_dir/$app_id.desktop"
}

simpi_install () {
    echo -e "--< Simpi Installer >--\n"

    STEP_check_requirements
    STEP_build
    STEP_copy_files
    STEP_add_start_menu_entry
    
    read -n 1 -s -r -p "Done! Press any key to continue..."
    echo -e "\n"
}

simpi_install
